#![no_std]

use core::alloc::GlobalAlloc;

/// This module defines C FFI definitions of dlmalloc library.
/// Use this library only if you understand the safety.
pub mod raw;
use raw::*;

pub struct DLMalloc;

unsafe impl GlobalAlloc for DLMalloc
{
	unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8
	{
		dlmalloc(layout.size()).cast()
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout)
	{
		dlfree(ptr.cast());
	}

	unsafe fn realloc(&self, ptr: *mut u8, _layout: core::alloc::Layout, new_size: usize) -> *mut u8
	{
		dlrealloc(ptr.cast(),new_size).cast()
	}
}