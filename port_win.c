#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>
#include <Windows.h>

void* dlmalloc(size_t length);
void* dlcalloc(size_t n_elements,size_t element_size);
void* dlrealloc(void* ptr,size_t length);
void dlfree(void* ptr);

typedef void* mspace;

mspace create_mspace(size_t capacity, int locked);
size_t destroy_mspace(mspace msp);
void* mspace_malloc(mspace msp, size_t bytes);
void mspace_free(mspace msp, void* mem);
void* mspace_memalign(mspace msp, size_t alignment, size_t bytes);

void* custom_mmap(size_t length)
{
	PVOID p=VirtualAlloc(NULL,length,MEM_COMMIT,PAGE_READWRITE);
	printf("[mmap] ptr: 0x%p, length: 0x%zx\n",p,length);
	return p==NULL?(void*)-1:p;
}

int custom_munmap(void* ptr,size_t length)
{
	BOOL b=VirtualFree(ptr,length,MEM_DECOMMIT);
	printf("[munmap] ptr: 0x%p, length: 0x%zx\n",ptr,length);
	return b?0:-1;
}

void* custom_direct_mmap(size_t length)
{
	// Return -1 because we don't have to support direct-mmap.
	return (void*)-1;
}

void custom_abort()
{
	puts("The dlmalloc library called abort!");
	ExitProcess(1);
}

int dprintf2(const char* src_fn,const int src_ln,const char* fmt,...)
{
	va_list arg_list;
	va_start(arg_list,fmt);
	char buff[512];
	int a=snprintf(buff,sizeof(buff),"[dlmalloc | %s@%d] ",src_fn,src_ln);
	int b=vsnprintf(&buff[a],sizeof(buff)-a,fmt,arg_list);
	va_end(arg_list);
	return (int)fwrite(buff,sizeof(char),a+b,stderr);
}

void init_lock(void** lock)
{
	*lock=(void*)SRWLOCK_INIT;
}

void final_lock(void* lock)
{
	;
}

void acquire_lock(void** lock)
{
	AcquireSRWLockExclusive((PSRWLOCK)lock);
}

void release_lock(void** lock)
{
	ReleaseSRWLockExclusive((PSRWLOCK)lock);
}

void test_dlmalloc()
{
	void* ptr=dlmalloc(0x401000);
	void *p1=dlmalloc(5),*p2=dlmalloc(0x1FFFFF);
	printf("ptr: 0x%p, p1: 0x%p, p2: 0x%p\n",ptr,p1,p2);
	dlfree(ptr);
	dlfree(p1);
	dlfree(p2);
}

void test_mspace()
{
	mspace ms=create_mspace(0,0);
	if(ms)
	{
		void* p1=mspace_malloc(ms,5555555);
		printf("mspace: 0x%p\n",ms);
		printf("p1: 0x%p\n",p1);
		mspace_free(ms,p1);
		destroy_mspace(ms);
	}
}

int main(int argc,char* argv[],char* envp[])
{
	if(argc>1)
	{
		if(strcmp(argv[1],"mspace")==0)
			test_mspace();
		else
			test_dlmalloc();
	}
	return 0;
}