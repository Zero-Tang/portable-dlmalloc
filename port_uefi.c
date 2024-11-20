#include <Uefi.h>
#include <Library/UefiLib.h>
#include <Library/BaseMemoryLib.h>
#include <Library/MemoryAllocationLib.h>
#include <Library/PrintLib.h>

EFI_STATUS EFIAPI UefiBootServicesTableLibConstructor(IN EFI_HANDLE ImageHandle,IN EFI_SYSTEM_TABLE *SystemTable);
EFI_STATUS EFIAPI UefiRuntimeServicesTableLibConstructor(IN EFI_HANDLE ImageHandle,IN EFI_SYSTEM_TABLE *SystemTable);
EFI_STATUS EFIAPI UefiLibConstructor(IN EFI_HANDLE ImageHandle,IN EFI_SYSTEM_TABLE *SystemTable);

EFI_SYSTEM_TABLE *gST;
EFI_BOOT_SERVICES *gBS;
EFI_RUNTIME_SERVICES *gRT;
EFI_SIMPLE_TEXT_INPUT_PROTOCOL *StdIn;
EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *StdOut;

CHAR8 gEfiCallerBaseName[]="dlmalloc-uefi-sample";

void* dlmalloc(size_t length);
void* dlcalloc(size_t n_elements,size_t element_size);
void* dlrealloc(void* ptr,size_t length);
void dlfree(void* ptr);

void custom_abort()
{
	StdOut->OutputString(StdOut,L"The dlmalloc library called abort!\r\n");
	gRT->ResetSystem(EfiResetShutdown,EFI_SUCCESS,0,NULL);
}

void* custom_mmap(size_t length)
{
	void* p=AllocateAlignedRuntimePages(length>>EFI_PAGE_SHIFT,0x200000);
	Print(L"[mmap] ptr: 0x%p, size=0x%X\n",p,length);
	return p?p:(void*)-1;
}

int custom_munmap(void* ptr,size_t length)
{
	FreePages(ptr,length>>EFI_PAGE_SHIFT);
	Print(L"[munmap] ptr: 0x%p, size=0x%X\n",ptr,length);
	return 0;
}

void* custom_direct_mmap(size_t length)
{
	// Return -1 because we don't have to support direct-mmap.
	return (void*)-1;
}

#define LOCK_IS_FREE	(void*)0
#define LOCK_IN_USE		(void*)1

void init_lock(void* *lock)
{
	*lock=NULL;
}

void final_lock(void* *lock)
{
	;
}

void acquire_lock(void* *lock)
{
	// In real machines, "pause" instruction has less power consumption than "nop".
	// In virtual machines, "pause" instruction can hint hypervisor a spinlock so that the hypervisor will schedule out the vCPU.
	while(_InterlockedCompareExchangePointer(lock,LOCK_IN_USE,LOCK_IS_FREE))_mm_pause();
}

void release_lock(void* *lock)
{
	_InterlockedExchangePointer(lock,LOCK_IS_FREE);
}

void BlockUntilKeyStroke(IN CHAR16 Unicode)
{
	EFI_INPUT_KEY InKey;
	do
	{
		UINTN fi=0;
		gBS->WaitForEvent(1,&StdIn->WaitForKey,&fi);
		StdIn->ReadKeyStroke(StdIn,&InKey);
	}while(InKey.UnicodeChar!=Unicode);
}

void SetConsoleModeToMaximumRows()
{
	UINTN MaxHgt=0,MaxWdh=0,OptIndex;
	for(UINTN i=0;i<StdOut->Mode->MaxMode;i++)
	{
		UINTN Col,Row;
		EFI_STATUS st=StdOut->QueryMode(StdOut,i,&Col,&Row);
		if(st==EFI_SUCCESS)
		{
			if(Row>=MaxHgt)
			{
				if(Col>MaxWdh)
				{
					OptIndex=i;
					MaxHgt=Row;
					MaxWdh=Col;
				}
			}
		}
	}
	StdOut->SetMode(StdOut,OptIndex);
	StdOut->ClearScreen(StdOut);
}

EFI_STATUS EfiEntry(IN EFI_HANDLE ImageHandle,IN EFI_SYSTEM_TABLE *SystemTable)
{
	// Initialize EDK2 Library.
	UefiBootServicesTableLibConstructor(ImageHandle,SystemTable);
	UefiRuntimeServicesTableLibConstructor(ImageHandle,SystemTable);
	// Initialize Console.
	StdIn=SystemTable->ConIn;
	StdOut=SystemTable->ConOut;
	SetConsoleModeToMaximumRows();
	// Initialization done.
	void* ptr=dlmalloc(0x20000);
	void *p1=dlmalloc(5),*p2=dlmalloc(0x1FFFFF);
	Print(L"ptr: 0x%p, p1: 0x%p, p2: 0x%p\n",ptr,p1,p2);
	dlfree(ptr);
	dlfree(p1);
	dlfree(p2);
	// Ending...
	StdOut->OutputString(StdOut,L"Press Enter key to shutdown...\r\n");
	BlockUntilKeyStroke(L'\r');
	gRT->ResetSystem(EfiResetShutdown,EFI_SUCCESS,0,NULL);
	return EFI_SUCCESS;
}