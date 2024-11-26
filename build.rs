fn main()
{
	cc::Build::new()
	.file("./malloc.c")
	.define("PORTABLE",None)
	.define("USE_DL_PREFIX",None)
	.define("USE_LOCKS","2")
	.define("DEFAULT_GRANULARITY","0x200000")
	.define("NO_MALLOC_STATS","1")
	.define("MSPACES",None)
	.compile("dlmalloc");
	println!("cargo::rerun-if-changed=./malloc.c");
}