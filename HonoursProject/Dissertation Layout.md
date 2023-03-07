# TITLE
### Developing Device Drivers in Rust

----

# ACKNOWLEDGEMENTS
----

# ABSTRACT
---

# TABLE OF CONTENTS
---

# 1. BACKGROUND[c] 10 (1397w)
An introduction to the problem, a brief history and showcase of my plan(s)
(~892 words w/o concepts section)

## Device Drivers
Device drivers are a vital component of Operating Systems which allow for the control of peripheral devices while interacting with underlying hardware. Drivers also provide facilities which can be used to extend an Operating System via file systems, network protocol, anti-virus capability and more (Ball et al, 2006). Described as the "software layer that lies between applications and physical devices" (Corbet et al, 2005), drivers are clearly a necessity within an Operating System however they suffer from a range of issues with dangerous consequences.

Drivers continue to be programmed with the C programming language which was first developed at Bell Labs between 1969 and 1973, alongside early development of Unix (Ritchie, M.D, 1993). It was designed as a "system implementation language for the nascent Unix operating system" (Ritchie, M. D, 1993). Languages such as C, C++ and Assembly have the potential to be memory unsafe (Gaynor, 2019) which can then lead to critical vulnerabilities as observed by several organisations over the years (Thomas and Gaynor, 2019).

Memory safety is an attribute found within various programmming languages with the aim of preventing the developer from introducing certain bugs which strongly relate to memory management (Prossimo, 2022). Memory safety issues usually lead to security problems with common vulnerablities being out-of-bounds reads, out-of-bounds writes and use-after-frees (Gaynor, 2019).

Furthermore, Linux drivers have seen little to no change within the last two decades. Evidence pointing to this can be found in Linux Device Drivers 3, a book written for Linux Kernel 2.6 (Corbet et al, 2005), where its code examples can compile and successfully run on more recent kernel versions with little to no change. Further evidence supports this point as even online tutorials from 2014 (Karthik M, 2014) continue to compile and run on recent kernel versions. Such examples were built and executed on a small collection of Linux distributions that utilise more recent kernel versions, specifically 4.19.0-17-amd64, 5.15 .0-52-generic and 5.15.67-v7l+. It is therefore clear that device drivers have not evolved in any significant way. Code which targets Linux kernel version from over a decade ago continues to run on more recent versions.

## Input from Industry 
During the project, prominent or relevant figures within Game Development and Software Engineering were contacted. These include Jonathan Blow (developer of Braid, The Witness and the Jai programming language) Dave Plummer (former Microsoft engineer, now entrepeneur, best known for his work in creating Windows Task Manager among other projects), Alex Gaynor (Software Resilience Engineer working on the Rust-For-Linux project),  Miguel Ojeda (Software Engineer working on the Rust-For-Linux project) and Asahi Lina (Software Engineer working on a Linux GPU driver in Rust for the Apple M1).

Blow fed back that drivers are a good problem to consider though felt that the project was not ambitious enough. He spoke of a concept called an 'Exokernel model' which he feels would fix a lot of device driver problems as drivers are simply considered 'normal' code. He also feels that the field is held back as the basic idea of what an operating system should be  has already been figured out, Blow seems to think there is little innovation in the field. Alongside his feedback, he also provided me with a USENIX ATC Keynote; 'It's Time for Operating Systems to Rediscover Hardware'.

Plummer explained that he had mostly worked on cache software and filesystems but had never really worked on drivers unless it was necessary. He recommended a textbook, 'Windows 7 Device Driver' by Ronald D. Reeves Ph.D. and gave his best wishes.

Gaynor praised the project, saying "Building a USB mouse driver with Rust for Linux sounds like a great project", and outlined the approach used by the RustForLinux community when writing Drivers which is as follows;

1. Check if RustForLinux already has existing APIs for the relevant kernel subsystem.
2. If not, they design a safe Rust API that exposes the original kernel APIs.
3. It is then possible to use these new abstractiosn to write the original driver.

Gaynor mentioned a pull request for the Rust-For-Linux repository which aims to add support for USB device drivers. After a short correspondance, Ojeda provided some advice and useful resources. He recommended getting used to reading C code within kernel. With regards to the Rust driver, his recommendation was to write a C version which can then be referenced for the Rust driver, this will also help in learning kernel APIs and verify whether issues are a result of Rust support or otherwise. 

## Project Goal
The aim of this project is to try and overcome the previously highlighted issues by developing a Linux device driver in Rust. Not only will it replace C, Rust and its features should prevent issues with memory safety. Rust is a relatively young language with several benefits and features that aim to improve memory safety. It continues to spread through industry as it was recently incorporated into the Linux Kernel from version 6.1 (Vaughan-Nichols, 2022) and there have been public calls from developers for Rust to be utilised more. An example of this being Microsoft Azure CTO, Mark Russinovich, urging the industry (regarding to C and C++) 'For the sake of security and reliability, the industry should declare those languages as deprecated.' (Claburn, 2022).

