// Rust example for using portable dlmalloc
use core::{fmt,ffi::c_void,ptr::null_mut,sync::atomic::{AtomicUsize,Ordering}};

use libc::*;
use static_collections::string::StaticString;

use crate::naprintln;

#[macro_export] macro_rules! naprint
{
	($($args:tt)*) =>
	{
		$crate::linux::system_print(format_args!($($args)*))
	};
}

pub fn system_print(args:fmt::Arguments)
{
	let mut w:StaticString<1024>=StaticString::new();
	if fmt::write(&mut w,args).is_ok()
	{
		unsafe
		{
			write(1,w.as_ptr().cast(),w.len());
		}
	}
}

// Implement required port routines for dlmalloc
#[no_mangle] unsafe extern "C" fn custom_mmap(length:usize)->*mut c_void
{
	let p=mmap(null_mut(),length,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);
	naprintln!("[mmap] page: {:p}, length: 0x{:X}",p,length);
	if p==MAP_FAILED
	{
		let errno_p=__errno_location();
		naprintln!("[mmap] Failed to allocate! Reason: {}",*errno_p);
	}
	p
}

#[no_mangle] unsafe extern "C" fn custom_munmap(ptr:*mut c_void,length:usize)->i32
{
	let b=munmap(ptr,length);
	naprintln!("[munmap] Ptr: 0x{:p}, Size: 0x{:X}",ptr,length);
	b
}

#[no_mangle] unsafe extern "C" fn init_lock(lock:*mut usize)
{
	naprintln!("[lock] initializing lock {lock:p}...");
	*lock=0;
}

#[no_mangle] unsafe extern "C" fn acquire_lock(lock:*mut usize)
{
	naprintln!("[lock] acquiring lock {lock:p}...");
	let p=AtomicUsize::from_ptr(lock);
	while p.compare_exchange(0,1,Ordering::Acquire,Ordering::Relaxed).is_err()
	{
		while p.load(Ordering::Relaxed)!=0
		{
			
		}
	}
}

#[no_mangle] unsafe extern "C" fn release_lock(lock:*mut usize)
{
	naprintln!("[lock] releasing lock {lock:p}...");
	AtomicUsize::from_ptr(lock).store(0,Ordering::Release);
}

#[no_mangle] unsafe extern "C" fn final_lock(lock:*mut usize)
{
	naprintln!("[lock] finalizing lock {lock:p}...");
}