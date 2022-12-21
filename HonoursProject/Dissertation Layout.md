# TITLE
### Developing Device Drivers in Rust

----


# ACKNOWLEDGEMENTS
----

# ABSTRACT
---

# TABLE OF CONTENTS
---

# 1. BACKGROUND[c] 10
An introduction to the problem, a brief history and showcase of my plan(s)

## Device Drivers
###  What are they?

### They have a lot of issues 
+ Memory unsafety in C
+ Not much evolution/change
+ Not in the limelight very often

## Rust
+ History
+ Why's
+ More detailed info comes in Literature review

## Here's my proposition of developing a Linux driver in Rust
+ Demonstrate driver work written in C, undertaken before/during project.
+ Link to Rust and it's benefits - its benefits can only make improvements to drivers, right?
+ Plan to write a driver for Linux in Rust that controls a peripheral connected to  Raspberry Pi 400 
+ Potentially try one of the frameworks and compare?
	+ 'Own' method 
		+ `make VERBOSE` on C driver
		+ Take info from that
		+ Try to make Rust driver (maybe by binding C and Rust or vice versa)
	+ Apple claim any language can be used, let's look into things and see if any research or work has been produced where Rust or similar have been used, has it helped? (Should this be more supplementary over being an outright point to make/discuss?)
	+ There is previous work in making drivers for Rust but none of it seems solid or widely adopted so maybe there's other methods that can be explored
		+ Securing embedded drivers
		+ Matias Heiden
		+ Thomas & Gaynor

---


# 2. LITERATURE REVIEW [c] 20
(chasing current research)

## Operating System Drivers
+ Discussion of differences between OS drivers, how they compare and have they changed

## Rust
+ Discuss Rust as a programming language
	+ Improvements over C/C++
	+ More and more peope are calling for Rust to replace C/C++, provide examples
	+ Loose discussion on similar memory safe programming lanuages (needs research)
	+ Discuss Rust frameworks for Linux drivers 

### Catches
+ All safe programming langs provide some kind of unsafe 'loophole' - is this good or bad? Is it a good point by Stroustrup?

### Google, Android 13
Android 13 has recently seen a significant drop in memory safety vulnerabilities and an associated drop in vulnerability severity with the annual number of memory safety vulnerablities dropping from 223 to 85 between 2019 and 2022 (Vander Stoep, J. 2022). Memory safety vulnerablities now account for 35% of Androids total vulnerabilities (previously 76%) with 2022 being the first year where the majority of vulnerabilities are not related to memory safety.  This drop coincides with a move away from memory unsafe programming languages with Android 13 being "the first Android release where a majority of new code added to the release is in a memory safe language". 

Rust was announced in Android 12 as an alternative to C and C++ with the goal being to shift development of new code to memory safe languages over time. Now, in Android 13, 21% of all new native code is written Rust with approximately 1.5 million total lines of Rust found within Android Open Source Projects across a handful of new features. Google found that "To date, there have been 0 memory safety vulnerabilities discovered in Androids Rust code." it is not expected for this number to remain 0 but is a significant result which suggests that Rust is fulfilling is intended purpose in preventing Androids most common source of vulnerabilities. It's believed that 'it's likely that using Rust has already prevented hundreds of vulnerabilities from reaching production'.

Google also found that the use of Rust allows optimisation of both security and system health with fewer compromises as safety measures typically slow memory-unsafe languages. This usually means developers must make trade-offs between security and performance in adding sandboxing, sanitizers, runtime mitigations, hardware protections which negatively impact code size, memory and performance. It was also found that when compared to other vulnerablities (which have a well defined scope of impact) Memroy safety vulnerabilities are much more versatile. If code execution is obtained in a process, not only is access granted to the specific resource but to everything that the process can access which provides an attack surface to other processes. "Memory safety vulnerablities are often flexible enough to allow chaining multiple vulnerabilities together", it was found that the majority of exploit chains abused in Google products use one or more safety vulnerability. Due to the decrease in severe vulnerabilities, there has been an increase in less severe types with around 15% of 2022 vulnerabilities being Denial of Service vulnerabilities which represents a drop in security risk.  


### Apple re-structuring of Kernel Extensions

## Memory Safety
Memory unsafe languages allow programmers to potentially access memory which is supposed to be outside the bounds of a given data structure (Gaynor, 2019). This is even more detrimental as memory safety vulnerabilities consistently account for the highest percentage of vulnerabilities within large codebases as showcased in figure X.

(Figure X is a custom table listing vuln results from Gaynors science article)

The statistics in Figure X were observed and reproduced across several large code bases (containing millions of lines of code). Each code base was built by a different company, started development at various points in time and applies a different development methodology. The single common property that unites these codebases is that they are written in memory-unsafe programming language such as C or C++. Gaynor concludes that the magnitude of memory-unsafe vulnerabilities is higher than memory-safe vulnerablities and that the research supports the notion that the use of memory-safe languages would critically reduce the total number of vulnerabilities. 
*Here I can also write about the data that we can take away from the table*

In the case of data structures, memory unsafe languages allow programmers to access memory which is supposed to outside the bounds of a given data structure. For instance, an array is able to access an element that doesn't exist. This then means that the program fetches whatever happens to be at that position in memory. When this is the case in a memory safe language, an error is thrown which forces the program to crash. 

As an example, we can consider a program that manages to-do lists for several users. If implemented in a memory unsafe language, it is possible for the programs data structure to both access negative elements and positive elements that don't exist thus the data structure can access data which is outside of its bounds. This can lead to users having the ability to read each others lists which would then be a security vulnerability in the program, this is known as an 'out-of-bounds read'. If users were able to change elements in other users lists, this is known as an 'out-of-bounds write'. If a to-do list is deleted and later requested then a memory unsafe language has the ability to fetch the memory that it was previously finished with. Within the program, this space might now contain another users list, this is known as a 'user-after-free' vulnerability.

## Tools
+ Discuss Dingo framework for drivers
+ Loosely talk about various tools that have came up (WHOOP alongside others used in proposal)

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
