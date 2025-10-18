// Rust example for using portable dlmalloc
use portable_dlmalloc::*;

#[cfg(target_os="windows")] mod win;
#[cfg(target_os="linux")] mod linux;

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
// #[global_allocator] static MSPACE_ALLOCATOR:MspaceAlloc=MspaceAlloc::new(0x200000);
// #[global_allocator] static SYSTEM_ALLOCATOR:SysAlloc=SysAlloc;

#[derive(Debug)]
#[repr(C,align(0x400))] struct AlignedHigher
{
	a:u8,
	b:u16,
	c:u32
}

/// ## Safety
/// The lifetime of the returned reference is not guaranteed to be safe. \
/// You have to manually validate the lifetime on your own.
unsafe fn nulstr_from_ptr<'a>(string:*const u8)->&'a str
{
	// Check the length of string.
	let mut s:usize=0;
	loop
	{
		if string.add(s).read()==0
		{
			break;
		}
		s+=1;
	}
	let str_slice=core::slice::from_raw_parts(string,s);
	core::str::from_utf8(str_slice).unwrap()
}

#[no_mangle] extern "C" fn custom_abort(message:*const u8,src_file:*const u8,src_line:u32)->!
{
	let msg=unsafe{nulstr_from_ptr(message)};
	let sfn=unsafe{nulstr_from_ptr(src_file)};
	panic!("The dlmalloc library executed abort! Reason: {msg}\n{sfn}@{src_line}");
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
	// Verify the realloc alignment.
	let mut v1:Vec<AlignedHigher>=Vec::with_capacity(1);
	v1.push(AlignedHigher{a:1,b:2,c:3});
	let v2=vec![555;2345];
	naprintln!("v1: {:p}, v2: {:p}",v1.as_ptr(),v2.as_ptr());
	v1.push(AlignedHigher{a:3,b:2,c:1});
	naprintln!("v1: {v1:?} is located at {:p}",v1.as_ptr());
}
