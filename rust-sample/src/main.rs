// Rust example for using portable dlmalloc
#![feature(allocator_api)]

use core::{alloc::AllocError};
use portable_dlmalloc::{alt_alloc::AltAlloc, DLMalloc, MspaceAlloc};

#[cfg(target_os="windows")] mod win;
#[cfg(target_os="linux")] mod linux;

#[macro_export] macro_rules! naprintln
{
	() =>
	{
		$crate::naprint!("\n")
	};
	($($arg:tt)*) =>
	{
		$crate::naprint!("{}\n",format_args!($($arg)*))
	};
}

#[global_allocator] static GLOBAL_ALLOCATOR:DLMalloc=DLMalloc;
// #[global_allocator] static SYSTEM_ALLOCATOR:SysAlloc=SysAlloc;
static MSPACE_ALT_ALLOCATOR:MspaceAlloc=MspaceAlloc::new(0x500000);

#[derive(Debug,Default)]
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

fn test_realloc()
{
	naprintln!("Testing realloc-alignment...");
	// Try realloc alignemnt.
	let mut v1:Vec::<AlignedHigher>=Vec::with_capacity(2);
	v1.push(AlignedHigher::default());
	v1.push(AlignedHigher::default());
	let v2=vec![123;2345];
	naprintln!("v1: {:p}, v2: {:p}",v1.as_ptr(),v2.as_ptr());
	v1.push(AlignedHigher::default());
	naprintln!("v1: {:p}",v1.as_ptr());
}

fn main()
{
	test_realloc();
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
	// Try allocator API on MspaceAlloc.
	naprintln!("Testing allocator API on Mspace API...");
	let ab:Box::<u32,&MspaceAlloc>=Box::new_in(15,&MSPACE_ALT_ALLOCATOR);
	let ac:Result<Box::<u32,&MspaceAlloc>,AllocError>=Box::try_new_in(26,&MSPACE_ALT_ALLOCATOR);
	naprintln!("ab: {:p}, ac: {:p}",&raw const ab,&raw const ac);
}
