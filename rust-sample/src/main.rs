// Rust example for using portable dlmalloc
use core::fmt;
use portable_dlmalloc::DLMalloc;

#[cfg(target_os="windows")] mod win;

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

#[macro_export] macro_rules! naprintln
{
	()=>
	{
		system_print(format_args!("\n"))
	};
	($($args:tt)*)=>
	{
		system_print(format_args!($($args)*))
	};
}

#[global_allocator] static GLOBAL_ALLOCATOR:DLMalloc=DLMalloc;

fn main()
{
	let p:Box<u32>=Box::new(55);
	let mut v:Vec<u32>=vec![5,4,1,6,3,8,9];
	v.sort();
	println!("Hello, world! {}\n{v:?}",p);
}
