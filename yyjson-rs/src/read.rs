use std::ffi::CStr;

use num_derive::FromPrimitive;

use yyjson_sys as ffi;

#[derive(Debug)]
pub enum ReadError {
    OnCreateErr(&'static str),
    OnRead {
        code: ReadCode,
        msg: &'static str,
        pos: usize,
    },
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ReadError::*;
        match self {
            OnCreateErr(s) => write!(f, "On create error: {}", s),
            OnRead { code, msg, pos } => {
                write!(f, "{:?} at pos {pos}: {}", code, msg)
            }
        }
    }
}

impl std::error::Error for ReadError {}

#[derive(Debug, FromPrimitive, PartialEq)]
#[repr(u32)]
pub enum ReadCode {
    Success = 0,
    ErrorInvalidParameter = 1,
    ErrorMemoryAllocation = 2,
    ErrorEmptyContent = 3,
    ErrorUnexpectedContent = 4,
    ErrorUnexpectedEnd = 5,
    ErrorUnexpectedCharacter = 6,
    ErrorJsonStructure = 7,
    ErrorInvalidComment = 8,
    ErrorInvalidNumber = 9,
    ErrorInvalidString = 10,
    ErrorLiteral = 11,
    ErrorFileOpen = 12,
    ErrorFileRead = 13,
}

impl ReadError {
    #[inline(always)]
    pub fn is_mem_allocation_err(&self) -> bool {
        match self {
            Self::OnRead { code, .. } => code == &ReadCode::ErrorMemoryAllocation,
            _ => false,
        }
    }
}

impl From<&ffi::yyjson_read_err> for ReadError {
    fn from(v: &ffi::yyjson_read_err) -> Self {
        use ReadError::*;
        let code: ReadCode = match num_traits::cast::FromPrimitive::from_u32(v.code) {
            Some(v) => v,
            None => return OnCreateErr("yyjson_read_code should be in range 0..=13"),
        };

        if code == ReadCode::Success {
            return OnCreateErr("yyjson_read_code is SUCCESS (non-error)");
        }

        if v.msg.is_null() {
            return OnCreateErr("error msg string is null yet error occured");
        }

        // SAFETY: if an error occured (i.e. read_code is not SUCCESS) then
        // msg is guaranteed to be non-null and point to a statically allocated
        // c-string if v is non error
        let msg = {
            let as_c_str = unsafe { CStr::from_ptr(v.msg) };
            match as_c_str.to_str() {
                Ok(s) => s,
                Err(_) => return OnCreateErr("error msg string is invalid utf-8"),
            }
        };

        OnRead {
            code,
            msg,
            pos: v.pos,
        }
    }
}

#[derive(Debug, Default)]
pub struct ReadOptions {
    // stop reading when reaching the end of a JSON document instead of
    // issuing an error if there's additional content after it. This is for
    // reading small pieces of JSON within larger data, such as NDJSON
    pub stop_when_done: bool,
    pub allow_trailing_commas: bool,
    pub allow_comments: bool,
    pub allow_inf_and_nan: bool,
    pub bignums_as_raw_strings: bool,
}

impl ReadOptions {
    pub fn to_read_flag(&self) -> u32 {
        use ffi::*;
        let mut flag: u32 = YYJSON_READ_NOFLAG;
        if self.stop_when_done {
            flag |= YYJSON_READ_STOP_WHEN_DONE;
        }
        if self.allow_trailing_commas {
            flag |= YYJSON_READ_ALLOW_TRAILING_COMMAS;
        }
        if self.allow_comments {
            flag |= YYJSON_READ_ALLOW_COMMENTS;
        }
        if self.allow_inf_and_nan {
            flag |= YYJSON_READ_ALLOW_INF_AND_NAN;
        }
        if self.bignums_as_raw_strings {
            flag |= YYJSON_READ_BIGNUM_AS_RAW;
        }
        return flag;
    }
}
