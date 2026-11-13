#![no_std]

use core::{alloc::*, ffi::c_void, ptr::null_mut, sync::atomic::*};
#[cfg(feature = "alt-alloc")]
use core::{ptr::NonNull, slice};

/// This module defines C FFI definitions of dlmalloc library.
/// Use this library only if you understand the safety.
pub mod raw;
use raw::*;

unsafe extern "C"
{
	fn memcpy(dest:*mut c_void,src:*const c_void,cb:usize)->*mut c_void;
}

/// ## DLMalloc allocator
/// This is the default allocator.
pub struct DLMalloc;

unsafe impl GlobalAlloc for DLMalloc
{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		unsafe
		{
			dlmemalign(layout.align(),layout.size()).cast()
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout)
	{
		unsafe
		{
			dlfree(ptr.cast())
		}
	}

	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
	{
		// We can optimize the `realloc` method by trying realloc-in-place.
		let p=unsafe{dlrealloc_in_place(ptr.cast(),new_size)};
		if p==ptr.cast()
		{
			// In-place reallocation is successful. Just return the original pointer.
			ptr
		}
		else
		{
			// Failed to reallocate in-place! Try to allocate a new chunk.
			assert!(p.is_null(),"dlrealloc_in_place returned Non-Null pointer on failure!");
			let p=unsafe{dlmemalign(layout.align(),new_size)};
			if !p.is_null()
			{
				unsafe
				{
					memcpy(p,ptr.cast(),layout.size().min(new_size));
					dlfree(ptr.cast());
				}
			}
			p.cast()
		}
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
			self.mspace.store(unsafe{create_mspace(self.capacity,1)},Ordering::Release);
		}
		unsafe{mspace_memalign(self.mspace.load(Ordering::Acquire),layout.align(),layout.size()).cast()}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout)
	{
		unsafe{mspace_free(self.mspace.load(Ordering::Acquire),ptr.cast())}
	}

	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
	{
		// We can optimize the `realloc` method by trying realloc-in-place.
		let p=unsafe{mspace_realloc_in_place(self.mspace.load(Ordering::Acquire),ptr.cast(),new_size)};
		if p==ptr.cast()
		{
			// In-place reallocation is successful. Just return the original pointer.
			ptr
		}
		else
		{
			// Failed to reallocate in-place! Try to allocate a new chunk.
			assert!(p.is_null(),"mspace_realloc_in_place returned Non-Null pointer on failure!");
			let p=unsafe{mspace_memalign(self.mspace.load(Ordering::Acquire),layout.align(),new_size)};
			if !p.is_null()
			{
				unsafe
				{
					memcpy(p,ptr.cast(),layout.size().min(new_size));
					mspace_free(self.mspace.load(Ordering::Acquire),ptr.cast());
				}
			}
			p.cast()
		}
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
	/// Destroys `MspaceAlloc` object. All `mmap`ed pages will be released.
	/// 
	/// ## Safety
	/// You must ensure all allocated objects (Vec, Box, etc.) are dropped before destroying the allocator!
	pub unsafe fn destroy(&self)
	{
		if self.init.compare_exchange(true,false,Ordering::Acquire,Ordering::Relaxed).is_ok()
		{
			unsafe
			{
				destroy_mspace(self.mspace.load(Ordering::Acquire));
			}
		}
	}
}

#[cfg(feature = "alt-alloc")]
unsafe impl Allocator for MspaceAlloc
{
	fn allocate(&self,layout:Layout)->Result<NonNull<[u8]>,AllocError>
	{
		unsafe
		{
			// Lazily initialize mspace.
			if self.init.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_ok()
			{
				let msp=create_mspace(self.capacity,1);
				if msp.is_null()
				{
					return Err(AllocError);
				}
				self.mspace.store(msp,Ordering::Release);
			}
			let p:*mut u8=mspace_memalign(self.mspace.load(Ordering::Acquire),layout.align(),layout.size()).cast();
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

	unsafe fn deallocate(&self,ptr:NonNull<u8>,_layout:Layout)
	{
		unsafe{mspace_free(self.mspace.load(Ordering::Acquire),ptr.as_ptr().cast())}
	}
}

impl Drop for MspaceAlloc
{
	fn drop(&mut self)
	{
		unsafe
		{
			self.destroy();
		}
	}
}