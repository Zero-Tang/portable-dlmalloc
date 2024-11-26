#![no_std]
#![feature(allocator_api,non_null_from_ref)]

use core::{alloc::*, ffi::*, ptr::NonNull, slice};

/// This module defines C FFI definitions of dlmalloc library.
/// Use this library only if you understand the safety.
pub mod raw;
use raw::*;

pub struct DLMalloc;

unsafe impl GlobalAlloc for DLMalloc
{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		dlmemalign(layout.align(),layout.size()).cast()
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout)
	{
		dlfree(ptr.cast());
	}

	unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8
	{
		dlrealloc(ptr.cast(),new_size).cast()
	}
}

/// ## Alternate Allocator API
/// The alternate allocator API is provided by the `mspace` functionality in dlmalloc library. \
/// Please note that Rust's support for allocator api is in nightly stage.
#[derive(Copy,Clone)] pub struct AltAlloc
{
	mspace:*mut c_void
}

impl AltAlloc
{
	pub fn new(capacity:usize,locked:bool)->Self
	{
		Self
		{
			mspace:unsafe{create_mspace(capacity,locked as i32)}
		}
	}

	pub fn destroy(self)
	{
		unsafe
		{
			destroy_mspace(self.mspace);
		}
	}
}

unsafe impl Allocator for AltAlloc
{
	fn allocate(&self, layout: Layout) -> Result<core::ptr::NonNull<[u8]>, AllocError>
	{
		unsafe
		{
			let p:*mut u8=mspace_memalign(self.mspace,layout.align(),layout.size()).cast();
			if p.is_null()
			{
				Err(AllocError)
			}
			else
			{
				let r:&mut [u8]=slice::from_raw_parts_mut(p,layout.size());
				Ok(NonNull::from_mut(r))
			}
		}
	}

	unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, _layout: Layout)
	{
		mspace_free(self.mspace,ptr.as_ptr().cast())
	}
}