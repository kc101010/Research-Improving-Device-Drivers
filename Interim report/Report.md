## Abstract

## 1. Overview (497/500)
Device drivers are a vital component of Operating Systems and facilitate the use of common peripheral devices, interaction with hardware as well as providing a multitude of extensions to an Operating System in its file system(s), network protocol, anti-virus capability and more (Ball et al, 2006). Drivers can also be described as the "software layer that lies between applications and physical devices" (Corbet et al, 2005.). While drivers are a clear necessity within an Operating System, they suffer from a range of issues that have various consequences.

First of all, drivers continue to be programmed with the C programming language. C was first developed at Bell Labs between 1969 and 1973, alongside early development of Unix (Ritchie, M. D, 1993). It was designed as a "system implementation language for the nascent Unix operating system" (Ritchie, M. D, 1993). C, C++ and Assembly have the potential to be memory unsafe (Gaynor, 2019) which can then lead to critical vulnerabilities as observed by several organisations over the years (Thomas and Gaynor, 2019).

Memory safety is an attribute of select programming langages that prevents developers from introducing certain bugs that strongly relate to memory management (Prossimo, 2022) Issues with memory safety usually lead to security problems with typical vulnerabilities being Out-of-bounds reads, out-of-bounds writes and use-after-frees (Gaynor, 2019).

Next, Drivers have seen little to no change within the last two decades. Evidence pointing to this can be found in Linux Device Drivers 3,  a book written for Linux Kernel 2.6 (Corbet et al, 2005), where its code examples can compile and succesfully run on more recent kernel versions with little to no change. Further evidence supports this point as even online tutorials from 2014 (Karthik M, 2014) continue to compile and run on recent kernel versions. 

Such examples have been built and executed on a small collection of Linux distributions that utilise more recent kernel versions, specifically 4.19.0-17-amd64, 5.15.0-52-generic and 5.15.67-v7l+. 

![[DriverRunningOnPi1.png]]
(Figure 1. Raspberry Pi with Linux kernel 5.15.67-v7l+ running the character driver from final episode of Karthik M. tutorials. )

![[cat_char_drv results.png]]
(Figure 2. Debian Virtual Machine with Linux Kernel 4.19.0-17-amd64 running the character driver from final episode of Karthik M. tutorials.)

![[LDD_Hello_Compiling.png]]
(Figure 3. Ubuntu Workstation with Linux Kernel 5.15.0-52-generic compiling Hello World driver example from Linux Device Drivers 3.)

Figures 1 and 2 demonstrate execution of a character driver from the Linux Device Drivers Training series by YouTube Channel 'Karthik M' (Karthik M, 2014). Figure 3 demonstrates the compilation of a Hello World example available in Linux Device Drivers 3 (Corbet et al, 2005). It is clear that Drivers, especially Linux Kernel Modules, have not evolved in any major way as code which targets Linux Kernel versions from over a decade ago continues to run on more recent versions.

The end goal of this project is to develop a Linux Driver in Rust. It is a relatively young language with several benefits and features that aim to improve memory safety. It continues to spread through industry as it was recently incorporated into the Linux Kernel version 6.1 (Vaughan-Nichols, 2022) and there have been public calls from developers for Rust to be utilised more. An example of this being Microsoft Azure CTO, Mark Russinovich, urging the industry (regarding to C and C++) 'For the sake of security and reliability, the industry should declare those languages as deprecated.' (Claburn, 2022)

## 2. Literature Review (1000)

### 2.1. Rust (257w)
Rust is a "compiled, concurrent, safe, systems programming language" (Klabnik, 2016) which released in 2015. It was originally invented by Graydon Hoare, an employee at Mozilla, who started the project in 2006 which was then adopted by Mozilla in 2010 (Klabnik, 2016). Rust has several features which are highly attractive especially with regards to drivers and memory safety. 

Cargo is the build tool package manager for the Rust language (Klabnik, 2016) and is reponsible for managing dependencies within a project while also allowing users to create their own packages (Rust Community, n.d.). Rust projects typically include a .toml configuration file which cargo uses to read dependencies. This way cargo can automatically download and install dependencies. If necessary it will also manage dependencies of dependencies and is therefore a highly convenient tool for developers. Cargo is supplemented by Crates.io which is an open-source repository (or registry) that holds all public crates or libraries (Klabnik, 2016). 

Rust is accompanied by a powerful compiler that makes use of a strong type system. It checks code at compile time so errors can be detected before code is deployed (Li et al, 2019).  Therefore, the compiler is also used to highlight errors and prevent developers from making common mistakes (Klabnik, 2016). This is critically important, especially within drivers, as it was previously established that writing device drivers is no easy task.
Developers previously struggled with the Windows XP driver API (Ball et al, 2006) and it has  been highlighted that writing C code for the kernel is difficult (Renzelmann and Swift, 2009).

## 3. Preliminary Work (0/500)

## 4. Current progress, future work (0/750)

## References