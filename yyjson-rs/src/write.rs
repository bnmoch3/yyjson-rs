use std::{ffi::CStr, fmt::Display, mem::MaybeUninit};

use num_derive::FromPrimitive;
use std::ffi::c_void;

use crate::Allocator;
use yyjson_sys as ffi;

#[derive(Debug)]
pub enum WriteError {
    OnCreateErr(&'static str),
    OnWrite { code: WriteCode, msg: &'static str },
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WriteError::*;
        match self {
            OnCreateErr(s) => write!(f, "On create error: {}", s),
            OnWrite { code, msg } => {
                write!(f, "{:?}: {}", code, msg)
            }
        }
    }
}

impl std::error::Error for WriteError {}

#[derive(Debug, FromPrimitive, PartialEq)]
#[repr(u32)]
pub enum WriteCode {
    Success = 0,
    ErrorInvalidParameter = 1,
    ErrorMemoryAllocation = 2,
    ErrorInvalidValueType = 3,
    ErrorNaNOrInf = 4,
    ErrorFileOpen = 5,
    ErrorFileWrite = 6,
    ErrorInvalidString = 7,
}

impl WriteError {
    #[inline(always)]
    pub fn is_mem_allocation_err(&self) -> bool {
        match self {
            Self::OnWrite { code, .. } => code == &WriteCode::ErrorMemoryAllocation,
            _ => false,
        }
    }
}

impl From<&ffi::yyjson_write_err> for WriteError {
    fn from(v: &ffi::yyjson_write_err) -> Self {
        use WriteError::*;
        let code: WriteCode = match num_traits::cast::FromPrimitive::from_u32(v.code) {
            Some(v) => v,
            None => return OnCreateErr("yyjson_write_code should be in range 0..=7"),
        };

        if code == WriteCode::Success {
            return OnCreateErr("yyjson_write_code is SUCCESS (non-error)");
        }

        if v.msg.is_null() {
            return OnCreateErr("error msg string is null yet error occured");
        }

        // SAFETY: if an error occured (i.e. write_code is not SUCCESS) then
        // msg is guaranteed to be non-null and point to a statically allocated
        // c-string if v is non error
        let msg = {
            let as_c_str = unsafe { CStr::from_ptr(v.msg) };
            match as_c_str.to_str() {
                Ok(s) => s,
                Err(_) => return OnCreateErr("error msg string is invalid utf-8"),
            }
        };

        OnWrite { code, msg }
    }
}

#[derive(Debug, Default)]
// By default (all options set to false), JSON is written in a minified format,
// inf, NaN, invalid utf-8 values are reported as errors and yyjson does not
// escape unicode or slashes
pub struct WriteOptions {
    // with 4 spaces
    pub pretty: bool,

    // overrides write_pretty
    pub pretty_with_two_spaces: bool,

    // makes the output ascii-only
    pub escape_unicode: bool,

    // escapes forward slash character '/' as '\/'
    pub escape_slashes: bool,

    pub allow_inf_and_nan: bool,

    // write inf and Nan as null instead of reporting error,
    // overrides allow_inf_and_nan
    pub inf_and_nan_as_null: bool,

    // e.g. for ndjson
    pub add_newline_at_end: bool,
}

impl WriteOptions {
    pub fn to_write_flag(&self) -> u32 {
        let mut flag: u32 = ffi::YYJSON_WRITE_NOFLAG;
        if self.pretty {
            flag |= ffi::YYJSON_WRITE_PRETTY;
        }
        if self.pretty_with_two_spaces {
            flag |= ffi::YYJSON_WRITE_PRETTY_TWO_SPACES;
        }
        if self.escape_unicode {
            flag |= ffi::YYJSON_WRITE_ESCAPE_UNICODE;
        }
        if self.escape_slashes {
            flag |= ffi::YYJSON_WRITE_ESCAPE_SLASHES;
        }
        if self.allow_inf_and_nan {
            flag |= ffi::YYJSON_WRITE_ALLOW_INF_AND_NAN;
        }
        if self.inf_and_nan_as_null {
            flag |= ffi::YYJSON_WRITE_INF_AND_NAN_AS_NULL;
        }
        if self.add_newline_at_end {
            flag |= ffi::YYJSON_WRITE_NEWLINE_AT_END;
        }
        flag
    }
}

pub struct Writer<'allocator> {
    alloc: Allocator<'allocator>,
    write_flag: u32,
}

