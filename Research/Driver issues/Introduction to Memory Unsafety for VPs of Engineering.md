[Introduction to Memory Unsafety for VPs of Engineering · Alex Gaynor](https://alexgaynor.net/2019/aug/12/introduction-to-memory-unsafety-for-vps-of-engineering/)

## What is memory unsafety?
A property of certain programming languages which can then allow the developer to introduce certain bugs and allow them to cause serious security issues.

"These bugs deal with errors in how memory is used spatially and temporally."

Memory unsafe languages allow programmers to potentially access memory which is supposed to be outside the bounds of a given data structure. In the case of data structures, memory unsafe languages allow programmers to access memory which is supposed to outside the bounds of a given data structure. For instance, an array is able to access an element that doesn't exist. This then means that the program fetches whatever happens to be at that position in memory. When this is the case in a memory safe language, an error is thrown which forces the program to crash. 

As an example, we can consider a program that manages to-do lists for several users. If implemented in a memory unsafe language, it is possible for the programs data structure to both access negative elements and positive elements that don't exist thus the data structure can access data which is outside of its bounds. This can lead to users having the ability to read each others lists which would then be a security vulnerability in the program, this is known as an 'out-of-bounds read'. If users were able to change elements in other users lists, this is known as an 'out-of-bounds write'. If a to-do list is deleted and later requested then a memory unsafe language has the ability to fetch the memory that it was previously finished with. Within the program, this space might now contain another users list, this is known as a 'user-after-free' vulnerability.

+ C, C++ and Assembly are all languages that are memory unsafe.
+ Memory unsafety also impacts stability, developer productivity and application performance
+ Can lead to a poor experience for users
+ These bugs can be incredibly difficult to track down
+ Multithreading doesn't do well alongside memory unsafety and typically exacerbates stability and security problems.
+ Mozilla tried to introduce multhreading into Firefox's C++ CSS subsystem before successfully re-writing the system in multithreaded rust.

There are practices which can lower the risk of using memory unsafe language;
+ Using modern C++ idioms that can help in producing more safe and reliable code
+ Using fuzzers and sanitizers to help find bugs before they make it into production
+ Using exploit mitigations to help increase difficulty when exploiting vulnerabilities
+ Privilege separation reduce damage potential in the case that a vulnerability is exploited

2019 - pwn2own, large hacking competition
Over half of vulnerabilities exploited were due to memory unsafety
With the exception of only one, every successful attack exploited at least one memory unsafety vulnerability

"... a review of the data makes clear we simply cannot consider continuing to use memory unsafe languages for security sensitive projects."

There are several alternatives to C and C++ including Rust, Swift and Go.