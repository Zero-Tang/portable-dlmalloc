#![no_std]
#![feature(allocator_api,non_null_from_ref)]

use core::alloc::*;

/// This module defines C FFI definitions of dlmalloc library.
/// Use this library only if you understand the safety.
pub mod raw;
use raw::*;

#[cfg(feature="alt-alloc")] pub mod alt_alloc;

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