## General Concepts (505w)

### Kernel (40 words)
A kernel is the primary interface between hardware and computer processes, ensuring resources are used as effectively as possible (Baeldung, 2022). The kernel runs within the operating system as a large section of executable code fulfilling several roles including process management, memory management, filesystem management, device control and networking. 

![[LinuxOSLayersBreakdown-Wiki.png]]
(Figure X, System layer breakdown of Linux. Wikipedia, 2022)

### User space and Kernel Space (84 words)
Kernel space is an area of memory used exclusively by the kernel and encapsulates device drivers . User space is a separate area where user applications run and file systems can be managed (Baeldung, 2022). User applications communicate with the kernel via system calls. Kernel and user space is separated to protect memory and protect the hardware layer of the system as showcased in Figure x. The term 'space' is interchangeable with the term 'mode' as this concept is also related to processors.

### System call interface (100w)
As kernel space is separate from user space this means that kernel-level operations are not freely available to the general user though such functions can still be accessed from user space via a system call interface.  As can be obsrved in figure X, the system call interface is the main boundary between the various kernel subsystems and user space.  System calls are widely used, especially in lower level programming languages such as C and Assembly, with their main function being to obtain information from the kernel for use within user space or send information from user space to the kernel.

### Device node system (72w)
Device nodes are special file types (especially in Unix-based systems) which represent a resource allocated by the kernel. These resources are identified by a major number and minor number which are both stored within the structure of the node. Typically, the major number identifies the device driver while the minor number identifies a specific device (or collection of devices) that the driver can control with these numbers being passed to the driver. 

### Build system (68w)
Building refers to the process of converting (or translating) source code into executable binary files. Thus, a build system is "a collection of software tools used to facilitate the build process" (Zhang, 2020). Build systems have been used for over 30 years and have not seen major change. There are several build systems that can be used in the present including: Make, GNU  Make, CMake, QMake and Ninja. 

### Makefile(82w)
A makefile controls the build system, 'make', and "describes the relationships among files in your program and provides commands for updating each file." (2022, Free Software Foundation, Inc.). The makefile contains a pre-written set of rules which control how make compiles and links a program. It is also possible to run various external commands via make, a cleanup operation as an example. When the makefile is run, it typically builds a piece of software, even updating specific files when additions are made.

### Development  (59 words)
Driver development can occur across 2 different machines. As is the case in Windows and, previously, Apple systems. In such scenarios, the debugger and driver are run on seprate computers (Microsoft, 2022). The computer running the debugger can be known as the 'host' machine while the computer running the driver can be known as the 'target' or 'test' machine. 

---

# 2. LITERATURE REVIEW [c] 20 (3685)

## Operating System Drivers (736/~500w)

### Linux (165/~200)
Linux utilises 'kernel modules' to implement most device drivers, these kernel modules are primarily written in C (Corbet et al, 2005). These kernel modules make use of kernel space, which is separated from user space though the kernel space itself is not compartmentalised. Kernel modules are typically compiled with makefiles. These convert the code into an object file by calling the kernel build system to link code and object files to '.ko' executables. Linux makes several commands available to add, remove and manage kernel modules which is one of the primary methods of testing custom built device drivers. 

Kernel space is separated from user space by the CPU, the Linux kernel runs in the highest CPU level so there is little restriction on what a driver can do this also means that faults that occur within kernel space have a high likelyhood of affecting the overall system. Kernel modules can be run in user space but the associated performance issues mean doing this is worthless.

### Apple ( 289/~200)
Apple device drivers encapsulate iPad, iMac and iPhone. Apple recently re-structured their device driver technology, 'Kernel extensions' to implement additional driver types known as 'Driver extensions' and 'System extensions' (Auricchio et al, 2019). The reason behind the change being that Apple developers found several issues with kernel extensions; They are difficult to develop and debug and also pose a risk to data security, privacy and overall reliability. System and driver extensions improve on kernel extensions and thus are easier to develop while improving security and reliability. 

A system extension is similar to a kernel extension but is instead a component of an application. They are intended to implement features that need kernel level co-operation, such as custom network behaviour (Apple Developer Documentation, 2022). These extensions run in userspace and contain 3 types: Network; Endpoint Security (including Anti-virus and Data loss protecetion); Driver (USB, Serial, NIC and HID). System extensions can use any framework within the macOS SDK as well as any programming language. System extensions are said to be much easier to debug as they do not pause the kernel, there is no need to restart the machine and it is possible to build, test and debug on a single machine with full debugger support. 

