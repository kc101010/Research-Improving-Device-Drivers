# TITLE
### Developing Device Drivers in Rust

----


# ACKNOWLEDGEMENTS
----

# ABSTRACT
---

# TABLE OF CONTENTS
---

# 1. BACKGROUND[c] 10 (~810 words)
An introduction to the problem, a brief history and showcase of my plan(s)

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

Gaynor also provided me with a pull request for the Rust-For-Linux repository which aims to add support for USB device drivers. 

## Project Goal
The aim of this project is to try and overcome the previously highlighted issues by developing a Linux device driver in Rust. Not only will it replace C, Rust and its features should prevent issues with memory safety. Rust is a relatively young language with several benefits and features that aim to improve memory safety. It continues to spread through industry as it was recently incorporated into the Linux Kernel from version 6.1 (Vaughan-Nichols, 2022) and there have been public calls from developers for Rust to be utilised more. An example of this being Microsoft Azure CTO, Mark Russinovich, urging the industry (regarding to C and C++) 'For the sake of security and reliability, the industry should declare those languages as deprecated.' (Claburn, 2022).


---


# 2. LITERATURE REVIEW [c] 20
(chasing current research)

## Operating System Drivers
+ Discussion of differences between OS drivers, how they compare and have they compare to previous efforts

## Rust
+ Discuss Rust as a programming language
	+ Improvements over C/C++
	+ More and more peope are calling for Rust to replace C/C++, provide examples
	+ Loose discussion on similar memory safe programming lanuages (needs research)
	+ Discuss Rust frameworks for Linux drivers 

### Writing a Driver
+ Introduce Rust for Linux - project which has led to the inclusion of Rust in the Linux Kernel (as of Linux 6.1)
+ Discuss various previous works on Rust drivers
	+ Apple claim any language can be used, let's look into things and see if any research or work has been produced where Rust or similar have been used, has it helped? (Should this be more supplementary over being an outright point to make/discuss?)
	+ There is previous work in making drivers for Rust but none of it seems solid or widely adopted so maybe there's other methods that can be explored
		+ Securing embedded drivers
		+ Matias Heiden
		+ Thomas & Gaynor

### Catches
+ All safe programming langs provide some kind of unsafe 'loophole' - is this good or bad? Is it a good point by Stroustrup?
+ Google - escape hatch is required for Systems program in order access additional resources, interacting with system resources and non-rust code.
	+ Unsafe Rust is used rarely and where safety can be easily reviewed

### Google, Android 13 (411w)
Android 13 has recently seen a significant drop in memory safety vulnerabilities and an associated drop in vulnerability severity with the annual number of memory safety vulnerablities dropping from 223 to 85 between 2019 and 2022 (Vander Stoep, J. 2022). Memory safety vulnerablities now account for 35% of Androids total vulnerabilities (previously 76%) with 2022 being the first year where the majority of vulnerabilities are not related to memory safety.  This drop coincides with a move away from memory unsafe programming languages with Android 13 being "the first Android release where a majority of new code added to the release is in a memory safe language". 

Rust was announced in Android 12 as an alternative to C and C++ with the goal being to shift development of new code to memory safe languages over time. Now, in Android 13, 21% of all new native code is written Rust with approximately 1.5 million total lines of Rust found within Android Open Source Projects across a handful of new features. Google found that "To date, there have been 0 memory safety vulnerabilities discovered in Androids Rust code." it is not expected for this number to remain 0 but is a significant result which suggests that Rust is fulfilling is intended purpose in preventing Androids most common source of vulnerabilities. It's believed that 'it's likely that using Rust has already prevented hundreds of vulnerabilities from reaching production'.

Google also found that the use of Rust allows optimisation of both security and system health with fewer compromises as safety measures typically slow memory-unsafe languages. This usually means developers must make trade-offs between security and performance in adding sandboxing, sanitizers, runtime mitigations, hardware protections which negatively impact code size, memory and performance. It was also found that when compared to other vulnerablities (which have a well defined scope of impact) Memroy safety vulnerabilities are much more versatile. If code execution is obtained in a process, not only is access granted to the specific resource but to everything that the process can access which provides an attack surface to other processes. "Memory safety vulnerablities are often flexible enough to allow chaining multiple vulnerabilities together", it was found that the majority of exploit chains abused in Google products use one or more safety vulnerability. Due to the decrease in severe vulnerabilities, there has been an increase in less severe types with around 15% of 2022 vulnerabilities being Denial of Service vulnerabilities which represents a drop in security risk.  


### Apple re-structuring of Kernel Extensions

## Memory Safety (~397w)
Memory unsafe languages allow programmers to potentially access memory which is supposed to be outside the bounds of a given data structure (Gaynor, 2019). This is even more detrimental as memory safety vulnerabilities consistently account for the highest percentage of vulnerabilities within large codebases as showcased in figure X.

(Figure X is a custom table listing vuln results from Gaynors science article)

The statistics in Figure X were observed and reproduced across several large code bases (containing millions of lines of code). Each code base was built by a different company, started development at various points in time and applies a different development methodology. The single common property that unites these codebases is that they are written in memory-unsafe programming language such as C or C++. Gaynor concludes that the magnitude of memory-unsafe vulnerabilities is higher than memory-safe vulnerablities and that the research supports the notion that the use of memory-safe languages would critically reduce the total number of vulnerabilities. 
*Here I can also write about the data that we can take away from the table*

In the case of data structures, memory unsafe languages allow programmers to access memory which is supposed to outside the bounds of a given data structure. For instance, an array is able to access an element that doesn't exist. This means that the program fetches whatever happens to be at that position in memory. When this is the case in a memory safe language, an error is thrown which forces the program to crash. 

As an example, we can consider a program that manages to-do lists for several users. If implemented in a memory unsafe language, it is possible for the programs data structure to both access negative elements and positive elements that don't exist thus the data structure can access data which is outside of its bounds. This can lead to users having the ability to read each others lists which would then be a security vulnerability in the program, this is known as an 'out-of-bounds read'. If users were able to change elements in other users lists, this is known as an 'out-of-bounds write'. If a to-do list is deleted and later requested then a memory unsafe language has the ability to fetch the memory that it was previously finished with. Within the program, this space might now contain another users list, this is known as a 'user-after-free' vulnerability.

## The Exo-kernel

## Tools
+ Discuss Dingo framework for drivers
+ Loosely talk about various tools that have came up (WHOOP alongside others used in proposal)
	+ Coccinelle is actually included within the Linux kernel alongside Rust so is relevant.

## Miscellaneous efforts
+ Trying to harden C
	+ Only mitigates issues, bugs still possible
	+ Doesn't permanetly solve issue
+ Isolation
	+ Sandboxing
	+ Microkernel
	+ Performance issues
+ C++ wasn't suitable
	+ Rejected by torvalds for use linux
	
----

# 3. DEVELOPMENT [c] 40
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
