use core::{alloc::*, ffi::*, ptr::NonNull, slice, sync::atomic::Ordering};
use crate::{raw::*, MspaceAlloc};

/// ## Alternate Allocator API
/// The alternate allocator API is provided by the `mspace` functionality in dlmalloc library. \
/// Please note that Rust's support for allocator api is in nightly stage. \
/// Do not worry about the performance of `Copy` trait, since this struct only carries a pointer to the created space.
pub struct AltAlloc
{
	mspace:*mut c_void
}

impl AltAlloc
{
	/// ## `new` method
	/// This method will create a new `AltAlloc` allocator with `create_mspace` from dlmalloc library.
	/// - `capacity` defines the initial size when creating the allocator. Specify 0 to use the default granularity size.
	/// - `locked` defines if this allocator should use a separate lock.
	pub fn new(capacity:usize,locked:bool)->Self
	{
		Self
		{
			mspace:unsafe
			{
				create_mspace(capacity,locked as i32)
			}
		}
	}

	/// ## `from_base` method
	/// This method will use existing `base` to create a new `AltAlloc` allocator. No allocations are made on creation. \
	/// Other arguments mean the same as `new` method.
	/// 
	/// Note that `AltAlloc` created from the `from_base` method does not automatically release the base. \
	/// When dropping `AltAlloc`, the expanded parts will be automatically released. \
	/// However, it is your responsibility to release the `base`.
	/// 
	/// ## Safety
	/// This method assumes the `base` is valid.
	pub unsafe fn from_base(base:*mut c_void,capacity:usize,locked:bool)->Self
	{
		Self
		{
			mspace:unsafe{create_mspace_with_base(base,capacity,locked as i32)}
		}
	}
}

impl Drop for AltAlloc
{
	fn drop(&mut self)
	{
		unsafe
		{
			destroy_mspace(self.mspace);
		}
	}
}

unsafe impl Allocator for AltAlloc
{
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>
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

	unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout)
	{
		unsafe{mspace_free(self.mspace,ptr.as_ptr().cast())}
	}
}

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