Developers also found that the Kernel is an unforgiving environment (Auricchio et al, 2019). That writing and debugging kernel code is difficult, kernel extensions need 2 machines in order to debug which introduces overhead and only has limited debugger support and that kernel extensions are a great risk to security as sucessful attackers can gain free reign in the kernel while any bug in a kernel extension could also be a critical reliability problem. Kernel extensions only support the C and C++ programming languages.

### Windows (143/~200)
Windows also implements the concept of user mode and kernel mode. Microsoft documentation lists various driver types including: Function driver (which communicates directly with the device); Filter driver (which performs auxiliary processing) and Software driver (used when desktop software needs to access something in kernel space). Windows also provides different frameworks which can be used when writing device drivers through Visual Sutdio such as the User Mode Driver Framework (UMDF) and Kernel Mode Driver Framework (KMDF). Device drivers are still written in C but are usually categorised under C++. 

Windows drivers seem to work similarly to those of Linux. Drivers are built into a '.dll' file with  accompanying files including '.inf' (which stores driver information). Depending on the driver type, it may be built into a '.sys' file with a '.cat' file which is used by during installation to verify the drivers signature.  

### FreeBSD (139/~100?)
FreeBSD drivers are similar to that of Linux and makes use of similar concepts. Drivers can be statically compiled into the Operating System or loaded via the dynamic kernel linker facility. FreeBSD makes use of the Unix device node system, known as '/dev' (CHANGE TEXT) which is also found in Linux. FreeBSD makes use of KLD to dynamically manage and extend the kernel without rebooting, which is very similar to Linux kernel module commands. 'kldload', 'kldunload' and 'kldstat' are examples of commands used in managing drivers.  FreeBSD makes use of 'pseudo-devices' where a driver emulates the behaviour of a device in software without hardware. Drivers on FreeBSD are split into two categories: Character and Network. Character devices are typically used to directly transfer data between a user process and other processes, they are the most common type of driver. 

## Rust (~740 words)
Rust is a "compiled, concurrent, safe, systems programming language" (Klabnik, 2016) which was released in 2015. It was originally invented by Graydon Hoare, an employee at Mozilla, who started the project in 2006 which was then adopted by Mozilla in 2010. Rust has several features which are highly attractive especially with regards to drivers and memory safety.

Rust is accompanied by a powerful compiler that makes use of a strong type system and enforces good practices in code. It checks code at compile time so errors can be detected before code is deployed (Li et al, 2019) thus highlighting errors with clear feedback and potential solutions which prevents developers from making common mistakes (Klabnik, 2016). This feature is critically important to drivers, it was previously established that writing device drivers is no easy task as developers previously struggled to work with the Windows XP driver API (Ball et al, 2006). It has also been highlighted that writing C code for the kernel is difficult (Renzelmann and Swift, 2009). The compiler also disallows unused
variables and enforces correct concurrency (Oatman, 2022). If a variable is sent to be owned by a thread or channel, it can no longer be read, and a compiler error occurs if an attempt to read is made. The compiler also forces the developer to handle errors.

Rust is reliable as code is backwards compatible; ensuring old code is always able to compile with new versions of the compiler (Oatman, 2022). Another benefit of this is that old code will continue to benefit from optimisations made to the rust toolchain. Code of all ages will improve and speed up alongside the language itself.  A further benefit is a small revolution in code maintenance, some of the most popular crates can be considered 'complete'. In some cases, they have not been updated in a long time, as the code has no issues and is less likely to rot. 

As shown in Figure 4, Rust has no defined memory model, utilising simple memory structures comparable to that of JVM, Go and C++11. As there is no garbage collection there is no generational memory or complex subtructures. Memory is managed as part of execution, applying the Ownership model during runtime (Sasidharan, 2020).

Rust, of course, implements a stack and dynamic heap within programs. Typically all variables are placed on the stack with the following exceptions: a manually created box; and when a variable size is unknown or grows over time. In these cases, the variable is then allocated to the heap with a pointer to the data placed on the stack. A box is an abstraction that represents a heap-allocated value. In order to manage memory, Rust uses a system of Ownership uphelp by three rules which are applied to both the stack and heap:

1. Each value must be owned by a variable
2. There must always be a single owner for a variable at any time
3. When the owner goes out of scope, the value is dropped

These rules are checked at compile-time. Memory management is conducted at runtime with execution which means there is no cost to performance or other miscellanous overhead. Ownership can be changed with the `move` (CHANGE MOVE TEXT) function. This is performed automatically when a variable is passed to a function or when the variable is re-assigned. The `copy`(CHANGE COPY TEXT) function is instead used for static primitives. 

Rust utilises RAIL - Resource Acquisition is Initialisation - which is enforced when a value is initialised. Under RAIL, the variable own its related resources with its destructor called when the variable goes out of  scope, which reduces the need for manual memory management. This concept is borrowed from C++. Rust also implements a system of borrowing where a resource can only ever have one owner at a time, variables can be passed by value or by reference and the Rust borrow-checker enforces ownership rules and ensures references make use of valid objects. (Sasidharan, 2020).

