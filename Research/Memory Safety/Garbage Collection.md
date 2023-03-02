https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)

Wikipedia (2023)  "Garbage collection (computer science)" [Online] Available: https://en.wikipedia.org/wiki/Garbage_collection_(computer_science) [Accessed 23 December 2022]

Hertz, M. Berger, D. E. (2005) "Quantifying the Performance of Garbage Collection vs Explicit Memory Management". [Online] Available: https://people.cs.umass.edu/~emery/pubs/gcvsmalloc.pdf [Accessed 2 March 2023]

A form of automatic memory management which utilises a 'garbage collector' in order to free memory which was previously allocated but now no longer referenced - this memory is known as 'garbage'. Garbage collection was invented in 1959 by John McCarthy, an American computer scientist to simplfiy manual memory management in the Lisp programming language.

This method of memory management relieves the developer the duty of performing manual memory management. It is possible that garbage collection utilises a large volume of processing time which will then affect performance.

Present in various programming languages
+ RPL
+ Java
+ C#
+ D
+ Go
+ Most scripting languages

### Advantages
+ Programmer no longer needs to perform manual de-allocation which avoids certain errors
	+ Dangling pointers - pointers to freed memory, similar to OOB
	+ Double free bugs - trying to free memory which is already freed (which may already be re-allocated)
	+ Certain memory leaks - program fails to free mem by unreachable objects, can lead to memory exhaustion

### Disadvantages
(check) GC needs 5 times memory to compensate for overhead to decide which mem to free while also performing as fast as the same program which uses ideal memory management.

### Strategies/Types
#### Tracing
+ Most common type 
+ Trying to determine which objects should be collected by tracing what objects are reachable by a chain of references from specific root objects - the rest are considered garbage and collected.
+ Various algorithms can be used which vary in complexity and performance

#### Reference Counting
+ Each object has a count of the number of references to it with Garbage indicated as having 0 references.
+ An objects reference count is incremented when a ref is created and decremented when a ref is destroyed. So when the count is 0, the objects memory is reclaimed.
+ Ref counting guarantees that objects are destroyed as soon as their last reference is destroyed.
+ Usually only accesses memory in CPU caches, in objects to be freed or directly pointed to by the former so tends to not have significant side effects on CPU cache and VRAM operation.


----
https://dlang.org/spec/garbage.html

D Language Foundation (2022) "Language Reference: 28. Garbage Collection" [Online] Available: https://dlang.org/spec/garbage.html [Accessed 23 December 2022]

Dlang garbage collector spec

D
+ Systems programming language w/ garbage collection
+ Simply allocate as needed, every so often the GC will return unused mem to the pool of available mem
+ D also provides mechanisms to write code where GC is not involved

Documentation claims:
+ GC programs are often faster
+ GCs can't suffer from memory leaks thus more long term stability
+ GC programs have fewer difficult pointer bugs due to no dangling references to freed mem. No code to expliciltly manage memory thus no bugs in such code.
+ GC programs are faster to develop and debug due to no need for developing, debugging, testing or maintaining explicit de-allocation code.

Though there are downsides:
+ Not always obvious when GC allocates memory, this can trigger a collection which can cause the program to pause unexpectedly
+ Time taken for collection is not bounded, while typically quick this is not always guaranteed.
+ Typically all threads aside from the collector thread must be halted while collection is in progress
+ GCs can keep around some memory that an explicit deallocator would not
+ GC should be implemented as a basic OS kernel service but is not so GC programs must carry the GC implementation. This may be a shared library but nevertheless is still overhead. 
---

Microsoft (2009) "Back to Basics: Reference Counting Garbage Collection". [Online] Available: https://learn.microsoft.com/en-gb/archive/blogs/abhinaba/back-to-basics-reference-counting-garbage-collection [Accessed 2 March 2023]

https://learn.microsoft.com/en-gb/archive/blogs/abhinaba/back-to-basics-reference-counting-garbage-collection

+ Simple, counts the number of reference(s) to a memory block/object from other blocks
+ When mem is created or a ref to the object is assigned, the refcount is increased
+ When a pointer is dealloacted/'assigned away', the refcount is decreased
+ If the refcount reaches 0, there are no pointer references. The memory is considered unreachable and should be reclaimed as garbage

This mechanism works by counting the number of references to a block of memory or object from other blocks. A reference count holds the number of references. The count is increased when memory or a reference to the object is created and decreased when a pointer to the memory is de-allocated or destroyed. Upon the count reaching 0, it is clear that there are no pointer references thus the memory is considered unreachable and should be reclaimed as garbage.

Advantages 
+ Easy to implement
+ Object reclaimed as soon as it becomes garbage
+ If objects support destructs/finalisers then sys resources can be return to OS faster
+ Refcount ensures garbage collection is distributed over the whole period of execution, in interactive systems there will be no system freezes

Disadvantages
+ Adds signficant bloat to code, as each assignment sees a call to update the refcount
+ In a multithreaded system, refcount can be a major issue as locks need to be used to update the refcount
+ Re-using even optimised atomic operations are costly when repeatedly used
+ Header space cost eats into the system, if they are used in small objects then it causes significant overhead

