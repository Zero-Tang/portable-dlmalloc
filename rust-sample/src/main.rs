// Rust example for using portable dlmalloc
use core::ffi::c_void;
use std::alloc::*;
use core::fmt;

#[cfg(target_os="windows")] mod win;
#[cfg(target_os="windows")] use win::system_print;

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

#[macro_export] macro_rules! naprint
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

fn main()
{
	let p:Box<u32>=Box::new(55);
	println!("Hello, world! {}\n",p);
}
