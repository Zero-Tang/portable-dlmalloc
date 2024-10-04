#include <stdio.h>
#include <stdlib.h>
#include <Windows.h>

__declspec(dllimport) void* dlmalloc(size_t length);
__declspec(dllimport) void* dlcalloc(size_t n_elements,size_t element_size);
__declspec(dllimport) void* dlrealloc(void* ptr,size_t length);
__declspec(dllimport) void dlfree(void* ptr);

#define THREAD_COUNT		12
#define ALLOCATION_COUNT	10
#define ALLOCATION_SIZE		800

PVOID PointerArray[THREAD_COUNT*ALLOCATION_COUNT];
ULONG volatile Signal=0;

int __cdecl PointerCompare(const void* a,const void* b)
{
	ULONG_PTR p1=*(PULONG_PTR)a;
	ULONG_PTR p2=*(PULONG_PTR)b;
	if(p1>p2)return 1;
	if(p1<p2)return -1;
	return 0;
}

DWORD WINAPI AllocatorProcedure(IN LPVOID lpParameter)
{
	SIZE_T Index=(SIZE_T)lpParameter;
	int i;
	while(Signal==0)_mm_pause();
	for(i=0;i<ALLOCATION_COUNT;i++)
		PointerArray[Index*ALLOCATION_COUNT+i]=dlmalloc(ALLOCATION_SIZE);
	return 0;
}

int main(int argc,char* argv[],char* envp[])
{
	HANDLE ht[THREAD_COUNT];
	DWORD tid[THREAD_COUNT];
	int i,c=0;
	for(i=0;i<THREAD_COUNT;i++)ht[i]=CreateThread(NULL,0,AllocatorProcedure,(LPVOID)i,0,&tid[i]);
	Signal=1;
	WaitForMultipleObjects(THREAD_COUNT,ht,TRUE,INFINITE);
	for(i=0;i<THREAD_COUNT;i++)CloseHandle(ht[i]);
	qsort(PointerArray,THREAD_COUNT*ALLOCATION_COUNT,sizeof(void*),PointerCompare);
	for(i=0;i<THREAD_COUNT*ALLOCATION_COUNT;i++)dlfree(PointerArray[i]);
	for(i=1;i<THREAD_COUNT*ALLOCATION_COUNT;i++)
	{
		ULONG_PTR Diff=(ULONG_PTR)PointerArray[i]-(ULONG_PTR)PointerArray[i-1];
		if(Diff<ALLOCATION_SIZE)c++;
	}
	puts(c?"Allocation conflict detected!":"No allocation conflict detected...");
}