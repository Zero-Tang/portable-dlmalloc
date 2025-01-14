# portable-dlmalloc
Portable Fork of Doug Lea's `malloc` Implementation.

## Introduction
[This code is originally implemented by Doug Lea.](https://gee.cs.oswego.edu/dl/html/malloc.html) The original source code is no longer available from [the FTP URL listed in the website](ftp://g.oswego.edu/pub/misc/malloc.c), but you can still find it through [Wayback Machine](https://web.archive.org/web/20190530015756/ftp://g.oswego.edu/pub/misc/malloc.c).

This repository serves as a fork so that `dlmalloc` can be ported to any arbitrary platforms modularly.

This repository also contains a Rust crate so that you can use `portable-dlmalloc` in almost anywhere.

## Rust Crate
This repository is published to [crates.io](https://crates.io/crates/portable-dlmalloc). \
You may add this crate in your `no_std` projects.

## Port To Your Platform
To port `dlmalloc` to your platform, implement the following procedures:

- `custom_mmap`/`custom_munmap`: Allocate and free pages from the system. `mmap` should return `(void*)-1` to indicate failure instead of `NULL`. `munmap` should return `0` to indicate success, and `-1` to indicate failure.
	```C
	void* custom_mmap(size_t length);
	int custom_munmap(void* ptr,size_t length);
	```
- `custom_direct_mmap`: Extend the allocated pages. This is optional. Return `(void*)-1` to indicate failure/no-support.
	```C
	void* custom_direct_mmap(size_t length);
	```
- `init_lock`/`final_lock`/`acquire_lock`/`release_lock`: Implement thread-safety for `dlmalloc`. The minimal implementation can be a simple spinlock. If you do not need thread-safety, define `USE_LOCKS=0` so you don't have to implement these routines.
	```C
	void init_lock(void* *lock);	// Initialize the mutex.
	void final_lock(void* *lock);	// Finalize the mutex.
	void acquire_lock(void* *lock);	// Acquire the mutex.
	void release_lock(void* *lock);	// Release the mutex.
	```
- `custom_abort`: Implement `abort()` routine. `dlmalloc` calls `custom_abort()` when internal assertion fails. You may use panic here and print detailed information.
	```C
	void custom_abort(char* message,const char* src_file_name,const int src_line_number);
	```
- `dprintf2`: Implement debug-printer, which can trace file-name and line-numbers, for `dlmalloc`. Note that the format specifiers are implementation-specific. For example, you might have to use `%a` to print ASCII-strings in EDK2. You do not have to implement `dprintf2` if you won't debug `malloc.c`.
	```C
	int dprintf2(const char* src_fn,const int src_ln,const char* fmt,...);
	```
	If you need to print stuff in `malloc.c` codes, just use the `dprintf` macro.

- `memcpy`/`memset`: I suppose no explanations are needed for these two. `dlmalloc` uses these two routines, but they can be easily implemented. Some embedded SDKs may even include `memcpy`/`memset`, though they might not have `malloc`.

When you compile `malloc.c`, define the following preprocessor flags:

- `PORTABLE`: This flag makes `dlmalloc` call `custom_mmap`, `custom_munmap` and `custom_direct_mmap` functions. This flag is required to port `dlmalloc` to your platform.
- `NO_MALLOC_STATS=1`: This flag forbids `dlmalloc` from outputting debug messages. Otherwise, you will have to implement `dprintf2`.
- `USE_LOCKS=2`: This flag makes `dlmalloc` call `init_lock`, `final_lock`, `acquire_lock` and `release_lock` functions. This flag is required to port `dlmalloc` to your platform while ensuring thread-safety. Otherwise, define `USE_LOCKS=0`.
- `USE_DL_PREFIX`: This flags makes all routines from `dlmalloc` has `dl` prefix so that you can avoid name-conflict issues.
- `DEFAULT_GRANULARITY`: This constant determines the size granularity when `dlmalloc` calls `custom_mmap`. The default is 64KiB.

Note: the following samples with Custom Port do not keep track of allocated pages. There will be unreleased pages after your program quits even if you freed all allocated buffers. If you port `dlmalloc` to programs that are volatile to its address-space (e.g.: Kernel-Mode drivers, pluggable dynamic libraries), you have to keep track of allocated pages and release them before you quit.

## Independent-Allocation API
The `dlmalloc` library also provides `Independent-Allocation` API, which allocates multiple elements all at once, and there memory addresses are guaranteed to be as close as possible. However, there is no `dlmemalign`-equivalent routines in this API. So alignments are not guaranteed to your needs. \
To utilize `Independent-Allocation` API, use `dlindependent_calloc` (all elements have the same size) or `dlindependent_comalloc` (each element has different size) to allocate memories, and use `dlbulk_free` to free them all at once.

## Mspace API
The `dlmalloc` library also provides `mspace` API, which uses a separate memory space to allocate memories. This set of API may be useful to isolate memory allocations. \
To utilize the `mspace` API, use `create_mspace` or `create_mspace_with_base` to create a memory space; use `mspace_malloc` (default alignment) or `mspace_memalign` (special alignment) to allocate memory; use `mspace_free` to free memory; and eventually use `destroy_mspace` to destroy the memory space.

Mspace API also provides `Independent-Allocation` API.

## Samples
This chapter describes how to build samples with `dlmalloc` library. Note that this chapter emphasizes on the given samples. If you wish to port `dlmalloc` to your platform, it's recommended to check out [Port to Your Platform](#port-to-your-platform) section, which provides the generalized guide for building the portable `dlmalloc`. You should be able to use this generalized guide if you are familiar with your compiler suite / build system.

### Build for Windows with Internal Port
Download [Windows Driver Kit 7.1 (WDK-7600)](https://www.microsoft.com/en-us/download/details.aspx?id=11800) and install to default location in C: drive.

Execute `compchk_win7x64.bat` (Debug/Check version, optimizations are disabled) and `compfre_win7x64.bat` (Release/Free version, optimizations are enabled) to build DLLs. The DLL has dependencies on `msvcrt.dll` and `kernel32.dll`, which are present in most Windows systems. The DLL exports both `dlmalloc` and `malloc`, just in case you might encounter name conflicts. This file is ready-to-use.

This option will build `dlmalloc` into a very small binary size. Note that `mmap` and `munmap` emulations via `VirtualAlloc` and `VirtualFree` are already implemented by Doug Lea.

The `dlmalloc.dll` can be directly used.

See `sample.c` for details.

### Build for Windows with Custom Port
Download [Enterprise Windows Driver Kit for Windows 11, version 24H2 with Visual Studio Build Tools 17.10.5 (EWDK-26100)](https://learn.microsoft.com/en-us/legal/windows/hardware/enterprise-wdk-license-2022) and mount it to V: drive. You may use [WinCDEmu](https://wincdemu.sysprogs.org/download/) to mount ISO images.

Execute `compchk_win11x64.bat` (Debug/Check version, optimizations are disabled) and `compfre_win11x64.bat` (Release/Free version, optimizations are enabled) to build static libraries and corresponding samples.

This option serves as a basic sample for implementing necessary routines for thread-safe `dlmalloc`. This sample uses Windows `SRWLock` to act as a lightweight mutex.

The `dlmalloc.lib` can be directly used in anywhere, as long as the program uses MSVC ABI and PE-COFF executable format.

See `port_win.c` for details.

### Build for UEFI with Custom Port
Download [Enterprise Windows Driver Kit for Windows 11, version 24H2 with Visual Studio Build Tools 17.10.5 (EWDK-26100)](https://learn.microsoft.com/en-us/legal/windows/hardware/enterprise-wdk-license-2022) and mount it to V: drive. You may use [WinCDEmu](https://wincdemu.sysprogs.org/download/) to mount ISO images.

Clone the [EDK-II-Library](https://github.com/Zero-Tang/EDK-II-Library) recursively and create the `EDK2_PATH` environment variable pointing to the `EDK-II-Library`.

Execute `compchk_uefix64.bat` (Debug/Check version, optimizations are disabled) and `compfre_uefix64.bat` (Release/Free version, optimizations are enabled) to build static libraries and corresponding samples.

This option serves as a basic sample for implementing necessary routines for thread-safe `dlmalloc` on UEFI platform. This sample implements a TAS spinlock to act as a mutex.

See `port_uefi.c` for details.

### Build for Rust with Custom Port
Install [Rust](https://www.rust-lang.org/) with target `x86_64-pc-windows-msvc`.

Execute:
```
cargo build --all
```

This option serves as a basic sample for utilizing `dlmalloc` as the Rust global allocator.

When you implement port functions, make sure you append `#[no_mangle] extern "C"` prefix on your function. For example:
```Rust
#[no_mangle] extern "C" fn custom_abort()->!
```

Note that `print!` (as well as `println!`) macro from rust std has allocation behaviors! To be precise, both `std::fmt::Arguments::to_string()` and `std::io::stdout().write()` have allocation behaviors! Therefore, this sample provides a `naprint!` macro for you to trace allocation operations without recursion. Be careful, the `naprint!` macro cannot handle formatted output larger than 512 bytes! \
Therefore, if you use `print!`/`println!` in `dlmalloc` port routines, you will find your program being deadlocked, as `SRWLock` does not support recursive locking.

## License
This repository is under the [MIT license](./license.txt). \
If you do not want to obey the MIT license, just [use the original `dlmalloc` implementation](https://web.archive.org/web/20190530015756/ftp://g.oswego.edu/pub/misc/malloc.c) instead. It is in the [Public Domain](https://wiki.creativecommons.org/wiki/public_domain).