# portable-dlmalloc
Portable Fork of Doug Lea's `malloc` Implementation for Rust.

## Introduction
[This code is originally implemented by Doug Lea.](https://gee.cs.oswego.edu/dl/html/malloc.html) The original source code is no longer available from [the FTP URL listed in the website](ftp://g.oswego.edu/pub/misc/malloc.c), but you can still find it through [Wayback Machine](https://web.archive.org/web/20190530015756/ftp://g.oswego.edu/pub/misc/malloc.c).

You may use this [crate](https://crates.io/crates/portable-dlmalloc) to help you make a portable global allocator. \
You will have to implement the eight C functions as described in [Port To Your Platform](#port-to-your-platform) chapter.

`dlmalloc` guarantees the alignment of allocation in comparison to just some wrappers on `malloc` functions (e.g.: wrapping `HeapAlloc` in Windows). If your structure is defined to be aligned on a big alignment (e.g.: 1024 bytes), this allocator guarantees the returned pointer if aligned on your specific boundary. The minimum alignment of `dlmalloc` is four times of the pointer size. (e.g.: 32 bytes on 64-bit platform.)

### Global Allocator
To use this crate as your global allocator:
```Rust
use portable_dlmalloc::DLMalloc;
#[global_alloactor] static GLOBAL_ALLOCATOR:DLMalloc=DLMalloc;
```
Then you will be able to use `alloc` crate.
```Rust
extern crate alloc;
```

The default alignment of `alloc` trait method automatically determines the required alignment.

Your `custom_mmap` implementation must track all allocated pages so that you can release pages in shared address-space (e.g.: DLL in Windows, SO in Linux). \
The `final_lock` routine will not be called for the global allocator! Your `init_lock` implementation must track all initialized locks so that you can finalize all locks, unless it is trivial to finalize the locks.

### Mspace Allocator
You can also use `MspaceAlloc` as your global allocator:
```Rust
use portable_dlmalloc:MspaceAlloc;
#[global_allocator] static GLOBAL_ALLOCATOR:MspaceAlloc=MspaceAlloc::new(0);
```
To destroy this allocator:
```Rust
unsafe
{
	GLOBAL_ALLOCATOR.destroy();
}
```
Use this allocator only if:
- You need a specific initial capacity bigger than default granularity.
- You need to destroy the allocator in one-shot, without tracing all allocated pages.

### Alternate Allocator
The [Allocator Trait](https://doc.rust-lang.org/alloc/alloc/trait.Allocator.html) is currently nightly-only. Therefore, the alternate allocator feature is only available in the 0.x version of this crate. This feature will not be included in 1.x version and later until the `Allocator Trait` is stablized.

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
	Hint: If you are in baremetal environment without `mmap`-like services like outdated embedded systems, you can just return the pointer to the free memory. \
	Note: This crate does not support `sbrk`, even though original implementation of `dlmalloc` supports it. Just emulate the behavior of `mmap` with `sbrk`.
- `custom_direct_mmap`: Extend the allocated pages. This is optional. Return `(void*)-1` to indicate failure/no-support.
	```Rust
	#[no_mangle] unsafe extern "C" custom_direct_mmap(length:usize)->*mut c_void;
	```
- `init_lock`/`final_lock`/`acquire_lock`/`release_lock`: Implement thread-safety for `dlmalloc`. The minimal implementation can be a simple spinlock. You can leave the implementations empty for this set of routines if you do not need thread-safety. \
	The exact type of `lock` depends on your implementation. It can be `*mut T` where T can be anything that has the size of a pointer.
	```Rust
	#[no_mangle] unsafe extern "C" init_lock(lock:*mut *mut c_void);    // Initialize the mutex.
	#[no_mangle] unsafe extern "C" final_lock(lock:*mut *mut c_void);   // Finalize the mutex.
	#[no_mangle] unsafe extern "C" acquire_lock(lock:*mut *mut c_void); // Acquire the mutex.
	#[no_mangle] unsafe extern "C" release_lock(lock:*mut *mut c_void); // Release the mutex.
	```
- `custom_abort`: Implement `abort()` routine. `dlmalloc` calls `custom_abort()` when internal assertion fails. You may use panic here.
	```Rust
	#[no_mangle] unsafe extern "C" custom_abort(message:*const u8,src_file:*const u8,src_line:u32)->!;
	```
	Note that `message` and `src_file` are null-terminated strings. The encodings of the string is implementation-specific of the C compiler. It is your duty to convert them into UTF-8 encoding. \
	The `message` string is guaranteed to contain ASCII characters only. However, `src_file` may contain non-ASCII characters.
- `memcpy`/`memset`: I suppose no explanations are needed for these two. `dlmalloc` uses these two routines, but they can be easily implemented anyway. You do not need to implement these two routines in Rust if your linker can find libraries that implement these two routines. Note that MSVC SDK provides source code of high-performance `memcpy` and `memset` implementations in Assembly!

Note: If you are using Rust 2024 or higher, you must use `#[unsafe(no_mangle)]` as prefix! See [Rust unsafe attributes](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-attributes.html) for more details. \
If, for some reasons, these procedure names must be reserved in your project, you may use the [`export_name` attribute](https://doc.rust-lang.org/reference/abi.html#the-export_name-attribute). Note that `export_name` attribute requires `unsafe` in Rust 2024 as well as `no_mangle`!

## Build
Since the core of the `dlmalloc` library is written in C, a working C compiler is required. \
If your target is somewhat unorthodox, you need to set the following environment variables before executing `cargo build`:

- `CC`: This environment variable specifies which compiler executable should be used to compile `malloc.c`.
- `AR`: This environment variable specifies which archiver executable should be used to archive this crate into a static library.
- `CFLAGS`: This environment variable specifies additional flags to the compiler. You might need this flag to add debug information (e.g.: `-g`). In kernel-mode with MSVC toolchain, you might need `/GS-` flag.

If `cc` crate does not know how to invoke your compiler and/or archiver, you should write a script to emulate `cc` and/or `ar`. \
In most circumstances, setting `CC` to `clang` and `AR` to `llvm-ar` should work well.

## License
This crate is under the [MIT license](./license.txt).