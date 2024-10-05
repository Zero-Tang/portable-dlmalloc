// Rust example for using portable dlmalloc
use core::ffi::c_void;
use std::alloc::*;
use std::process::abort;
use std::ptr::null_mut;

use core::fmt;

use windows::Win32::System::{Memory::*,Threading::*,Console::*};

// Implement a formatter without alloc operation!
struct FormatBuffer
{
	buffer:[u8;512],
	used:usize
}

impl FormatBuffer
{
	fn new()->Self
	{
		FormatBuffer{buffer:[0;512],used:0}
	}
}

impl fmt::Write for FormatBuffer
{
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		let remainder=&mut self.buffer[self.used..];
		let current=s.as_bytes();
		if remainder.len()<current.len()
		{
			return Err(fmt::Error);
		}
		remainder[..current.len()].copy_from_slice(current);
		self.used+=current.len();
		return Ok(());
	}
}

#[no_mangle] fn system_print(args:fmt::Arguments)
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

macro_rules! naprint
{
	($($args:tt)*) =>
	{
		system_print(format_args!($($args)*))
	};
}

extern "C"
{
	fn dlmalloc(length:usize)->*mut c_void;
	fn dlfree(ptr:*mut c_void)->();
	fn dlrealloc(ptr:*mut c_void,length:usize)->*mut c_void;
}

struct DLMalloc;

unsafe impl GlobalAlloc for DLMalloc
{
	unsafe fn alloc(&self, layout:Layout) -> *mut u8
	{
		naprint!("[malloc] length: 0x{:X}\n",layout.size());
		dlmalloc(layout.size()).cast()
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout)
	{
		naprint!("[free] ptr: 0x{:p}\n",ptr);
		dlfree(ptr.cast())
	}

	unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8
	{
		naprint!("[realloc] ptr: 0x{:p} length: 0x{:X}\n",ptr,new_size);
		dlrealloc(ptr.cast(),new_size).cast()
	}
}

#[global_allocator] static GLOBAL_ALLOCATOR:DLMalloc=DLMalloc;

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

fn main()
{
	let p:Box<u32>=Box::new(55);
	println!("Hello, world! {}\n",p);
}