Variables have lifetimes, a concept which is important for the functionality of the ownership system. A variable's lifetime begins at initialisation and ends when it is closed or destroyed. This should not be considered variable scope. The borrow-checker uses this concept at compile time to ensure that alll references to an object are valid. It is clear that Rusts implementation of memory management will no doubt help in ensuring memory safety, an important factor for the application of Rust within drivers.

### Rust for Linux (210w)
Rust for Linux is a project, originally started in 2019 by Miguel Ojeda with the aim of introducing a new system programming language into Linux kernel. Rust would be chosen as it "guarantees no undefined behaviour takes place (as long as unsafe code is sound), particularly in terms of memory management." (Ojeda, 2022) which would eliminate issues such as use-after-free, double free and data races. The project was created as there had long been desire to write Linux kernel code in Rust. Several attempts were made, the earliest being in 2013, though none of these projects provided Rust support from within the kernel. Rust for Linux carries on initial work that was undertaken by Gaynor and Thomas (2019) in an attempt to introduce Rust into the Linux kernel.

There has since been various technical achievements within the project with several organisations from industry approaching Ojeda with interest including Google, Arm, Microsoft and Red Hat as well as private companies. Alongside these companies, academics  have also reached out such as researchers at the University of Washington. Work carried out by the Rust for Linux project was recently integrated into the Linux kernel, starting from version 6.1, marking the first time a new programming language has successfully been introduced into the kernel. 

### Criticisms of Rust (246w) 
Creator of C++, Bjarne Stroustrup, has previously criticised Rust and similar memory safe languages as "every safe language, including Rust, has loopholes allowing unsafe code" (Claburn, 2022) however in Googles security blog, the use of unsafe is described as an "... escape hatch which allows interacting with system resources and non-Rust code." (Vander Stoep, 2022). Klabnik also previously noted that " fundamentally the machine definitely is not immutable by default ..." therefore unsafe is important to the language (Klabnik, 2016). 

The comparison could be likened to how I/O is implemented in Haskell, a purely functional language where no functions allow for the use of traditional I/O and there are no global variables. Haskell, similarly to Rust and 'unsafe', implements several backdoors for the sake of functionality within the language yet Haskell continues to be used doesn't seem to have met such criticism as Rust has. 

It would seem that while the inclusion of 'unsafe' could potentially lead to issues within a safe programming language, it is ultimately deemed a necessity which allows such languages to be used within system environments and permit interactions with other languages while also considering how the hardware itself works. There are other languages, namely Haskell, that have similarly implemented necessary backdoors. However, it should be considered that the use of unsafe requires the developer to be reponsible for safety. Stroustrups concerns may be valid although the implementation of unsafe seems to be a necessary trade-off for the language to function as intended.  

+ Discuss Rust as a programming language [X]
	+ Improvements over C/C++ [X]
	+ More and more peope are calling for Rust to replace C/C++, provide examples [X]
	+ ~~Loose discussion on similar memory safe programming lanuages (needs research)
		+ ~~Carbon?
		+ ~~Zig?
	+ Discuss Rust frameworks for Linux drivers [X]

### Writing a Driver
+ Introduce Rust for Linux - project which has led to the inclusion of Rust in the Linux Kernel (as of Linux 6.1) [X]
+ Discuss various previous works on Rust drivers [X]
	+ ~~Apple claim any language can be used, let's look into things and see if any research or work has been produced where Rust or similar have been used, has it helped? (Should this be more supplementary over being an outright point to make/discuss?)~~
	+ There is previous work in making drivers for Rust but none of it seems solid or widely adopted so maybe there's other methods that can be explored
		+ Securing embedded drivers - good point to talk about 
		+ Matias Heiden - can be discussed as Windows alternative
		+ Thomas & Gaynor - work laid foundation for Rust for Linux

### Catches
+ All safe programming langs provide some kind of unsafe 'loophole' - is this good or bad? Is it a good point by Stroustrup? [X]
+ Google - escape hatch is required for Systems program in order access additional resources, interacting with system resources and non-rust code. [X]
	+ Unsafe Rust is used rarely and where safety can be easily reviewed

### Google, Android 13 (411w)
Android 13 has recently seen a significant drop in memory safety vulnerabilities and an associated drop in vulnerability severity with the annual number of memory safety vulnerablities dropping from 223 to 85 between 2019 and 2022 (Vander Stoep, J. 2022). Memory safety vulnerablities now account for 35% of Androids total vulnerabilities (previously 76%) with 2022 being the first year where the majority of vulnerabilities are not related to memory safety.  This drop coincides with a move away from memory unsafe programming languages with Android 13 being "the first Android release where a majority of new code added to the release is in a memory safe language". 

