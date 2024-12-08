#![no_std]
#![feature(allocator_api,non_null_from_ref)]

use core::{alloc::*, ffi::c_void, ptr::null_mut, sync::atomic::*};

/// This module defines C FFI definitions of dlmalloc library.
/// Use this library only if you understand the safety.
pub mod raw;
use raw::*;

#[cfg(feature="alt-alloc")] pub mod alt_alloc;

/// ## DLMalloc allocator
/// This is the default allocator.
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

/// ## MspaceAlloc allocator
/// This allocator allows you to use a initial capacity bigger than the default granularity.
pub struct MspaceAlloc
{
	mspace:AtomicPtr<c_void>,
	init:AtomicBool,
	capacity:usize
}

unsafe impl GlobalAlloc for MspaceAlloc
{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		// Lazily initialize mspace.
		if self.init.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_ok()
		{
			self.mspace.store(create_mspace(self.capacity,1),Ordering::Release);
		}
		mspace_memalign(self.mspace.load(Ordering::Acquire),layout.align(),layout.size()).cast()
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout)
	{
		mspace_free(self.mspace.load(Ordering::Acquire),ptr.cast())
	}

	unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8
	{
		mspace_realloc(self.mspace.load(Ordering::Acquire),ptr.cast(),new_size).cast()
	}
}

impl MspaceAlloc
{
	/// ## `new` method
	/// Initialize `MspaceAlloc` object. Use this method to initialize a static variable annotated as the global allocator.
	pub const fn new(capacity:usize)->Self
	{
		Self
		{
			// The `mspace` will be lazily initialized.
			mspace:AtomicPtr::new(null_mut()),
			init:AtomicBool::new(false),
			capacity
		}
	}

	/// ## `destroy` method
	/// Destroys `MspaceAlloc` object.
	/// 
	/// ## Safety
	/// You must ensure all allocated objects are dropped before destroying the allocator!
	pub unsafe fn destroy(&self)
	{
		if self.init.compare_exchange(true,false,Ordering::Acquire,Ordering::Relaxed).is_ok()
		{
			destroy_mspace(self.mspace.load(Ordering::Acquire));
		}
	}
}