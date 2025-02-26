use std::alloc::{self, Layout};
use std::ffi::c_void;
use std::ptr;

use yyjson_sys as ffi;

#[repr(transparent)]
pub struct Allocator<'parent> {
    alc: *mut ffi::yyjson_alc,
    // just in case the Allocator is an embedded field within a parent/holder it
    // should not outlive the parent
    _lifetime: std::marker::PhantomData<&'parent ()>,
}

impl Allocator<'_> {
    /// # Safety
    /// caller should make sure that alc is valid. it is okay for alc
    /// to be null - in such a case yyjson will default to the system allocator
    #[inline(always)]
    pub unsafe fn as_mut_ptr(&self) -> *mut ffi::yyjson_alc {
        self.alc
    }
}

impl Default for Allocator<'static> {
    fn default() -> Self {
        Self {
            alc: ptr::null_mut(),
            _lifetime: std::marker::PhantomData,
        }
    }
}

// =============================================================================
//                  SINGLE BUF ALLOCATOR
// =============================================================================

pub struct SingleBufAllocator {
    alc: *mut ffi::yyjson_alc,
    buf: *mut u8,
    layout: Layout,
}

impl Drop for SingleBufAllocator {
    fn drop(&mut self) {
        // SAFETY: SingelBufAllocator can only be created if buf is non-null and
        // points to valid memory and layout is valid
        unsafe { alloc::dealloc(self.buf, self.layout) };

        // SAFETY:: alc is initialized from a pointer from the boxed value.
        let _ = unsafe { Box::from_raw(self.alc) };
    }
}

impl<'a> SingleBufAllocator {
    #[inline(always)]
    pub fn allocator(&'a self) -> Allocator<'a> {
        Allocator {
            alc: self.alc,
            _lifetime: std::marker::PhantomData::<&'a ()>,
        }
    }

    pub fn create(
        expected_json_size: usize,
        options: Option<&crate::ReadOptions>,
    ) -> Result<Self, &'static str> {
        let flag = if let Some(v) = options {
            v.to_read_flag()
        } else {
            ffi::YYJSON_READ_NOFLAG
        };
        // SAFETY: on error (e.g.overflow), this function returns 0. When
        // creating layout below, if buf_size is 0, it will return an error to
        // the caller
        let buf_size = unsafe { ffi::yyjson_read_max_memory_usage(expected_json_size, flag) };
        let layout = match Layout::from_size_align(buf_size, 64) {
            Ok(l) => l,
            Err(_) => {
                return Err("align should be non-zero and a power of 2, size should not overflow")
            }
        };
        let buf: *mut u8 = unsafe { alloc::alloc(layout) };
        if buf.is_null() {
            return Err("allocator returned null");
        }
        let alc: *mut ffi::yyjson_alc = {
            let b = Box::new(ffi::yyjson_alc {
                malloc: None,
                realloc: None,
                free: None,
                ctx: ptr::null_mut(),
            });
            Box::into_raw(b)
        };

        // this has to be created here so that if `yyjson_alc_pool_init` fails
        // then both alc and buf will be dropped
        let sb = SingleBufAllocator { alc, buf, layout };

        // init pool
        // SAFETY: alc is non-null since it's retrieved from a Boxed value and
        // initialized with valid values, buf is non-null and of given
        // `buf_size`
        let ok: bool = unsafe { ffi::yyjson_alc_pool_init(alc, buf.cast::<c_void>(), buf_size) };
        if !ok {
            return Err("Failed to initialized pool allocator, buf or size invalid");
        }

        Ok(sb)
    }
}

// =============================================================================
//                  DYN ALLOCATOR
// =============================================================================

#[repr(transparent)]
pub struct DynamicAllocator {
    alc: *mut ffi::yyjson_alc,
}

impl Default for DynamicAllocator {
    fn default() -> Self {
        let alc: *mut ffi::yyjson_alc = unsafe { ffi::yyjson_alc_dyn_new() };
        Self { alc }
    }
}

impl<'a> DynamicAllocator {
    #[inline(always)]
    pub fn allocator(&'a self) -> Allocator<'a> {
        Allocator {
            alc: self.alc,
            _lifetime: std::marker::PhantomData::<&'a ()>,
        }
    }
}

impl Drop for DynamicAllocator {
    fn drop(&mut self) {
        // SAFETY: by construction, either DynamicAllocator.alc is null in which
        // case `yyjson_alc_dyn_free` will not error out or is non-null in which
        // case it points to a valid yyjson_alc that will be freed appropriately
        unsafe { ffi::yyjson_alc_dyn_free(self.alc) }
    }
}
