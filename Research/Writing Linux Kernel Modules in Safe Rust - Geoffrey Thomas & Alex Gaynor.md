tags: #rust #linux 

Key terms:
CVE - Common Vulnerabilities and Exposures

![[kernel-modules-in-rust-lssna2019.pdf]]


+ Vulns due to memory unsafety are common and preventable
+ ###### Mem unsafety
	+ Use after free, double free & wild free
	+ Buffer over/underflow, wild pointer
	+ Use of uninit mem
	+ Data races (that lead to one or all of the above issues)
+ ###### Stats (user space)
	+ Around 49% of Chrome security vulns in 2019 had mem unsafety as root cause
	+ Around 72% of Firefox sec vulns in 2019 had mem unsafety as root cause
	+ Around 81% of zero day vulnerabilities since 2014 have memory unsafety as root cause (tracked by Google Project Zero)
+ ###### Stats (kernel space)
	+ Around 88% of macOS kernel space vulns in [10.14 series](https://en.wikipedia.org/wiki/MacOS_Mojave) had memory unsafety as root cause
	+ Microsoft: Since 2006, around 70% of vulns across all their products as a root cause
	+ Ubuntu: Around 65% of kernel CVEs in Ubuntu Security Notifications between April and October 2019 had mem unsafety as root cause
	+ Around 65% of Android CVEs from May 2017 to May 2018 had mem unsafety as root cause
	+ 2019 paper, Use after free static analysis on kernel data structures focused on concurrency had hundreds of findings
+ Vulnerabilities have the same root cause: C and C++ as they enable programmers to trivially introduce critical vulnerabilities with various consequences
+ ###### Solutions
	+ Trying to harden C
		+ Only mitigates issues, bugs are still possible
		+ Don't permanetly solve issue
	+ Isolation
		+ Sandboxing
		+ Microkernel, overhead issue with isolated code speed
		+ Performance issues
	+ C++ isn't suitable
		+ Was rejected by Torvalds for use in Linux


+ ###### Rust for systems programming
	+ Mem and thread safety
	+ C-compatible calling convention
	+ Variables are const by default, they must be switched to mutable to allow for changing
	+ Structs with slightly different syntax
		+ No inheritance
		+ Good parts OO
	+ Influenced by functional programming languages
	+ Polymorphism through 'Traits' 
		+ Much more obvious and clearer
		+ Can use generics
		+ "You only pay for things you want to pay for"
	+ Enums
		+ can carry data, access to data is safe
	+ Option and Result, error handling/exceptions, similar to kernel err handling
		+ ? is a small char used to quickly handle errors w/o try-catches
	+ Panic
	+ Safety w/o garbage collector
		+ Borrowing
		+ Tracks how long the var is borrowed for, so if something no longer exists it can't be borrowed
		+ References const by default
		+ Mutable references essentially have read/write locks
	+ Safe abstractions for unsafe code
		+ Unsafe can let you bypass safety rules if there's the code you're writing is technically safe
	+ Atomics
		+ No risk of reading while the value is being written
		+ Safe for various uses
	+ Safe & Unsafe Rust
		+ star ptrs are all unsafe but still can be used
	+ Calling C from Rust
		+ Straightforward
		+ uses extern similar to C
		+ Rust insists that C functions are unsafe (in this example Rust has to figure out how C works with ptrs)
		+ Also works backwards (calling Rust from C)
		+ Can also access C types
	