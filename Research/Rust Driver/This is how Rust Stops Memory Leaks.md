tags: #rust 

["This is How Rust Stops Memory Leaks](https://www.youtube.com/watch?v=DJdUjjOmyx8)

+ C prone to leaking memory
+ Leaked memory wastes available resources and eventually degrade overall system performance
+ Garbage Collection: Runtime concept that checks segs of memory that are no longer accessible and frees them, the trade-off here is performance as the language needs to compile a runtime into the executable. Can be bulky and inefficient (Golang and Java)
+ Rust: Mem mgmt at compile time, which retains performance and doesn't need a runtime/garbage collector
+ Rust [Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
	+ System with a set of rules to manage mem
	+ Rule violations means code won't be compiled
	+ Each value has an owner, values can only have 1 owner at any time, if the owner goes out of scope then the value is dropped (Almost cleaning up memory as it goes)
+ Rust Borrowing: Ownership of variables switches between functions to prevent double free crashes
+ Can't reference twice in Rust