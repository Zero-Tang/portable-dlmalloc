@echo off
set ddkpath=V:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.38.33130
set path=%ddkpath%\bin\Hostx64\x64;V:\Program Files\Windows Kits\10\bin\10.0.26100.0\x64;%path%
set incpath=V:\Program Files\Windows Kits\10\Include\10.0.26100.0
set mdepath=%EDK2_PATH%\edk2\MdePkg
set libpath=%EDK2_PATH%\bin\MdePkg
set binpath=.\bin\compfre_uefix64
set objpath=.\bin\compfre_uefix64\Intermediate

if not exist %objpath% (mkdir %objpath%)

echo Compiling dlmalloc...
cl malloc.c /I"%incpath%\ucrt" /I"%ddkpath%\include" /D"PORTABLE" /D"USE_DL_PREFIX" /D"NO_MALLOC_STATS=1" /D"USE_LOCKS=2" /D"DEFAULT_GRANULARITY=0x200000" /Zi /nologo /W3 /WX /O2 /Zc:wchar_t /FAcs /Fa"%objpath%\malloc.cod" /Fo"%objpath%\malloc.obj" /Fd"%objpath%\vc140.pdb" /GS- /std:c17 /Qspectre /TC /c /errorReport:queue

lib "%objpath%\malloc.obj" /NOLOGO /OUT:"%binpath%\dlmalloc.lib" /Machine:X64 /ERRORREPORT:QUEUE

echo Compiling Sample...
cl .\port_uefi.c /I"%mdepath%\Include" /I"%mdepath%\Include\X64" /Zi /nologo /W3 /WX /O1 /Zc:wchar_t /FAcs /Fa"%objpath%\sample.cod" /Fo"%objpath%\sample.obj" /Fd"%objpath%\vc140.pdb" /GS- /TC /c /errorReport:queue

ml64 /W3 /WX /Zf /Zd /Fo"%objpath%\memcpy.obj" /I"%incpath%/shared" /c /nologo "%ddkpath%\crt\src\x64\memcpy.asm"

ml64 /W3 /WX /Zf /Zd /Fo"%objpath%\memset.obj" /I"%incpath%/shared" /c /nologo "%ddkpath%\crt\src\x64\memset.asm"

lib "%ddkpath%\lib\x64\msvcrt.lib" /NOLOGO /EXTRACT:"D:\a\_work\1\s\Intermediate\crt\vcstartup\build\md\msvcrt_kernel32\msvcrt_kernel32.nativeproj\objr\amd64\cpu_disp.obj" /OUT:"%objpath%\cpu_disp.obj"

link "%objpath%\sample.obj" "%objpath%\memcpy.obj" "%objpath%\cpu_disp.obj" "%objpath%\memset.obj" "%binpath%\dlmalloc.lib" /NODEFAULTLIB /LIBPATH:"%libpath%\compfre_uefix64" "MdePkgGuids.lib" "BaseLib.lib" "BaseDebugPrintErrorLevelLib.lib" "BaseMemoryLib.lib" "BasePrintLib.lib" "UefiLib.lib" "UefiDebugLibConOut.lib" "UefiMemoryAllocationLib.lib" "UefiDevicePathLibDevicePathProtocol.Lib" "UefiBootServicesTableLib.Lib" "UefiRuntimeServicesTableLib.Lib" /NOLOGO /INCREMENTAL:NO /OPT:REF /OPT:ICF /DEBUG /PDB:"%binpath%\bootx64.pdb" /OUT:"%binpath%\bootx64.efi" /ENTRY:"EfiEntry" /SUBSYSTEM:EFI_APPLICATION /Machine:X64 /ERRORREPORT:QUEUE

set /A imagesize_kb=1440
set /A imagesize_b=%imagesize_kb*1024

if exist %binpath%\sample.img (fsutil file setzerodata offset=0 length=%imagesize_b% %binpath%\sample.img) else (fsutil file createnew %binpath%\sample.img %imagesize_b%)
mformat -i %binpath%\sample.img -f %imagesize_kb% ::/
mmd -i %binpath%\sample.img ::/EFI
mmd -i %binpath%\sample.img ::/EFI/BOOT
mcopy -i %binpath%\sample.img %binpath%\bootx64.efi ::/EFI/BOOT

echo Completed!