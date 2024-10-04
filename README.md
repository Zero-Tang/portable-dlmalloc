# portable-dlmalloc
Portable Fork of Doug Lea's `malloc` Implementation.

## Introduction
[This code is originally implemented by Doug Lea.](https://gee.cs.oswego.edu/dl/html/malloc.html) The original source code is no longer available from [the FTP URL listed in the website](ftp://g.oswego.edu/pub/misc/malloc.c), but you can still find it through [Wayback Machine](https://web.archive.org/web/20190530015756/ftp://g.oswego.edu/pub/misc/malloc.c).

This repository serves as a fork so that `dlmalloc` can be ported to any arbitrary platforms modularly.

## Port To Your Platform
To port `dlmalloc` to your platform, implement the following procedures:

- `custom_mmap`/`custom_munmap`: Allocate and free pages from the system. `mmap` should return `(void*)-1` to indicate failure. `munmap` should return `0` to indicate success, and `-1` to indicate failure.
	```C
	void* custom_mmap(size_t length);
	int custom_munmap(void* ptr,size_t length);
	```
- `custom_direct_mmap`: Extend the allocated pages. This is optional. Should you wish to implement this, you would have to keep track of allocated pages. Return `(void*)-1` to indicate failure/no-support.
	```C
	void* custom_direct_mmap(size_t length);
	```
- `init_lock`/`final_lock`/`acquire_lock`/`release_lock`: Implement thread-safety for `dlmalloc`. The minimal implementation can be a simple spinlock. You do not need to implement this set of routines if you do not need thread-safety.
	```C
	void init_lock(void* *lock);	// Initialize the mutex.
	void final_lock(void* *lock);	// Finalize the mutex.
	void acquire_lock(void* *lock);	// Acquire the mutex.
	void release_lock(void* *lock);	// Release the mutex.
	```
- `dprintf2`: Implement debug-printer, which can trace file-name and line-numbers, for `dlmalloc`. Note that the format specifiers are implementation-specific. For example, you might have to use `%a` to print ASCII-strings in EDK2. You do not have to implement `dprintf2` if you won't debug `malloc.c`.
	```C
	int dprintf2(const char* src_fn,const int src_ln,const char* fmt,...);
	```
	If you need to print stuff in `malloc.c` codes, just use the `dprintf` macro.

When you compile `malloc.c`, define the following preprocessor flags:

- `PORTABLE`: This flag makes `dlmalloc` call `custom_mmap`, `custom_munmap` and `custom_direct_mmap` functions. This flag is required to port `dlmalloc` to your platform.
- `NO_MALLOC_STATS=1`: This flag forbids `dlmalloc` from outputting debug messages. Otherwise, you will have to implement `dprintf2`.
- `USE_LOCKS=2`: This flag makes `dlmalloc` call `init_lock`, `final_lock`, `acquire_lock` and `release_lock` functions. This flag is required to port `dlmalloc` to your platform while ensuring thread-safety. Otherwise, define `USE_LOCKS=0`.
- `USE_DL_PREFIX`: This flags makes all routines from `dlmalloc` has `dl` prefix so that you can avoid name-conflict issues.

## Build
This chapter describes how to build this `dlmalloc` library. Note that this chapter only guides you how to build the samples. If you wish to port `dlmalloc` to your platform, it's recommended to check out [Port to Your Platform](#port-to-your-platform) section.

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

## License
This repository is under the [MIT license](./license.txt). \
If you do not want to obey the MIT license, just [use the original `dlmalloc` implementation](https://web.archive.org/web/20190530015756/ftp://g.oswego.edu/pub/misc/malloc.c) instead. It is in the [Public Domain](https://wiki.creativecommons.org/wiki/public_domain).