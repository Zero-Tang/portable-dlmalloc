@echo off
set ddkpath=C:\WinDDK\7600.16385.1
set path=%ddkpath%\bin\x86\amd64;%ddkpath%\bin\x86;%path%
set incpath=%ddkpath%\inc
set libpath=%ddkpath%\lib
set binpath=.\bin\compfre_win7x64
set objpath=.\bin\compfre_win7x64\Intermediate

if not exist %objpath% (mkdir %objpath%)

echo Compiling dlmalloc...
cl .\malloc.c /I"%incpath%\crt" /I"%incpath%\api" /D"WIN32" /D"USE_DL_PREFIX" /D"NO_MALLOC_STATS=1" /D"USE_LOCKS=1" /D"MSPACES" /Zi /nologo /W3 /WX /O2 /Zc:wchar_t /FAcs /Fa"%objpath%\malloc.cod" /Fo"%objpath%\malloc.obj" /Fd"%objpath%\vc90.pdb" /GS- /Gy /TC /c /errorReport:queue

link "%objpath%\malloc.obj" /LIBPATH:"%libpath%\win7\amd64" /LIBPATH:"%libpath%\Crt\amd64" /NODEFAULTLIB "kernel32.lib" "msvcrt.lib" /NOLOGO /INCREMENTAL:NO /OPT:ICF /OPT:REF /DEBUG /DEF:"export.def" /PDB:"%binpath%\dlmalloc.pdb" /OUT:"%binpath%\dlmalloc.dll" /NOENTRY /SUBSYSTEM:WINDOWS /DLL /Machine:X64 /ERRORREPORT:QUEUE

echo Compiling Sample...
cl .\sample.c /I"%incpath%\crt" /I"%incpath%\api" /Zi /nologo /W3 /WX /O2 /Zc:wchar_t /MD /FAcs /Fa"%objpath%\sample.cod" /Fo"%objpath%\sample.obj" /Fd"%objpath%\vc90.pdb" /GS- /Gy /TC /c /errorReport:queue

link "%objpath%\sample.obj" "%binpath%\dlmalloc.lib" /LIBPATH:"%libpath%\win7\amd64" /LIBPATH:"%libpath%\Crt\amd64" /NOLOGO /INCREMENTAL:NO /OPT:ICF /OPT:REF /DEBUG /PDB:"%binpath%\sample.pdb" /OUT:"%binpath%\sample.exe" /SUBSYSTEM:CONSOLE /Machine:X64 /ERRORREPORT:QUEUE

echo Completed!