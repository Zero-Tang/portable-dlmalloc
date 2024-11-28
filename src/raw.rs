// This module defines C FFI definitions of dlmalloc library.
// Use this library only if you understand the safety.

use core::ffi::c_void;

pub type DLInspectHandler=unsafe extern "C" fn (start:*mut c_void,end:*mut c_void,used_bytes:usize,callback_arg:*mut c_void);

pub const M_TRIM_THRESHOLD:i32=-1;
pub const M_GRANULARITY:i32=-2;
pub const M_MMAP_THRESHOLD:i32=-3;

#[repr(C)] pub struct MallInfo
{
	/// non-mapped spaced allcated from system
	pub arena:usize,
	/// number of free chunks
	pub ordblks:usize,
	/// always 0
	pub smblks:usize,
	/// always 0
	pub hblks:usize,
	/// space in mmapped regions
	pub hblkhd:usize,
	/// maximum total allocated space
	pub usmblks:usize,
	/// always 0
	pub fsmblks:usize,
	/// total allocated space
	pub uordblks:usize,
	/// total free space
	pub fordblks:usize,
	/// releaseable (via malloc_trim space)
	pub keepcost:usize
}

extern "C"
{
	/// `malloc(size_t n)`
	///
	/// Returns a pointer to a newly allocated chunk of at least n bytes, or
	/// null if no space is available, in which case errno is set to ENOMEM
	/// on ANSI C systems.
	///
	/// If n is zero, malloc returns a minimum-sized chunk. (The minimum
	/// size is 16 bytes on most 32bit systems, and 32 bytes on 64bit
	/// systems.)  Note that size_t is an unsigned type, so calls with
	/// arguments that would be negative if signed are interpreted as
	/// requests for huge amounts of space, which will often fail. The
	/// maximum supported value of n differs across systems, but is in all
	/// cases less than the maximum representable value of a size_t.
	pub fn dlmalloc(n:usize)->*mut c_void;
	/// `dlfree(void* p)`
	/// 
	/// Releases the chunk of memory pointed to by p, that had been previously
	/// allocated using malloc or a related routine such as realloc.
	/// It has no effect if p is null. If p was not malloced or already
	/// freed, free(p) will by default cause the current program to abort.
	pub fn dlfree(p:*mut c_void);
	/// `calloc(size_t n_elements, size_t element_size);`
	/// 
	/// Returns a pointer to n_elements * element_size bytes, with all locations
	/// set to zero.
	pub fn dlcalloc(n_elements:usize,element_size:usize)->*mut c_void;
	/// `realloc(void* p, size_t n)`
	/// 
	/// Returns a pointer to a chunk of size n that contains the same data
	/// as does chunk p up to the minimum of (n, p's size) bytes, or null
	/// if no space is available.
	///
	/// The returned pointer may or may not be the same as p. The algorithm
	/// prefers extending p in most cases when possible, otherwise it
	/// employs the equivalent of a malloc-copy-free sequence.
	///
	/// If p is null, realloc is equivalent to malloc.
	///
	/// If space is not available, realloc returns null, errno is set (if on
	/// ANSI) and p is NOT freed.
	///
	/// if n is for fewer bytes than already held by p, the newly unused
	/// space is lopped off and freed if possible.  realloc with a size
	/// argument of zero (re)allocates a minimum-sized chunk.
	///
	/// The old unix realloc convention of allowing the last-free'd chunk
	/// to be used as an argument to realloc is not supported.
	pub fn dlrealloc(p:*mut c_void,n:usize)->*mut c_void;
	/// `realloc_in_place(void*  p, size_t n)`
	/// 
	/// Resizes the space allocated for p to size n, only if this can be
	/// done without moving p (i.e., only if there is adjacent space
	/// available if n is greater than p's current allocated size, or n is
	/// less than or equal to p's size). This may be used instead of plain
	/// realloc if an alternative allocation strategy is needed upon failure
	/// to expand space; for example, reallocation of a buffer that must be
	/// memory-aligned or cleared. You can use realloc_in_place to trigger
	/// these alternatives only when needed.
	///
	/// Returns p if successful; otherwise null.
	pub fn dlrealloc_in_place(p:*mut c_void,n:usize)->*mut c_void;
	/// `memalign(size_t alignment, size_t n);`
	/// 
	/// Returns a pointer to a newly allocated chunk of n bytes, aligned
	/// in accord with the alignment argument.
	///
	/// The alignment argument should be a power of two. If the argument is
	/// not a power of two, the nearest greater power is used.
	/// 8-byte alignment is guaranteed by normal malloc calls, so don't
	/// bother calling memalign with an argument of 8 or less.
	///
	/// Overreliance on memalign is a sure way to fragment space.
	pub fn dlmemalign(alignment:usize,n:usize)->*mut c_void;
	/// `int posix_memalign(void** pp, size_t alignment, size_t n);`
	/// 
	/// Allocates a chunk of n bytes, aligned in accord with the alignment
	/// argument. Differs from memalign only in that it
	/// 1. assigns the allocated memory to *pp rather than returning it,
	/// 2. fails and returns EINVAL if the alignment is not a power of two
	/// 3. fails and returns ENOMEM if memory cannot be allocated.
	pub fn dlposix_memalign(pp:*mut *mut c_void,alignment:usize,n:usize)->i32;
	/// `valloc(size_t n);`
	/// 
	/// Equivalent to memalign(pagesize, n), where pagesize is the page
	/// size of the system. If the pagesize is unknown, 4096 is used.
	pub fn dlvalloc(n:usize)->*mut c_void;
	/// `mallopt(int parameter_number, int parameter_value)`
	/// 
	/// Sets tunable parameters The format is to provide a
	/// (parameter-number, parameter-value) pair.  mallopt then sets the
	/// corresponding parameter to the argument value if it can (i.e., so
	/// long as the value is meaningful), and returns 1 if successful else
	/// 0.  To workaround the fact that mallopt is specified to use int,
	/// not size_t parameters, the value -1 is specially treated as the
	/// maximum unsigned size_t value.
	///
	/// SVID/XPG/ANSI defines four standard param numbers for mallopt,
	/// normally defined in malloc.h.  None of these are use in this malloc,
	/// so setting them has no effect. But this malloc also supports other
	/// options in mallopt. See below for details.  Briefly, supported
	/// parameters are as follows (listed defaults are for "typical"
	/// configurations).
	///
	/// | Symbol           | param #  | default     |  allowed param values
	/// |---|---|---|---|
	/// | M_TRIM_THRESHOLD |    -1    | 2x1024x1024 |   any   (-1 disables)
	/// | M_GRANULARITY    |    -2    |  page size  |   any power of 2 >= page size
	/// | M_MMAP_THRESHOLD |    -3    | 2x1024x1024 |   any   (or 0 if no MMAP support)
	pub fn dlmallopt(parameter_number:i32,parameter_value:i32)->i32;
	/// `malloc_footprint();`
	/// 
	/// Returns the number of bytes obtained from the system.  The total
	/// number of bytes allocated by malloc, realloc etc., is less than this
	/// value. Unlike mallinfo, this function returns only a precomputed
	/// result, so can be called frequently to monitor memory consumption.
	/// Even if locks are otherwise defined, this function does not use them,
	/// so results might not be up to date.
	pub fn dlmalloc_footprint()->usize;
	/// `malloc_max_footprint();`
	/// 
	/// Returns the maximum number of bytes obtained from the system. This
	/// value will be greater than current footprint if deallocated space
	/// has been reclaimed by the system. The peak number of bytes allocated
	/// by malloc, realloc etc., is less than this value. Unlike mallinfo,
	/// this function returns only a precomputed result, so can be called
	/// frequently to monitor memory consumption.  Even if locks are
	/// otherwise defined, this function does not use them, so results might
	/// not be up to date.`
	pub fn dlmalloc_max_footprint()->usize;
	/// `malloc_footprint_limit();`
	/// 
	/// Returns the number of bytes that the heap is allowed to obtain from
	/// the system, returning the last value returned by
	/// malloc_set_footprint_limit, or the maximum size_t value if
	/// never set. The returned value reflects a permission. There is no
	/// guarantee that this number of bytes can actually be obtained from
	/// the system.
	pub fn dlmalloc_footprint_limit()->usize;
	/// `malloc_set_footprint_limit();`
	/// 
	/// Sets the maximum number of bytes to obtain from the system, causing
	/// failure returns from malloc and related functions upon attempts to
	/// exceed this value. The argument value may be subject to page
	/// rounding to an enforceable limit; this actual value is returned.
	/// Using an argument of the maximum possible size_t effectively
	/// disables checks. If the argument is less than or equal to the
	/// current malloc_footprint, then all future allocations that require
	/// additional system memory will fail. However, invocation cannot
	/// retroactively deallocate existing used memory.
	pub fn dlmalloc_set_footprint_limit(bytes:usize)->usize;
	/// `malloc_inspect_all(void(*handler)(void *start,void *end,size_t used_bytes,void*  callback_arg),void*  arg);`
	/// 
	/// Traverses the heap and calls the given handler for each managed
	/// region, skipping all bytes that are (or may be) used for bookkeeping
	/// purposes.  Traversal does not include include chunks that have been
	/// directly memory mapped. Each reported region begins at the start
	/// address, and continues up to but not including the end address.  The
	/// first used_bytes of the region contain allocated data. If
	/// used_bytes is zero, the region is unallocated. The handler is
	/// invoked with the given callback argument. If locks are defined, they
	/// are held during the entire traversal. It is a bad idea to invoke
	/// other malloc functions from within the handler.
	///
	/// For example, to count the number of in-use chunks with size greater
	/// than 1000, you could write:
	/// ```C
	/// static int count = 0;
	/// void count_chunks(void*  start, void* end, size_t used, void* arg)
	/// {
	///     if (used >= 1000) ++count;
	/// }
	/// ```
	/// then:
	/// ```C
	/// malloc_inspect_all(count_chunks, NULL);
	/// ```
	///
	/// malloc_inspect_all is compiled only if MALLOC_INSPECT_ALL is defined.
	pub fn dlmalloc_inspect_all(handler:DLInspectHandler,arg:*mut c_void);
	/// `mallinfo()`
	/// 
	/// Returns (by copy) a struct containing various summary statistics:
	///
	/// -  arena:     current total non-mmapped bytes allocated from system
	/// -  ordblks:   the number of free chunks
	/// -  smblks:    always zero.
	/// -  hblks:     current number of mmapped regions
	/// -  hblkhd:    total bytes held in mmapped regions
	/// -  usmblks:   the maximum total allocated space. This will be greater than current total if trimming has occurred.
	/// -  fsmblks:   always zero
	/// -  uordblks:  current total allocated space (normal or mmapped)
	/// -  fordblks:  total free space
	/// -  keepcost:  the maximum number of bytes that could ideally be released back to system via malloc_trim. ("ideally" means that it ignores page restrictions etc.)
	///
	/// Because these fields are ints, but internal bookkeeping may
	/// be kept as longs, the reported values may wrap around zero and
	/// thus be inaccurate.
	pub fn dlmallinfo()->MallInfo;
	/// `independent_calloc(size_t n_elements, size_t element_size, void*  chunks[]);`
	///
	/// independent_calloc is similar to calloc, but instead of returning a
	/// single cleared space, it returns an array of pointers to n_elements
	/// independent elements that can hold contents of size elem_size, each
	/// of which starts out cleared, and can be independently freed,
	/// realloc'ed etc. The elements are guaranteed to be adjacently
	/// allocated (this is not guaranteed to occur with multiple callocs or
	/// mallocs), which may also improve cache locality in some
	/// applications.
	///
	/// The "chunks" argument is optional (i.e., may be null, which is
	/// probably the most typical usage). If it is null, the returned array
	/// is itself dynamically allocated and should also be freed when it is
	/// no longer needed. Otherwise, the chunks array must be of at least
	/// n_elements in length. It is filled in with the pointers to the
	/// chunks.
	///
	/// In either case, independent_calloc returns this pointer array, or
	/// null if the allocation failed.  If n_elements is zero and "chunks"
	/// is null, it returns a chunk representing an array with zero elements
	/// (which should be freed if not wanted).
	///
	/// Each element must be freed when it is no longer needed. This can be
	/// done all at once using bulk_free.
	///
	/// independent_calloc simplifies and speeds up implementations of many
	/// kinds of pools.  It may also be useful when constructing large data
	/// structures that initially have a fixed number of fixed-sized nodes,
	/// but the number is not known at compile time, and some of the nodes
	/// may later need to be freed. For example:
	/// ```C
	/// struct Node { int item; struct Node*  next; };
	///
	/// struct Node*  build_list() {
	///     struct Node**  pool;
	///     int n = read_number_of_nodes_needed();
	///     if (n <= 0) return 0;
	///     pool = (struct Node**)(independent_calloc(n, sizeof(struct Node), 0);
	///     if (pool == 0) die();
	///     // organize into a linked list...
	///     struct Node*  first = pool[0];
	///     for (i = 0; i < n-1; ++i)
	///       pool[i]->next = pool[i+1];
	///     free(pool);     // Can now free the array (or not, if it is needed later)
	///     return first;
	/// }
	/// ```
	pub fn dlindependent_calloc(n_elements:usize,element_size:usize,chunks:*mut *mut c_void)->*mut *mut c_void;
	/// `independent_comalloc(size_t n_elements, size_t sizes[], void*  chunks[]);`
	///
	/// independent_comalloc allocates, all at once, a set of n_elements
	/// chunks with sizes indicated in the "sizes" array.    It returns
	/// an array of pointers to these elements, each of which can be
	/// independently freed, realloc'ed etc. The elements are guaranteed to
	/// be adjacently allocated (this is not guaranteed to occur with
	/// multiple callocs or mallocs), which may also improve cache locality
	/// in some applications.
	///
	/// The "chunks" argument is optional (i.e., may be null). If it is null
	/// the returned array is itself dynamically allocated and should also
	/// be freed when it is no longer needed. Otherwise, the chunks array
	/// must be of at least n_elements in length. It is filled in with the
	/// pointers to the chunks.
	///
	/// In either case, independent_comalloc returns this pointer array, or
	/// null if the allocation failed.  If n_elements is zero and chunks is
	/// null, it returns a chunk representing an array with zero elements
	/// (which should be freed if not wanted).
	///
	/// Each element must be freed when it is no longer needed. This can be
	/// done all at once using bulk_free.
	///
	/// independent_comallac differs from independent_calloc in that each
	/// element may have a different size, and also that it does not
	/// automatically clear elements.
	///
	/// independent_comalloc can be used to speed up allocation in cases
	/// where several structs or objects must always be allocated at the
	/// same time.  For example:
	///
	/// ```C
	/// struct Head { ... }
	/// struct Foot { ... }
	///
	/// void send_message(char*  msg) {
	///     int msglen = strlen(msg);
	///     size_t sizes[3] = { sizeof(struct Head), msglen, sizeof(struct Foot) };
	///     void*  chunks[3];
	///     if (independent_comalloc(3, sizes, chunks) == 0)
	///       die();
	///     struct Head*  head = (struct Head*)(chunks[0]);
	///     char*         body = (char*)(chunks[1]);
	///     struct Foot*  foot = (struct Foot*)(chunks[2]);
	///     // ...
	/// }
	/// ```
	/// In general though, independent_comalloc is worth using only for
	/// larger values of n_elements. For small values, you probably won't
	/// detect enough difference from series of malloc calls to bother.
	///
	/// Overuse of independent_comalloc can increase overall memory usage,
	/// since it cannot reuse existing noncontiguous small chunks that
	/// might be available for some of the elements.
	pub fn dlindependent_comalloc(n_element:usize,sizes:*const usize,chunks:*mut *mut c_void)->*mut *mut c_void;
	/// `bulk_free(void*  array[], size_t n_elements)`
	/// 
	/// Frees and clears (sets to null) each non-null pointer in the given
	/// array.  This is likely to be faster than freeing them one-by-one.
	/// If footers are used, pointers that have been allocated in different
	/// mspaces are not freed or cleared, and the count of all such pointers
	/// is returned.  For large arrays of pointers with poor locality, it
	/// may be worthwhile to sort this array before calling bulk_free.
	pub fn dlbulk_free(array:*mut *mut c_void,n_elements:usize)->usize;
	/// `pvalloc(size_t n);`
	/// Equivalent to valloc(minimum-page-that-holds(n)), that is,
	/// round up n to nearest pagesize.
	pub fn dlpvalloc(n:usize)->*mut c_void;
	/// `malloc_trim(size_t pad);`
	///
	/// If possible, gives memory back to the system (via negative arguments
	/// to sbrk) if there is unused memory at the `high' end of the malloc
	/// pool or in unused MMAP segments. You can call this after freeing
	/// large blocks of memory to potentially reduce the system-level memory
	/// requirements of a program. However, it cannot guarantee to reduce
	/// memory. Under some allocation patterns, some large free blocks of
	/// memory will be locked between two used chunks, so they cannot be
	/// given back to the system.
	///
	/// The `pad' argument to malloc_trim represents the amount of free
	/// trailing space to leave untrimmed. If this argument is zero, only
	/// the minimum amount of memory to maintain internal data structures
	/// will be left. Non-zero arguments can be supplied to maintain enough
	/// trailing space to service future expected allocations without having
	/// to re-obtain memory from the system.
	///
	/// Malloc_trim returns 1 if it actually released any memory, else 0.
	pub fn dlmalloc_trim(pad:usize)->i32;
	/// `malloc_usable_size(void*  p);`
	///
	/// Returns the number of bytes you can actually use in
	/// an allocated chunk, which may be more than you requested (although
	/// often not) due to alignment and minimum size constraints.
	/// You can use this many bytes without worrying about
	/// overwriting other allocated objects. This is not a particularly great
	/// programming practice. malloc_usable_size can be more useful in
	/// debugging and assertions, for example:
	/// ```C
	/// p = malloc(n);
	/// assert(malloc_usable_size(p) >= 256);
	/// ```
	pub fn dlmalloc_usable_size(p:*mut c_void)->usize;
	/// `create_mspace` creates and returns a new independent space with the
	/// given initial capacity, or, if 0, the default granularity size.  It
	/// returns null if there is no system memory available to create the
	/// space.  If argument locked is non-zero, the space uses a separate
	/// lock to control access. The capacity of the space will grow
	/// dynamically as needed to service mspace_malloc requests.  You can
	/// control the sizes of incremental increases of this space by
	/// compiling with a different DEFAULT_GRANULARITY or dynamically
	/// setting with mallopt(M_GRANULARITY, value).
	pub fn create_mspace(capacity:usize,locked:i32)->*mut c_void;
	/// `destroy_mspace` destroys the given space, and attempts to return all
	/// 
	/// of its memory back to the system, returning the total number of
	/// bytes freed. After destruction, the results of access to all memory
	/// used by the space become undefined.
	pub fn destroy_mspace(msp:*mut c_void)->usize;
	/// `create_mspace_with_base`` uses the memory supplied as the initial base
	/// of a new mspace. Part (less than 128*sizeof(size_t) bytes) of this
	/// space is used for bookkeeping, so the capacity must be at least this
	/// large. (Otherwise 0 is returned.) When this initial space is
	/// exhausted, additional memory will be obtained from the system.
	/// Destroying this space will deallocate all additionally allocated
	/// space (if possible) but not the initial base.
	pub fn create_mspace_with_base(base:*mut c_void,capacity:usize,locked:i32)->*mut c_void;
	/// `mspace_track_large_chunks` controls whether requests for large chunks
	/// are allocated in their own untracked mmapped regions, separate from
	/// others in this mspace. By default large chunks are not tracked,
	/// which reduces fragmentation. However, such chunks are not
	/// necessarily released to the system upon destroy_mspace.  Enabling
	/// tracking by setting to true may increase fragmentation, but avoids
	/// leakage when relying on destroy_mspace to release all memory
	/// allocated using this space.  The function returns the previous
	/// setting.
	pub fn mspace_track_large_chunks(msp:*mut c_void,enable:i32)->i32;
	/// `mspace_malloc` behaves as malloc, but operates within the given space.
	pub fn mspace_malloc(msp:*mut c_void,bytes:usize)->*mut c_void;
	/// mspace_free behaves as free, but operates within
	/// the given space.
	///
	/// If compiled with FOOTERS==1, mspace_free is not actually needed.
	/// free may be called instead of mspace_free because freed chunks from
	/// any space are handled by their originating spaces.
	pub fn mspace_free(msp:*mut c_void,mem:*mut c_void);
	/// `mspace_realloc` behaves as realloc, but operates within
	/// the given space.
	///
	/// If compiled with FOOTERS==1, mspace_realloc is not actually
	/// needed.  realloc may be called instead of mspace_realloc because
	/// realloced chunks from any space are handled by their originating
	/// spaces.
	pub fn mspace_realloc(msp:*mut c_void,mem:*mut c_void,newsize:usize)->*mut c_void;
	/// `mspace_calloc` behaves as calloc, but operates within the given space.
	pub fn mspace_calloc(msp:*mut c_void,n_elements:usize,elem_size:usize)->*mut c_void;
	/// `mspace_memalign` behaves as memalign, but operates within the given space.
	pub fn mspace_memalign(msp:*mut c_void,alignment:usize,bytes:usize)->*mut c_void;
	/// `mspace_independent_calloc` behaves as independent_calloc, but operates within the given space.
	pub fn mspace_independent_calloc(msp:*mut c_void,n_elements:usize,elem_size:usize,chunks:*mut *mut c_void)->*mut *mut c_void;
	/// `mspace_independent_comalloc` behaves as independent_comalloc, but operates within the given space.
	pub fn mspace_independent_comlloc(msp:*mut c_void,n_elements:usize,sizes:*const usize,chunks:*mut *mut c_void)->*mut *mut c_void;
	/// `mspace_footprint()` returns the number of bytes obtained from the system for this space.
	pub fn mspace_footprint(msp:*mut c_void)->usize;
	/// `mspace_max_footprint()` returns the peak number of bytes obtained from the system for this space.
	pub fn mspace_max_footprint(msp:*mut c_void)->usize;
	/// `mspace_mallinfo` behaves as mallinfo, but reports properties of the given space.
	pub fn mspace_mallinfo(msp:*mut c_void)->MallInfo;
	/// `mspace_trim` behaves as malloc_trim, but operates within the given space.
	pub fn mspace_trim(msp:*mut c_void,pad:usize)->i32;
}