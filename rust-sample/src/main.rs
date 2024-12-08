// Rust example for using portable dlmalloc
#![feature(allocator_api)]

use core::{fmt,ptr::null_mut,ffi::c_void};
use portable_dlmalloc::{alt_alloc::AltAlloc, DLMalloc};

#[cfg(target_os="windows")] mod win;
#[cfg(target_os="windows")] use win::*;

// Implement a formatter without alloc operation!
struct FormatBuffer
{
	buffer:[u8;512],
	used:usize
}

impl Default for FormatBuffer
{
	fn default() -> Self
	{
		Self
		{
			buffer:[0;512],
			used:0
		}
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
		Ok(())
	}
}

#[macro_export] macro_rules! naprint
{
	($($args:tt)*) =>
	{
		system_print(format_args!($($args)*))
	};
}

#[macro_export] macro_rules! naprintln
{
	() =>
	{
		naprint!("\n")
	};
	($($arg:tt)*) =>
	{
		naprint!("{}\n",format_args!($($arg)*))
	};
}

#[global_allocator] static GLOBAL_ALLOCATOR:DLMalloc=DLMalloc;
// static SYSTEM_ALLOCATOR:SysAlloc=SysAlloc;

#[derive(Debug)]
#[repr(C,align(0x400))] struct AlignedHigher
{
	a:u8,
	b:u16,
	c:u32
}

#[no_mangle] extern "C" fn custom_abort()->!
{
	panic!("The dlmalloc library executed abort!\n");
}

#[no_mangle] unsafe extern "C" fn custom_direct_mmap(_length:usize)->*mut c_void
{
	null_mut::<u8>().sub(1).cast()
}

fn main()
{
	let p:Box<u32>=Box::new(55);
	let q:Box<AlignedHigher>=Box::new(AlignedHigher{a:4,b:555,c:6666666});
	let mut v:Vec<u32>=vec![5,4,1,6,3,8,9];
	v.sort();
	naprintln!("Hello, world! {}\n{v:?}\n{q:?}",p);
	naprintln!("{:p} {:p} {:p}",&raw const *p,v.as_ptr(),&raw const *q);
	let pp=&raw const *p;
	let pq=&raw const *q;
	// Verify the alignment.
	assert_eq!(pp as usize & (align_of::<u32>()-1),0);
	assert_eq!(pq as usize & (align_of::<AlignedHigher>()-1),0);
	naprintln!("Testing allocator API...");
	// Try allocator API.
	let a=AltAlloc::new(0x300000,true);
	let ab:Box::<u32,&AltAlloc>=Box::new_in(4,&a);
	naprintln!("{:p} | {}",&raw const *ab,ab);
	let mut av:Vec::<u32,&AltAlloc>=Vec::new_in(&a);
	for i in v
	{
		av.push(i);
	}
	naprintln!("av1: {:p} | {:?}",av.as_ptr(),av);
	let av2:Vec::<u8,&AltAlloc>=Vec::with_capacity_in(0x300000,&a);
	naprintln!("av2: {:p} | {:?}",av2.as_ptr(),av2);
}
