use std::mem::MaybeUninit;
use std::ptr;

use crate::read::{ReadError, ReadOptions};
use crate::val::Val;
use crate::write::{WriteError, WriteOutput, Writer};

use yyjson_sys as ffi;

#[derive(Debug)]
#[repr(transparent)]
pub struct Doc {
    p: *mut ffi::yyjson_doc,
}

impl Drop for Doc {
    fn drop(&mut self) {
        // SAFETY: self.doc is non-null since only way to build a valid doc
        // is through `read_from` which ensures that doc is valid and non-null
        unsafe { ffi::yyjson_doc_free(self.p) };
    }
}

impl std::fmt::Display for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root().fmt(f)
    }
}

impl Doc {
    pub fn read_from(buf: &[u8], options: Option<&ReadOptions>) -> Result<Doc, ReadError> {
        let flag = if let Some(v) = options {
            v.to_read_flag()
        } else {
            ffi::YYJSON_READ_NOFLAG
        };
        let mut read_err = MaybeUninit::<ffi::yyjson_read_err>::uninit();
        // SAFETY: buf is not modified as long as we do not pass the
        // YYJSON_READ_INSITU flag hence it's okay to cast it to `char *` i.e.
        // as `*mut i8`. If no error occurs, `doc` will be non-NULL since buf
        // is non-NULL
        let doc: *mut ffi::yyjson_doc = unsafe {
            ffi::yyjson_read_opts(
                buf.as_ptr() as *mut i8,
                buf.len(),
                flag,
                ptr::null(), // use default lib_c allocator, TODO use rust's
                read_err.as_mut_ptr(),
            )
        };
        if doc.is_null() {
            // SAFETY: if error occurs, yyjson_read_opts will set the fields in
            // the read_err appropriately
            let read_err = unsafe { read_err.assume_init() };
            Err((&read_err).into())
        } else {
            Ok(Self { p: doc })
        }
    }

    // number of bytes are read when parsing JSON
    pub fn read_size(&self) -> usize {
        // SAFETY: by construct, doc is non-null and points to a valid doc
        // object
        unsafe { ffi::yyjson_doc_get_read_size(self.p) }
    }

    // number of values in this document
    pub fn val_count(&self) -> usize {
        // SAFETY: by construction, doc is non-null and points to a valid doc
        // object
        unsafe { ffi::yyjson_doc_get_val_count(self.p) }
    }

    // number of values in this document
    #[inline(always)]
    pub fn root<'doc>(&'doc self) -> Val<'doc> {
        Val {
            p: unsafe { ffi::yyjson_doc_get_root(self.p) },
            _doc_lifetime: std::marker::PhantomData,
        }
    }

    pub fn write<'a>(&self, writer: &mut Writer<'a>) -> Result<WriteOutput<'a>, WriteError> {
        writer.write(
            |write_flag: u32,
             alc: *mut ffi::yyjson_alc,
             len: &mut usize,
             write_err: *mut ffi::yyjson_write_err| {
                // SAFETY: it is okay if doc is null, yyjson writes out null string
                // otherwise doc should be a valid yyjson_doc
                unsafe {
                    ffi::yyjson_write_opts(self.p, write_flag, alc, len, write_err).cast::<u8>()
                }
            },
        )
    }
}
