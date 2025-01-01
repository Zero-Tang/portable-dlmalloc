// Rust example for using portable dlmalloc
use core::{fmt,ffi::c_void,ptr::null_mut};
use std::alloc::*;

use windows::Win32::System::{Memory::*,Threading::*,Console::*};

use crate::{naprint, naprintln, FormatBuffer};

pub struct SysAlloc;

unsafe impl GlobalAlloc for SysAlloc
{
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		naprintln!("[alloc] size: {} bytes, alignment: {} bytes",layout.size(),layout.align());
		HeapAlloc(GetProcessHeap().unwrap(),HEAP_FLAGS(0),layout.size()).cast()
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout)
	{
		naprintln!("[free] ptr: 0x{ptr:p}");
		let _=HeapFree(GetProcessHeap().unwrap(),HEAP_FLAGS(0),Some(ptr.cast()));
	}

	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
	{
		naprintln!("[alloc-zeroed] size: {} bytes, alignment: {} bytes",layout.size(),layout.align());
		HeapAlloc(GetProcessHeap().unwrap(),HEAP_ZERO_MEMORY,layout.size()).cast()
	}

	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
	{
		naprintln!("[realloc] ptr: {ptr:p} size: {} bytes, alignment: {} bytes",layout.size(),layout.align());
		HeapReAlloc(GetProcessHeap().unwrap(),HEAP_FLAGS(0),Some(ptr.cast()),new_size).cast()
	}
}

pub fn system_print(args:fmt::Arguments)
{
	let mut w=FormatBuffer::default();
	let r=fmt::write(&mut w,args);
	if r.is_ok()
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
#[no_mangle] unsafe extern "C" fn custom_mmap(length:usize)->*mut c_void
{
	let p=VirtualAlloc(None, length, MEM_COMMIT, PAGE_READWRITE);
	naprint!("[mmap] page: {:p}, length: 0x{:X}\n",p,length);
	if p.is_null() {null_mut::<u8>().sub(1).cast()} else {p}
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

#[no_mangle] unsafe extern "C" fn init_lock(lock:*mut SRWLOCK)
{
	naprint!("[lock] initializing lock {lock:p}...\n");
	*lock=SRWLOCK_INIT;
}

#[no_mangle] unsafe extern "C" fn acquire_lock(lock:*mut SRWLOCK)
{
	naprint!("[lock] acquiring lock {lock:p}...\n");
	AcquireSRWLockExclusive(lock);
}

#[no_mangle] unsafe extern "C" fn release_lock(lock:*mut SRWLOCK)
{
	naprint!("[lock] releasing lock {lock:p}...\n");
	ReleaseSRWLockExclusive(lock);
}

#[no_mangle] unsafe extern "C" fn final_lock(lock:*mut SRWLOCK)
{
	// SRWLock requires no finalization.
	naprint!("[lock] finalizing lock {lock:p}...\n");
}