// Rust example for using portable dlmalloc
use core::{fmt,ffi::c_void,ptr::null_mut,sync::atomic::{AtomicUsize,Ordering}};
use crate::{naprint, FormatBuffer};

use libc::*;

pub fn system_print(args:fmt::Arguments)
{
	let mut w=FormatBuffer::default();
	let r=fmt::write(&mut w,args);
	if r.is_ok()
	{
		let b=&w.buffer;
		unsafe
		{
			write(1,b.as_ptr().cast(),w.used);
		}
	}
}

// Implement required port routines for dlmalloc
#[no_mangle] unsafe extern "C" fn custom_mmap(length:usize)->*mut c_void
{
	let p=mmap(null_mut(),length,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);
	naprint!("[mmap] page: {:p}, length: 0x{:X}\n",p,length);
	if p==MAP_FAILED
	{
		let errno_p=__errno_location();
		naprint!("[mmap] Failed to allocate! Reason: {}\n",*errno_p);
	}
	p
}

#[no_mangle] unsafe extern "C" fn custom_munmap(ptr:*mut c_void,length:usize)->i32
{
	let b=munmap(ptr,length);
	naprint!("[munmap] Ptr: 0x{:p}, Size: 0x{:X}\n",ptr,length);
	b
}

#[no_mangle] unsafe extern "C" fn init_lock(lock:*mut usize)
{
	naprint!("[lock] initializing lock {lock:p}...\n");
	*lock=0;
}

#[no_mangle] unsafe extern "C" fn acquire_lock(lock:*mut usize)
{
	naprint!("[lock] acquiring lock {lock:p}...\n");
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
	naprint!("[lock] releasing lock {lock:p}...\n");
	AtomicUsize::from_ptr(lock).store(0,Ordering::Release);
}

#[no_mangle] unsafe extern "C" fn final_lock(lock:*mut usize)
{
	naprint!("[lock] finalizing lock {lock:p}...\n");
}