impl<'allocator> Writer<'allocator> {
    pub fn new(alloc: Allocator<'allocator>, options: Option<&WriteOptions>) -> Self {
        let write_flag = if let Some(v) = options {
            v.to_write_flag()
        } else {
            ffi::YYJSON_WRITE_NOFLAG
        };
        Self { alloc, write_flag }
    }

    #[inline(always)]
    pub fn write(
        &self,
        do_write: impl Fn(u32, *mut ffi::yyjson_alc, &mut usize, *mut ffi::yyjson_write_err) -> *mut u8,
    ) -> Result<WriteOutput<'allocator>, WriteError> {
        let mut len: usize = 0; // receives output length, will not include NUL
        let mut write_err = MaybeUninit::<ffi::yyjson_write_err>::uninit();
        // SAFETY: it is okay if alc is null. If it is non-null, by construction
        // it points to a valid ffi::yyjson_alc struct
        let alc = unsafe { self.alloc.as_mut_ptr() };
        let ptr = do_write(self.write_flag, alc, &mut len, write_err.as_mut_ptr());
        if ptr.is_null() {
            // This means either something is wrong with the document OR we
            // passed a null document
            // SAFETY: if error occurs, yyjson_write_opts will set the fields in
            // the write_err appropriately
            let write_err = unsafe { write_err.assume_init() };
            Err((&write_err).into())
        } else {
            Ok(WriteOutput {
                ptr,
                len,
                alc,
                _lifetime: std::marker::PhantomData,
            })
        }
    }
}

#[derive(Debug)]
pub struct WriteOutput<'allocator> {
    ptr: *mut u8,
    len: usize,
    alc: *mut ffi::yyjson_alc,
    _lifetime: std::marker::PhantomData<&'allocator ()>,
}

impl WriteOutput<'_> {
    #[inline(always)]
    fn as_str(&self) -> &str {
        // SAFETY: at the point DocWriteOutput is constructed, the ptr value is
        // assigned only if it is non-null and the len value is set by yyjson if
        // the write is successful
        let buf: &[u8] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };

        // SAFETY: when parsing the input, yyjson checks if it is valid utf-8 (
        // unless compiled with the flag disabling utf-8 checks in which case
        // we have to do it prior to passing the input to yyjson). Therefore
        // when writing out, yyjson ouptuts valid utf-8 from the internal doc
        // structure. Additionally, DocWriteOutput is only constructed if the
        // write was successful.
        let s: &str = unsafe { std::str::from_utf8_unchecked(buf) };
        s
    }
}

impl Display for WriteOutput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.as_str();
        write!(f, "{}", s)
    }
}

impl Drop for WriteOutput<'_> {
    fn drop(&mut self) {
        if self.alc.is_null() {
            // SAFETY: if the output was not allocated into a passed allocator
            // then yyjson allocates it using malloc hence it should be freed
            // via a call to libc free
            unsafe { libc::free(self.ptr.cast::<c_void>()) }
        } else {
            // SAFETY: if alc is non-NULL, it holds a validly constructed
            // Allocator
            let alc: &ffi::yyjson_alc = unsafe { &*self.alc };
            let free_fn = alc
                .free
                .expect("if alc is non-null, then the free field should be set to Some");
            // SAFETY: free_fn is not None since we've unwrapped it above. In
            // all cases where an Allocator is constructed, free field is
            // initialized to the appropriate memory allocation function
            unsafe { free_fn(alc.ctx, self.ptr.cast::<c_void>()) };
        }
    }
}
