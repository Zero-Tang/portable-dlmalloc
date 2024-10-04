@echo off
set ddkpath=V:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.38.33130
set path=%ddkpath%\bin\Hostx64\x64;V:\Program Files\Windows Kits\10\bin\10.0.26100.0\x64;%path%
set incpath=V:\Program Files\Windows Kits\10\Include\10.0.26100.0
set libpath=V:\Program Files\Windows Kits\10\Lib
set binpath=.\bin\compchk_win11x64
set objpath=.\bin\compchk_win11x64\Intermediate

if not exist %objpath% (mkdir %objpath%)

echo Compiling dlmalloc...
cl malloc.c /I"%incpath%\ucrt" /I"%ddkpath%\include" /D"PORTABLE" /D"USE_DL_PREFIX" /D"NO_MALLOC_STATS=0" /D"USE_LOCKS=2" /Zi /nologo /W3 /WX /Oi /Od /Zc:wchar_t /FAcs /Fa"%objpath%\malloc.cod" /Fo"%objpath%\malloc.obj" /Fd"%objpath%\vc140.pdb" /GS- /std:c17 /Qspectre /TC /c /errorReport:queue

lib "%objpath%\malloc.obj" /NOLOGO /OUT:"%binpath%\dlmalloc.lib" /Machine:X64 /ERRORREPORT:QUEUE

echo Compiling Sample...
cl .\port_win.c /I"%incpath%\ucrt" /I"%incpath%\shared" /I"%incpath%\um" /I"%ddkpath%\include" /Zi /nologo /W3 /WX /Od /Zc:wchar_t /MT /FAcs /Fa"%objpath%\sample.cod" /Fo"%objpath%\sample.obj" /Fd"%objpath%\vc90.pdb" /GS- /Gy /TC /c /errorReport:queue

link "%objpath%\sample.obj" "%binpath%\dlmalloc.lib" /LIBPATH:"%libpath%\10.0.26100.0\um\x64" /LIBPATH:"%libpath%\10.0.26100.0\ucrt\x64" /LIBPATH:"%ddkpath%\lib\x64" /NOLOGO /INCREMENTAL:NO /DEBUG /PDB:"%binpath%\sample.pdb" /OUT:"%binpath%\sample.exe" /SUBSYSTEM:CONSOLE /Machine:X64 /ERRORREPORT:QUEUE

echo Completed!