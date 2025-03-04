use std::alloc::{self, Layout};
use std::ffi::c_void;
use std::ptr;

use yyjson_sys as ffi;

#[repr(transparent)]
pub struct YyjsonAllocator<'a> {
    pub(crate) p: *mut ffi::yyjson_alc,
    // just in case the Allocator is an embedded field within a parent/holder it
    // should not outlive the parent
    pub(crate) _lifetime: std::marker::PhantomData<&'a ()>,
}

pub trait YyjsonAllocProvider {
    /// Creates and returns a `YyjsonAllocator`.
    ///
    /// # Lifetimes
    /// - `'a` ensures that the returned `YyjsonAllocator` cannot outlive the `self` reference,
    ///   which prevents use-after-free issues if `self` owns or manages the underlying memory.
    fn get_allocator<'a>(&'a self) -> YyjsonAllocator<'a>;
}

// =============================================================================
//                  DEFAULT ALLOCATOR
// =============================================================================

pub struct BasicAllocProvider {}
impl Default for BasicAllocProvider {
    fn default() -> Self {
        return BasicAllocProvider {};
    }
}
impl YyjsonAllocProvider for BasicAllocProvider {
    fn get_allocator<'a>(&'a self) -> YyjsonAllocator<'a> {
        YyjsonAllocator {
            p: ptr::null_mut(),
            _lifetime: std::marker::PhantomData,
        }
    }
}

// =============================================================================
//                  SINGLE BUF ALLOCATOR
// =============================================================================

// Uses a fixed length pre-allocated memory
pub struct PoolAllocProvider {
    alc: *mut ffi::yyjson_alc,
    buf: *mut u8,
    layout: Layout,
}

impl Drop for PoolAllocProvider {
    fn drop(&mut self) {
        // SAFETY: SingelBufAllocator can only be created if buf is non-null and
        // points to valid memory and layout is valid
        unsafe { alloc::dealloc(self.buf, self.layout) };

        // SAFETY:: alc is initialized from a pointer from the boxed value.
        let _ = unsafe { Box::from_raw(self.alc) };
    }
}

impl YyjsonAllocProvider for PoolAllocProvider {
    #[inline(always)]
    fn get_allocator<'a>(&'a self) -> YyjsonAllocator<'a> {
        YyjsonAllocator {
            p: self.alc,
            _lifetime: std::marker::PhantomData,
        }
    }
}

impl PoolAllocProvider {
    pub fn new(
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
        let sb = PoolAllocProvider { alc, buf, layout };

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
pub struct DynamicAllocProvider {
    alc: *mut ffi::yyjson_alc,
}

impl Default for DynamicAllocProvider {
    fn default() -> Self {
        let alc: *mut ffi::yyjson_alc = unsafe { ffi::yyjson_alc_dyn_new() };
        Self { alc }
    }
}

impl YyjsonAllocProvider for DynamicAllocProvider {
    #[inline(always)]
    fn get_allocator<'a>(&'a self) -> YyjsonAllocator<'a> {
        YyjsonAllocator {
            p: self.alc,
            _lifetime: std::marker::PhantomData,
        }
    }
}

impl Drop for DynamicAllocProvider {
    fn drop(&mut self) {
        // SAFETY: by construction, either DynamicAllocator.alc is null in which
        // case `yyjson_alc_dyn_free` will not error out or is non-null in which
        // case it points to a valid yyjson_alc that will be freed appropriately
        unsafe { ffi::yyjson_alc_dyn_free(self.alc) }
    }
}
