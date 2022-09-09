## [Chaper 9. Writing FreeBSD Device Drivers](https://docs.freebsd.org/en/books/arch-handbook/driverbasics/)
Device; hardware-related stuff that belongs to the system e.g. disks, printers or graphics display with its keyboard.

Device Driver; Software component of OS that controls a specific device.

Pseudo-devices; Driver emulates behaviour of a device in software with zero hardware.

Drivers can be compiled into the system statically or loaded on demand through the dynamic kernel linker facility.

FreeBSD uses the Unix device node system. (`/dev`)

Drivers can be broken down into 2 categories
1. Character
2. Network

### KLD
`kld` - dynamically add/remove functionality from running system. Driver writers can load new changes into a running kernel without the need to reboot. 

`kldload`  - load module
`kldunload` - unload module
`kldstat` - lists loaded modules

*KLD Skeleton layout is very similar to a Linux Kernel module. Written in C, dedicated config code etc. I think that this FreeBSD module code is actually slightly cleaner than Linux module code. There doesn't seem to be any dedicated library (Linux has `<linux/*lib*>`)

Rather than having dedicated functions, loading and unloading is placed into a switch statement that itself is held in a loader function. Module declaration seems to be a simple struct with  module info and there's also a function `DECLARE_MODULE` that takes the struct. I assume this handles telling the rest of the kernel about this new module.
*

FreeBSD modules also use a makefile but it seems to be much shorter and more concise. 

*So far, FreeBSD seems to have made improvements over Linux kernel modules cc code layout while retaining some aspects (such as using a makefile, loading/unloading through commands) I think I'll set a FreeBSD virtual machine up at some point and toy around with FreeBSD drivers.*

### Char devices
Transfers data directly to/from user process. It's the most common driver type with several examples available.

*I could potentially write Example 1 (Simple Echo pseudo-device KLD) into a Linux module*

*Looking at the code, functions are declared in what looks to be the exact same way as Linux kernel modules so you declare a struct that matches the original functions to custom funcs in the code  *

![[function_declr.png]]

*So freeBSD modules are fairly similar to Linux kernel modules with some significant changes such as reducing the volume of configuration code required. That's not to say that they are totally different as freeBSD uses declares funcitions in, effectively the same way as Linux. I'm very eager to set up a freeBSD VM and get to writing some drivers so I can compare freeBSD and Linux code. It might be a good way of demonstrating the improvements that could be made to Linux code? and how they already benefit freeBSD. *