# portable-dlmalloc
Portable Fork of Doug Lea's `malloc` Implementation.

## Introduction
[This code is originally implemented by Doug Lea.](https://gee.cs.oswego.edu/dl/html/malloc.html) The original source code is no longer available from [the FTP URL listed in the website](ftp://g.oswego.edu/pub/misc/malloc.c), but you can still find it through [Wayback Machine](https://web.archive.org/web/20190530015756/ftp://g.oswego.edu/pub/misc/malloc.c).

This repository serves as a fork so that `dlmalloc` can be ported to any arbitrary platforms modularly.

## Build
This chapter describes how to build this `dlmalloc` library.

### Build for UEFI
Download [Enterprise Windows Driver Kit for Windows 11, version 24H2 with Visual Studio Build Tools 17.10.5 (EWDK-26100)](https://learn.microsoft.com/en-us/legal/windows/hardware/enterprise-wdk-license-2022) and mount it to V: drive. You may use [WinCDEmu](https://wincdemu.sysprogs.org/download/) to mount ISO images.

This option is not available yet.

### Build for Windows with Legacy SDK
Download [Windows Driver Kit 7.1 (WDK-7600)](https://www.microsoft.com/en-us/download/details.aspx?id=11800) and install to default location in C: drive.

Execute `compchk_win7x64.bat` (Debug/Check version, optimizations are disabled) and `compfre_win7x64.bat` (Release/Free version, optimizations are enabled) to build DLLs. The DLL has dependencies on `msvcrt.dll` and `kernel32.dll`, which are present in most Windows systems. The DLL exports both `dlmalloc` and `malloc`, just in case you might encounter name conflicts.

This option will build `dlmalloc` into a very small binary size. Note that `mmap` and `munmap` emulations via `VirtualAlloc` and `VirtualFree` are already implemented by Doug Lea.

## License
This repository is under the [MIT license](./license.txt). \
If you do not want to obey the MIT license, [use the original `dlmalloc` implementation](https://web.archive.org/web/20190530015756/ftp://g.oswego.edu/pub/misc/malloc.c) instead. It is in the [Public Domain](https://wiki.creativecommons.org/wiki/public_domain).