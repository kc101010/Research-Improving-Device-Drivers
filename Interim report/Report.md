## Overview (319/500)
Device drivers are a vital component of Operating Systems and facilitate the use of common peripheral devices, interaction with hardware as well as providing a multitude of extensions to an Operating System in its file system(s), network protocol, anti-virus capability and more (Ball et al, 2006). As much as Drivers are a necessity, they also suffer from a range of issues. 

First of all, drivers continue to be programmed with the C programming language. C was first developed at Bell Labs between 1969 and 1973, alongside early development of Unix (Ritchie, M. D, 1993). It was designed as a "system implementation language for the nascent Unix operating system" (Ritchie, M. D, 1993). C has the potential to be memory unsafe (alongside C++). NEED CITATION

Memory safety is an attribute of select programming langages that prevents developers from introducing certain bugs that strongly relate to memory management (Prossimo, 2022) Issues with memory safety usually lead to security problems with typical vulnerabilities being Use-After-Free, Buffer Overflow and Out-of-bounds read/write. 

Next, Drivers have seen little to no change within the last two decades. Evidence pointing to this can be found in Linux Device Drivers 3,  a book written for Linux Kernel 2.6 (Rubini et al, 2005), where its code examples can compile and succesfully run on more recent kernel versions with little to no change. Further evidence supports that code used to write Linux kernel modules has seen almost no change as even online tutorials from 2014 (Karthik M, 2014) continue to compile and run on recent kernel versions. 

I have compiled and run such examples on a small collection of Linux distributions that utilise more recent kernel versions, specifically 4.19.0-17-amd64, 5.15.0-52-generic and 5.15.67-v7l+. 

![[DriverRunningOnPi1.png]]
(Figure 1. Raspberry Pi with Linux kernel 5.15.67-v7l+ running the character driver from final episode of Karthik M. tutorials. )

![[cat_char_drv results.png]]
(Figure 2. Debian Virtual Machine with Linux Kernel 4.19.0-17-amd64 running the character driver from final episode of Karthik M. tutorials.)

![[LDD_Hello_Compiling.png]]
(Figure 3. Ubuntu Workstation with Linux Kernel 5.15.0-52-generic compiling Hello World driver example from Linux Device Drivers 3.)

Figures 1 and 2 demonstrate a character driver which I had written from the Linux Device Drivers Training series by YouTube Channel 'Karthik M'. Figure 3 demonstrates a Hello World example available in Linux Device Drivers 3. 

## Literature Review (0/1000)

## Preliminary Work (0/500)

## Current progress, future work (0/750)

## References