Rust was announced in Android 12 as an alternative to C and C++ with the goal being to shift development of new code to memory safe languages over time. Now, in Android 13, 21% of all new native code is written Rust with approximately 1.5 million total lines of Rust found within Android Open Source Projects across a handful of new features. Google found that "To date, there have been 0 memory safety vulnerabilities discovered in Androids Rust code." it is not expected for this number to remain 0 but is a significant result which suggests that Rust is fulfilling is intended purpose in preventing Androids most common source of vulnerabilities. It's believed that 'it's likely that using Rust has already prevented hundreds of vulnerabilities from reaching production'.

Google also found that the use of Rust allows optimisation of both security and system health with fewer compromises as safety measures typically slow memory-unsafe languages. This usually means developers must make trade-offs between security and performance in adding sandboxing, sanitizers, runtime mitigations, hardware protections which negatively impact code size, memory and performance. It was also found that when compared to other vulnerablities (which have a well defined scope of impact) Memory safety vulnerabilities are much more versatile. If code execution is obtained in a process, not only is access granted to the specific resource but to everything that the process can access which provides an attack surface to other processes. "Memory safety vulnerablities are often flexible enough to allow chaining multiple vulnerabilities together", it was found that the majority of exploit chains abused in Google products use one or more safety vulnerability. Due to the decrease in severe vulnerabilities, there has been an increase in less severe types with around 15% of 2022 vulnerabilities being Denial of Service vulnerabilities which represents a drop in security risk.  

## Memory Safety (~397w)
Memory unsafe languages allow programmers to potentially access memory which is supposed to be outside the bounds of a given data structure (Gaynor, 2019). This is even more detrimental as memory safety vulnerabilities consistently account for the highest percentage of vulnerabilities within large codebases as showcased in the figure below.

(Figure X is a custom table listing vuln results from Gaynors science article)

These statistics were observed and reproduced across several large code bases (containing millions of lines of code). Each code base was built by a different company, started development at various points in time and applies a different development methodology. The single common property that unites these codebases is that they are written in memory-unsafe programming language such as C or C++. Gaynor concludes that the magnitude of memory-unsafe vulnerabilities is higher than memory-safe vulnerablities and that the research supports the notion that the use of memory-safe languages would critically reduce the total number of vulnerabilities. 
*Here I can also write about the data that we can take away from the table*

In the case of data structures, memory unsafe languages allow programmers to access memory which is supposed to outside the bounds of a given data structure. For instance, an array is able to access an element that doesn't exist. This means that the program fetches whatever happens to be at that position in memory. When this is the case in a memory safe language, an error is thrown which forces the program to crash. 

As an example, we can consider a program that manages to-do lists for several users. If implemented in a memory unsafe language, it is possible for the programs data structure to both access negative elements and positive elements that don't exist thus the data structure can access data which is outside of its bounds. This can lead to users having the ability to read each others lists which would then be a security vulnerability in the program, this is known as an 'out-of-bounds read'. If users were able to change elements in other users lists, this is known as an 'out-of-bounds write'. If a to-do list is deleted and later requested then a memory unsafe language has the ability to fetch the memory that it was previously finished with. Within the program, this space might now contain another users list, this is known as a 'user-after-free' vulnerability.

### Garbage Collection (347w)
Garbage collection refers to automatic memory management which is carried out by what is known as a garbage collector. It can also be described as a "memory recovery feature" which is "built into programming languages" (Sheldon, 2022). A programming language which uses a garbage collector may utilise many collectors with the aim of freeing memory allocated to objects that are no longer in use or required by the program thus the free memory can be re-used for future object allocations. Garbage collection is utilised in several programming languages including: Java, C# and D. 

D is a systems programming language that utilises garbage collection. The developer allocates memory as needed and from time to time the garbage collector will free unused memory, making such memory freely available once again. D garbage collection is carried out as follows;

1. All other threads are stopped and the current thread is hijacked for garbage collection.
2. Root memory ranges are scanned for pointers to allocated memory, thisReference counting is a mechanism applied in garbage collection. This mechanism works by counting the number of references to a block of memory or object from other blocks. A reference count holds the number of references. The count is increased when memory or a reference to the object is created and decreased when a pointer to the memory is de-allocated or destroyed. Upon the count reaching 0, it is clear that there are no pointer references thus the memory is considered unreachable and should be reclaimed as garbage.

Refence counting has several advantages including being easy to implement, reclaiming objects as soon as they become garbage, quick return of system resources (especially if objects support destructors) and ensuring that garbage collection is distributed throughout all the execution period (these means no system freezes, especially in interactive systems). Although there are several disadvantages, namely the addition of signifcant bloat within code as each assignment will see a call which updates the reference count. In multithreaded systems, the reference count becomes a potential problem as locks must be used to update the reference count. It should also be noted that re-used of optimised atomic operations can still be costly when used repeatedly and the header space used within reference counting has a high cost, when causes significant overhead especially when used with small objects. memory itself is recursively scanned for more pointerAttempts at building a virtualbox machine, recompiling the Linux distro - this saw the most progress and best results+s.
3. All memory that holds no active pointers is freed with unreachable memory requiring destructors queued.
4. All other threads are resumed and destructors run for all queued memory, any remaining unreachable memory is freed.
5. Current thread is returned to previous work. 

