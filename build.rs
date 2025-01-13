fn main()
{
	let mmap_granularity=match option_env!("DEFAULT_MMAP_GRANULARITY")
	{
		Some(s)=>s,
		None=>"0x200000"
	};
	// Check the integer prefix.
	let (radix,mg)=if mmap_granularity.starts_with("0x")
	{
		(16,&mmap_granularity[2..])
	}
	else
	{
		(10,mmap_granularity)
	};
	// Parse the string into an integer.
	match u64::from_str_radix(mg,radix)
	{
		Ok(g)=>
		{
			// Check if the granularity is a power of two.
			if g==0 || (g & (g-1))!=0
			{
				panic!("The granularity {mmap_granularity} ({g}) is not a power of two!");
			}
		}
		Err(e)=>panic!("Failed to parse granularity value! Reason: {e}")
	}
	cc::Build::new()
	.file("./malloc.c")
	.define("PORTABLE",None)
	.define("USE_DL_PREFIX",None)
	.define("USE_LOCKS","2")
	.define("DEFAULT_GRANULARITY",mmap_granularity)
	.define("NO_MALLOC_STATS","1")
	.define("MSPACES",None)
	.debug(true)
	.compile("dlmalloc");
	println!("cargo::rerun-if-changed=./malloc.c");
}