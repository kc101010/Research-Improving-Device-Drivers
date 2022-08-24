## Pre-project observations

##### Stats
("Writing kernel mods in safe rust")
Mem unsafety as root cause of error in user space programs
+ ~49% of chrome security vulns (2019)
+ ~72% of firefox sec vulns (2019)
+ ~81% of 0-day vulns since 2014
Mem unsafety as root cause of error in kernel space programs
+ ~88% of macOS vulns in 10.14 series
+ ~70% of vulns across all MS products since 2006
+ ~65% kernel CVEs in Ubuntu Sec Notifications between April and October 2019

It's clear that in **both** user-space and kernel-space, memory safety should be a top priority as memory unsafety is responsible for high amounts of vulnerabilities within various products. 


##### Specifically in the case of Linux, drivers haven't changed
+ I'm using a book from 2005 (LDD3) in order to learn about Linux device drivers
+ Another resource (Karthik M Youtube channel) which i've previously used is from 7 years ago
+ Code examples targetting Linux kernel 2.x continue to run on 4.x, haven't yet tested on 5.x but I expect such examples to still work


##### Research is there but not in mass amounts
+ 2019, some research has occured attempting to solve the driver problem with Rust
+ I've observed that a selection of this new research uses the exact same sources (i.e. from early 2000s etc) as I did in my proposal thus there isn't really any newer data to draw from


###### Rust is the most commonly proposed replacement for C
+ Seems that most research wants to give C the boot in favor of Rust
+ It seems that solely the use of C is responsible for a portion of driver issues/failures (which Rust solves)
+ C's method of error handling is 'clumsy and error-prone' - notes from securing embedded drivers
+ Matthias Heiden worked on a solution with pros and cons
	+ *Pros*: less crashing = more speed, no need to worry about deallocation, custom allocator, more efficient language, similar setup to current drivers
	+ *Cons:* implementation/translation isn't perfect, some aspects can be tedious, raw pointers
+ Li et al: Their paper showcases various reasons why Rust is beneficial to driver dev
	+ Use of ownership system automates a lot of aspects such as mem mgmt
	+ This framework manages to reduce boilerplate code by implementing driver types as traits which will share an interface (a point that I discussed in my research paper where I believed the use of more modern languages could likely reduce code volume or repeating code)
	+ Drivers are more easily managed due to the sharing of a unified interface
	+ Compiler messages are much clearer and more assistive
	+ Rust merges fairly well with C, there are a few nicks and issues but the framework is viable.


###### There is room for blame on developers but understand, the area in which they work is delicate and difficult
+ "The hard part is understanding your  device and how to maximise performance." - Corbet, J. Linux Device Drivers. 3rd Ed. pg 18. (January, 2005)
+ ' prevent developers from making mistakes that then become attack vectors for hackers' - my own notes on secure drivers paper
+ 'This situation is even worse for 3rd party devs due to little scrutiny and general low quality of products' - see above
+ 'Writing a safe driver is not easy. The monolithic architecture means that the kernel itself and drivers run in the same address space and privilege level. This suggests that drivers can't be stopped from changing critical kernel memory or calling the wrong kernel functions which could lead to a kernel panic. ' - see above
+ "programers tend to focus on functionalities instead of carefully considering all possible causes of errors" - s.a.
+ 'Missing error handling may not be realised until the product is shipped' - s.a.
+  "programmers usually use inconsistent and implicit placeholders as return values" (0 = success, -1 = failure) (Result enum in Rust can solve this) - s.a.
+ 'Programmers have a common problem of forgetting to check the return value which is then a failure to handle the error' -s.a.
+ 'Vulnerabilities have the same root cause: C and C++ as they enable programmers to trivially introduce critical vulnerabilities with various consequences' - linux kernel mods in rust


##### Apple re-structures their kernel extensions for improved reliability - 2019
Apple cites a lot of issues which are already highlighted in my proposal, this project and similar papers
+ Kernel code is difficult to write and debug, it's highly demanding
+ Any kext bug could be a critical reliability problem
+ Language support is limited

And their solution implements a lot of what I discussed as being possible in my proposal
+ Use any programming language (Swift is specified in slides)
+ Easier to debug. Performed on 1 machine with full debugger support.
+ Improved security, privacy, reliability.
+ Less restrictive, more access to frameworks

+ Kernel extensions were broken down and placed into User-space in order to combat several issues
	+ Kernel code is difficult to write and debug, it's highly demanding
	+ Compromised kernel extensions can give attackers free reign in kernel
	+ Any kext bug could be a critical reliability problem
	+ Language support is limited
	+ Risk to Data Security/Privacy/Reliability
+ System and Driver Extensions replace Kexts 
	+ Still similar
	+ Runs in userspace, faults & crashes don't affect the OS or device itself
	+ Less restrictive, more access to frameworks
	+ Use any programming language (Swift is specified in slides)
	+ Easier to debug. Performed on 1 machine with full debugger support.
	+ Improved security, privacy, reliability.

I'll describe system extensions further with 2 quotes; "In userspace, outside the kernel" with a "comfortable, modern programming environment"

##### What if we treated device drivers in the same way as applications


 