Garbage collection has several benefits such as ensuring a program doesn't exceed allocated memory, ensuring continued functionality and taking responsbility from developers who would otherwise need to manually manage such memory thus reducing the likelihood of memory-related bugs. Specifically in the case of D, it was found that garbage collected programs are often faster, can't suffer from memory leaks (thus have more long term stability) and are faster to develop and debug (explicit de-allocation code is not developed, debugged, tested or maintained) (D Language Foundation, 2022). However, garbage collection is not perfect and has clear disadvantages. It was previously found that garbage collection requires 5 times more memory to compensate for overhead when selecting memory to free while retaining performance similar to that of a program without garbage collection (Hertz & Berger, 2005).

### Reference Counting (222w)
Reference counting is a mechanism applied in garbage collection. This mechanism works by counting the number of references to a block of memory or object from other blocks. A reference count holds the number of references. The count is increased when memory or a reference to the object is created and decreased when a pointer to the memory is de-allocated or destroyed. Upon the count reaching 0, it is clear that there are no pointer references thus the memory is considered unreachable and should be reclaimed as garbage.

Refence counting has several advantages including being easy to implement, reclaiming objects as soon as they become garbage, quick return of system resources (especially if objects support destructors) and ensuring that garbage collection is distributed throughout all the execution period (these means no system freezes, especially in interactive systems). Although there are several disadvantages, namely the addition of signifcant bloat within code as each assignment will see a call which updates the reference count. In multithreaded systems, the reference count becomes a potential problem as locks must be used to update the reference count. It should also be noted that re-used of optimised atomic operations can still be costly when used repeatedly and the header space used within reference counting has a high cost, when causes significant overhead especially when used with small objects.

## The Exo-kernel (401w)
The exo-kernel is a concept originally developed at MIT that attempts to return management of hardware resources to the application itself. This kernel is designed in a way that separates resource protection from resource management in order to allow applications to customise how they interact with underlying resources thus the application is completely in charge of its own paging, scheduling, context switching and handling of page faults. 

In present designs, the Operating System is positioned between applications and the physical hardware which impacts performance as well as functionality and scope of applications. The exo-kernel philosophy looks to force as little abstractions as possible, exposing hardware where possible. The kernel itself is small, all hardware abstractions are moved into untrusted OS libraries in order to ensure there is no forced abstraction though components such as POSIX are still available if required by a given application. 

Such kernel would provide several features including improved support to application control (as security is separated from management) and the availability of a low-level interface. Abstractions are securely converted intro libraries, and offer high portability and compatability. This design and its underlying features are complimented by several benefits which include improved performance in applications, more efficiency when using hardware (due to precise resource management), development and testing is simplified alongside each application having the ability to apply its own optimised memory management. It should be noted that the main drawbacks of the exo-kernel are less consistency and a more complex design in the kernels interfaces.

The exo-kernel is an example of how changes in operating system design & implementation (OSDI) may be leveraged in order to improve device drivers. It was previously found that OSDI has stagnated and, similarly to device drivers, does not see much work or research. It was previously described as 'hugely rich design space' (Roscoe, 2021) that has 'very little published work'. Roscoe, in his USENIX keynote, declared that OSDI doesn't have the priority that it should and that the design of operating systems is in fact, affecting how hardware is designed to the detriment of both the operating system and hardware (almost like a feedback loop). It may be that some of the problems we observe with device drivers could be related to the aforementioned stagnations in OSDI and that new research in the field (or making use of new concepts such as the exokernel) could lead to an improvement within drivers.  

## Summary
(summarise main points, how they relate to the project)

### Rust
Bring up main points
+ Clearly reduces memory unsafety and makes several positive changes to a given codebase
+ Is not immune from criticism (though such point is refuted by previous languages and their features i.e. Haskell)

### Memory Safety
We do have other approaches to Memory safety that aren't just Rust such as Garbage Collection though this isn't perfect.

### Exokernel
All different approaches to improve device drivers, the exokernel is a very alternative approach wherein the design of the system itself is considered rather than the specific component (drivers) of a wider system.

----

# 3. DEVELOPMENT [c] 40 (1633)

## Writing C drivers (210w)
Before working on Rust drivers, it is necessary to know how to work with their predecessor, C drivers. C being the primary language utilised by the Linux kernel and, as previously discussed, was created between 1969 and 1974 alongside Unix. C accounts for 98.5% of the code written for Linux (Torvalds, 2023). It should be noted that Linux drivers can often be referred to as 'kernel modules', though within this report they will be referred to as drivers for simplicity.

