// Rust example for using portable dlmalloc
use core::{fmt,ffi::c_void};
use std::{process::abort,ptr::null_mut};

use windows::Win32::System::{Memory::*,Threading::*,Console::*};

use crate::{FormatBuffer,naprint};

#[no_mangle] pub fn system_print(args:fmt::Arguments)
{
	let mut w=FormatBuffer::new();
	let r=fmt::write(&mut w,args);
	if let Ok(_)=r
	{
		let b=&w.buffer;
		let r=unsafe{GetStdHandle(STD_OUTPUT_HANDLE)};
		if let Ok(h)=r
		{
			let mut size:u32=w.used as u32;
			let _=unsafe{WriteConsoleA(h, b, Some(&mut size as *mut u32), None)};
		}
	}
}

// Implement required port routines for dlmalloc
#[no_mangle] extern "C" fn custom_abort()->!
{
	naprint!("The dlmalloc library executed abort!\n");
	abort()
}

#[no_mangle] unsafe extern "C" fn custom_mmap(length:usize)->*mut c_void
{
	let p=VirtualAlloc(None, length, MEM_COMMIT, PAGE_READWRITE);
	naprint!("[mmap] page: {:p}, length: 0x{:X}\n",p,length);
	if p==null_mut() {null_mut::<u8>().sub(1).cast()} else {p}
}

#[no_mangle] unsafe extern "C" fn custom_munmap(ptr:*mut c_void,length:usize)->i32
{
	let b=VirtualFree(ptr,length,MEM_DECOMMIT);
	naprint!("[munmap] Ptr: 0x{:p}, Size: 0x{:X}\n",ptr,length);
	match b
	{
		Ok(_)=>0,
		Err(_)=>-1
	}
}

#[no_mangle] unsafe extern "C" fn custom_direct_mmap(_length:usize)->*mut c_void
{
	null_mut::<u8>().sub(1).cast()
}

#[no_mangle] unsafe extern "C" fn init_lock(lock:*mut SRWLOCK)->()
{
	naprint!("[lock] initializing lock...\n");
	*lock=SRWLOCK_INIT;
}

#[no_mangle] unsafe extern "C" fn acquire_lock(lock:*mut SRWLOCK)->()
{
	naprint!("[lock] acquiring lock...\n");
	AcquireSRWLockExclusive(lock);
}

#[no_mangle] unsafe extern "C" fn release_lock(lock:*mut SRWLOCK)->()
{
	naprint!("[lock] releasing lock...\n");
	ReleaseSRWLockExclusive(lock);
}

#[no_mangle] unsafe extern "C" fn final_lock(_lock:*mut SRWLOCK)->()
{
	// SRWLock requires no finalization.
}