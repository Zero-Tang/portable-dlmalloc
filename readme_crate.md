# portable-dlmalloc
Portable Fork of Doug Lea's `malloc` Implementation for Rust.

## Introduction
[This code is originally implemented by Doug Lea.](https://gee.cs.oswego.edu/dl/html/malloc.html) The original source code is no longer available from [the FTP URL listed in the website](ftp://g.oswego.edu/pub/misc/malloc.c), but you can still find it through [Wayback Machine](https://web.archive.org/web/20190530015756/ftp://g.oswego.edu/pub/misc/malloc.c).

You may use this [crate](https://crates.io/crates/portable-dlmalloc) to help you make a portable global allocator. \
You will have to implement the eight C functions as described in [Port To Your Platform](#port-to-your-platform) chapter.

### Global Allocator
To use this crate as your global allocator:
```Rust
use portable_dlmalloc::DLMalloc;
#[global_alloactor] static GLOBAL_ALLOCATOR:DLMalloc=DLMalloc;
```
Then you will be able to use `alloc` crate.
```
extern crate alloc;
```

The default alignment of `dlmalloc` is twice the pointer size (e.g.: 16 bytes on 64-bit systems). \
If you need to use a different alignment, use `dlmemalign` function to implement your [`GlobalAlloc` trait](https://doc.rust-lang.org/alloc/alloc/trait.GlobalAlloc.html).

### Alternate Allocator
The [Allocator Trait](https://doc.rust-lang.org/alloc/alloc/trait.Allocator.html) is currently nightly-only. \
Currently, this crate does not support this trait.

### Raw FFI
The `raw` module from this crate exports FFI bindings for `dlmalloc` library.
```Rust
use portable_dlmalloc::raw::*;
```
For example, you may use `dlmallopt` to adjust `mmap` granularity (default is 2MiB in Rust crate):
```Rust
dlmallopt(M_GRANULARITY,0x20000);	// Change `mmap` granularity to 128KiB.
```
You may use `dlpvalloc` to allocate memory on page-granularity.
```Rust
let p=dlpvalloc(12345);
assert_eq!(p as usize & 0xfff,0);
```
**Warning**: `dlpvalloc` - as well as other routines that allocate memories with higher granularities - may cause serious memory fragmentation if you overrely on them.
```Rust
// Assume 4096 is page size.
let p=dlpvalloc(4096) as usize;
let q=dlpvalloc(4096) as usize;
// Consecutive allocations do not guarantee them to be adjacent.
assert_eq!(q-p,8192);
```

## Port to Your Platform
To port `dlmalloc` to your platform, implement the following procedures:

- `custom_mmap`/`custom_munmap`: Allocate and free pages from the system. `mmap` should return `(void*)-1` to indicate failure instead of `NULL`. `munmap` should return `0` to indicate success, and `-1` to indicate failure.
	```Rust
	#[no_mangle] unsafe extern "C" custom_mmap(length:usize)->*mut c_void;
	#[no_mangle] unsafe extern "C" custom_munmap(ptr:*mut c_void,length:usize)->i32;
	```
- `custom_direct_mmap`: Extend the allocated pages. This is optional. Return `(void*)-1` to indicate failure/no-support.
	```Rust
	#[no_mangle] unsafe extern "C" custom_mmap(length:usize)->*mut c_void;
	```
- `init_lock`/`final_lock`/`acquire_lock`/`release_lock`: Implement thread-safety for `dlmalloc`. The minimal implementation can be a simple spinlock. You can leave the implementations empty for this set of routines if you do not need thread-safety.
	```Rust
	#[no_mangle] unsafe extern "C" init_lock(lock:*mut *mut c_void);	// Initialize the mutex.
	#[no_mangle] unsafe extern "C" final_lock(lock:*mut *mut c_void);	// Finalize the mutex.
	#[no_mangle] unsafe extern "C" acquire_lock(lock:*mut *mut c_void);	// Acquire the mutex.
	#[no_mangle] unsafe extern "C" release_lock(lock:*mut *mut c_void);	// Release the mutex.
	```
- `custom_abort`: Implement `abort()` routine. `dlmalloc` calls `custom_abort()` when internal assertion fails. You may use panic here.
	```Rust
	#[no_mangle] unsafe extern "C" custom_abort()->!;
	```
- `memcpy`/`memset`: I suppose no explanations are needed for these two. `dlmalloc` uses these two routines, but they can be easily implemented anyway. You do not need to implement these two routines in Rust if your linker can find libraries that implement these two routines.

## Build
Since the core of the `dlmalloc` library is written in C, a working C compiler is required. \
If your target is somewhat unorthodox, you need to set environment variables before executing `cargo build`:

- `CC`: This environment variable specifies which compiler executable should be used to compile `malloc.c`
- `AR`: This environment variable specifies which archiver executable should be used to archive this crate into a static library.

If `cc` crate does not know how to invoke your compiler and/or archiver, you should write a script to emulate `cc` and/or `ar`.

## License
This crate is under the [MIT license](./license.txt).