It should be noted that kernel modules also implement extensions such as filesystems which is why they are not strictly labelled as 'drivers'. As such, filesystem modules are not considered to be a driver in the traditional sense and can rather be considered to be a software driver as it maps low-level data structures to higher-level data structures. While a filesystem extension indeed implements lower level system calls for file access and mapping functionality, the interface which it utilises is separate from that used to facilitate the physical transfer to the disk which is, instead, carried out by a block device driver. Thus kernel modules can also be used to facilitate core but broad functionality such as providing the low-level functions which are required for the functionlity of standard filesytems.

### Driver Types (229w)
There are different types of drivers available for implementation within the Linux kernel, such types are flexible but are categorised into the following classes: Character, Block and Network. Character typically works with a stream of bytes by using file functions such as open, close, read and write. and by working with file system nodes. Unlike a regular file, character devices can only be accessed sequentially. Examples of a character device can be found in text consoles and serial ports. 

A block driver typically works with any device that can host a filesystem. Linux itself reads and writes block devices as if they were a character device thus the only difference between block and character devices is the way that data is managed and they utilise radically different kernel interfaces.

A network driver controls a network interface (whether hardware or software) which itself conducts network transactions between different hosts. The driver itself mostly uses the network subsystem within the kernel and is typically in charge of sending and receiving network packets, the driver is typically designed with a focus on packet transmission rather than focusing on individual connections. Unlike the aforementioned driver classes, the network does not utilise a stream and is not mapped to a filesystem node (though a unique name is still assigned), thus function calls for packet transmission are made rather than calls to read and write.

### Build steps (192w)
C is the main language used within Linux drivers with several, if not all, driver subsystems written in C. Driver source code is stored in '.c' files, similarly to traditional programs. They can either be represented in a single file or across multiple interconnecting files which contribute to make a single driver.  These files can be found alongside a 'makefile' which is used to create the executable format of the driver, in way of the '.ko' file type, which is the object code of the driver.

After running make, the executable driver is produced alongside several other files. With this, it is now possible for the driver to be used within the kernel. Several commands are available to use and interact with a driver, 'lsmod' is an example which lists all drivers and their statuses within the kernel. 'insmod' and 'rmmod' can both be utilised to link and unlink '.ko' files to the current kernel. 'dmesg' can be used to display messages from within the kernel and can act as a form of debugging when loading and running a driver. Such commands form the basis of introducing, using and testing drivers within a Linux kernel and are commonly used during development. 

### 'Hello World' driver (199w)
'Hello, World.' is the common introduction to programming in any language, this also applies to drivers. A simple 'Hello, World' driver can easily be created and serves to showcase key concepts of Linux drivers and how they differ to traditional programs. Much like C the first 'include' lines represent headers and libraries and are commonly used to call various subsystems that can be used within the driver. In this example, libraries ('linux/init' and 'linux/module') can be observed that provide core components for drivers. 

Following this is a macro function that declares the license utilised by the driver. This function is usually a necessity and without it, issues may be encountered regarding compilation and loading. Next, are the functions utilised by the driver, in this case we simply have 'hello_init' and 'hello_exit'. Drivers typically make use of initialiser (which carry out pre-configurations before the driver runs) and exit functions (to complete all de-allocations before unloading the driver). Such functions are present within all drivers and are executed upon inserting and unloading. Finally, we have macro functions in the form of 'module_init' and 'module_exit' which declare the initialiser and exit functions so these can be suitably called and used.

### Character driver (345w)
The character driver can be used to further introduce driver programming concepts. In this case, the 'file_operations' struct is introduced which controls how the driver interacts with files. Such structures are typically used to store and declare key functions throughout several kernel subsystems. Alongside this new struct is the introduction of driver registration. Driver registration refers to the process of driver software being assigned a major and minor number as well as a physical device file. It can also be observed that the driver utilises the filesystem 'fs' subsystem.

Drivers are registered to a device entry file by running the following command: `sudo mknod -m 666 /dev/"DRIVER_NAME" c 240 0` . The first parameter '666' sets the permissions of the given file. Next is the intended name of the entry. The 'c' indicates that the driver is of the character class and finally, the major and minor number that should be used for both the device and driver. The '/dev/' directory is typically used to hold all device/driver entries. Major and minor numbers are used to associate drivers and devices. The major number, specifically, is used to associate entries to the driver with the minor number being used to represent the number of instances of that device.

The file_operations struct assigns and stores new functions, written by the developer, to the typical file functions: read, write, open, close (in this case, known as 'release') allowing for customised behaviour to be assigned to such functions. The struct also declares the owner of these new functions. Within this example, new replacements have been created in the form of 'chdev_open', 'chdev_read', 'chdev_write' and 'chdev_close' which simply print a message alongside the function name when that function is called on the device file.

In order to test the functionality of this driver, various commands can be executed on the device file including 'cat', 'less', 'more' and so on which utilise the previously mentioned file functions. After interacting with the device file, the 'dmesg' command can be run to examine results and verify whether the driver itself is functioning as expected. 

### Scull (42w)
Scull (Simple Character Utility for Loading Localities) is an example of a virtual driver that can be used when learning to work with Linux drivers. Scull acts on kernel memory as if it were a physical device thus is not hardware dependant. Scull implements four devices: 'scull0' through 'scull3' with each device holding global, persistent memory for all data is shared and retained between the aformentioned devices. Alongside these device are four 'First-In-First-Out' devices which act like typical system pipes.


## Rust system software
(writing about the various system/applications written in Rust for learning/practise)

## Building Linux with Rust support

### Initial work (148 words)
Early research and development resulted in the creation of a virtual machine used to test Rust for Linux and Rust integration into the Linux kernel. This virtual machine was built via make using busybox as an aid for configuration. Running via the QEMU hypervisor, the system served to provide insight into core concepts for the project such as building via make, enabling Rust support in the kernel and testing rust support. 

Upon rust being enabled and restarting the machine, various samples were compiled and available for testing. With this, it was possible to add a new sample entry in the way of a simple echo server. This server simply prints out whatever input it receives to its device entry. After writing a new entry into the necessary kernel configuration files and makefile, the echo server was then compiled and loaded as part of the Rust samples on boot. 

### Build steps

### Results (71w)
Further research resulted in the development being focused on virtual Linux systems available via the VirtualBox hypervisor. With this, the available system was much closer to that of a physical machine and was ultimately more capable when compared to the initial QEMU machine. 

A first attempt was made using the Debian distribution though.... 

Later, a second attempt succeeded by utilising Ubuntu which eventually became the main system for development and testing. 

### Rust 'Hello, World.'

### USB driver

### Building on physical hardware (197w)
Attempts were made throughout development to build a Rust-enabled Linux kernel on physical hardware. One such machine was a Raspberry Pi 400 model. It was eventually found that the aforementioned method of rebuilding a Linux kernel is not the most optimal method with regards to the Raspberry Pi. The built in command 'rpi-update next' was instead used to build Linux 6.1 on the machine however this, too, was unsucessful as the kernel reverted to its previous version on restarting the machine. It was also found that while the new kernel could be built and temporaily utilised, it was not possible to enable Rust support therefore it was ultimately not possible to utilise the raspberry pi within development.

Alongside the Raspberry Pi, an attempt was made to build the Rust for Linux kernel on a workstation which was much more similar to the systems created in VirtualBox. Similarly to the Pi, the new kernel would build and install though issues were encountered when attempting to rebuild the kernel with Rust support. As neither attempt to enable Rust support on the Raspberry Pi or workstation was successful, it was necessary to return to working on the functional virtualbox instance.

+ Attempts at building a virtualbox machine, recompiling the Linux distro - this saw the most progress and best results
	+ detail steps
	+ developed a hello world driver (there should also be rust version of the char driver sitting somewhere!!
	+ wasn't as simple, minor details on issues
	+ attempted to create the linux usb mouse driver but ran into problems, USB support isn't quite there yet
	
+ Initial virtual machine - following Filhos tutorial
	+ detail steps
	+ rust echo server
	+ 'hello world example'?
	
+ Attempt at building on a physical machine - not as much time spent, this should actually just be a small note somewhere




----

# 4. EXPERIMENTS [c] 10
### Discuss Results of Rust driver
+ Direct comparison to C
+ Showcase where Rust prevents issues (race conditions etc)
+ Discuss any shortcomings with Rust
+ Short discussion on development experience? (Productivity etc)
----


# CONCLUSION [c] 10
### What is in store for drivers/rust
+ Focusing on Drivers/Kernel etc
	+ Addition of Rust into Linux, what has it done and what will it do?
	+ Anymore progress from Apple/MS?
	+ Are there any solutions/improvements appearing that aren't Rust?
	+ Refer back to Google & Android 13
	+ Consider my own experience?
+ Focusing on Rust;
	+ Discuss how Volvo and other automotive companies are carrying out Research regarding Rust.
	+ DRM driver w/ Linux on Apple silicon
----

# REFLECTION [c] 10
problems etc, not always needed

----


# REFERENCES 
----



# APPENDICES


-----









-----





Before you start writing, make sure you know:

-   The word count (and whether you will be penalised for being too many words over or under) [X]
-   Any compulsory sections and the structure required [X]
-   The style of writing required
-   What types of sources are permitted [X]
-   The types of methodology you are allowed to use 
-   The deadline for submission
-   The requirements for submitting your paper copy for marking, such as formatting and binding 
-   Where, when, and how you submit your dissertation [X]
