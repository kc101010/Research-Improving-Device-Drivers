### Intro
+ 'black-boxes' to make specific hardware work with an internal code interface
	+ black box: no knowledge of internal workings - focused on inputs/outputs
+ role: system calls are mapped to device operations used on hardware
+ modular/plugged in
+ new hardware is becoming more common (as well as obsolete), necessity for drivers to be written for such hardware
+ each driver is different and specific though similar principles/techniques are used for all

### Role of driver
+  as the programmer; "choose an acceptable trade-off between the programmming time required and the flexibility of the result"
+ providing mechanism, not policy
	+ mechanism = 'what capabilities are to be provided', lower-level or base functionality
	+ policy = 'how those capabilities can be used', higher level control over functionality
	+ examples: X server, window/session managers; structure of TCP/IP
+ addressing issues with different parts of a program or even different programs can lead to easier development and adaptation in software package
+ "write kernel code to access the hardware, but don't force particular policies on the user, since different users have different needs"
+ deal with making the hardware available which then allows flexibility (due to little/no constraints)
+ it is sometimes necessary to implement some policy, moreso in the case of limitations/boundaries
+ different perspective: software layer that lies between applications and physical device
+ "driver design should be a balance between many different considerations" 
	+ example: 1 device may be used concurrently between programs, programmer has complete freedom to determine how to handle concurrency
	+ could create own implementations of systems or provide libraries to help application programmers handle policies for base driver(s)
+ There is a trade-off between providing the user with all possible options, the time to write the driver and the need to keep things simple to minimise errors
+ Policy-free drivers
	+ support both sync/async operations
	+ can be opened many times
	+ can utilise full capabilities of hardware
	+ little to no software layers to simplify things or provide policy operations
	+ work better for end users
	+ are easier to write & maintain
	+ policy-free is a common target for software designers
+ Drivers can be released alongside user program to aid configuration and device access - client lib can be provided to implement features that don't need to be part of the driver itself
+ User programs are integral to software packages, policy-free packages use config files too set default behaviours to mechanisms

### Kernel sections
kernel is a big chunk of executable code that handles all resource requests from processes

+ Process management
	+ creation/destruction of processes
	+ handling I/O
	+ inter-process communication
	+ scheduler (to control how CPU is shared between processes)
	+ kernels proc mgmt abstractions many processes onto either 1 or many CPU(s)
+ Memory management
	+ memory policy is critical for system performance
	+ kernel builds virtual addressing space for any/all processes on top of the available limit of resources
	+ different parts interact with mem-mgmt subsystem through function calls such as;
		+ malloc()
		+ free()
+ Filesystems
	+ structured filesystem on top of unstructured hardware
	+ result of file abstraction is  used heavily through entire system
	+ Linux has different ways of organising data on physical medium (supports many filesytems)
+ Device control
	+ Every system operation will map to a physical device at some point (except CPU, mem and few others)
	+ device control operations are performed by code specific to the addressed device (aka a device driver)
	+ kernel must hold a driver for every peripheral on the system
+ Networking
	+ managed by OS
	+ most network operations aren't specific to a process so incoming packets are asynchronous events
	+ packets must  be collected, identified & dispatch before a process handles them
	+ system is in charge of delivering packets across program and network interfaces and must control program execution acocording to network activity
	+ all routting and address resolutions are implemented within the kernel
### Kernel Modules
+ extend kernel features at runtime, add/remove functionality while kernel is up and running
+ Linux offers support for a few different types/classes including device drivers
+ Each module made up of object code (unlinked from executable) that can then be linked & unlinked dynamically to running kernel by using insmod and rmmod
+ modules are separated by class depending on what they actually do

### Module/Device Classes
+ 3 flexible types
	+ ###### Char module
		- stream of bytes
		- uses open, close, read, write
		- examples; text console and serial ports
		- accessed via file system nodes
		- char devices can only be accessed sequentially unlike a regular file
	+ ###### Block module
		+ block refers to a device that can host a filesystem (a disk)
		+ Linux reads/writes block like a char device, any number of bytes at a time. Incls functions
		+ block & char devs only differ by way that data is managed
		+ also accessed via fs nodes
		+ block and char drivers have totally different interfaces to kernel
	+ ###### Network module
		+ any net transaction is made through an interface (device able to exchange data between other hosts)
		+ interface may be hardware or software (loopback intf)
		+ in charge of sending/receiving packets, driven by net subsys of kernel, w/o knowing how transactions and packets properly relate
		+ usually designed around transmission/receipt of packets, knowing nothing abouot individual connections
		+ as its not a stream, it's not mapped to a fs node. Unix still assigns a unique name but that has no entry in FS
		+ Rather than using read and write, kernel calls functions related to packet transmission
+ Good programmers typically create a different module for each new functionality they implement. 
+ Decomposition is a key element of scalability and extendability

{SYSTEM CALL INTERFACE}
^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Proc mgmt     mem mgmt   fs     devctl   net
FEATURES (concurrency, VRAM, Files/Dirs, dev access, connectivity)
SOFTWARE SUPPORT (arch code, mem mger, FS types, char devs, net subs)
HARDWARE (CPU, MEM, DISKS/CDs, CONSOLES, NET INTERFs)

+ Some drivers work with more kernel layers of kernel support functions for a certain type of device. USB devices are driven by USB module that works with USB subsys but device shows up in subsys as char (USB serial port), block (USB mem card reader) or net device (USB net interface)
+ More classes added e.g. FireWire drivers, I2o drivers
+ ==Kernel developers collected class-wide features and exported them to driver programmers to avoid duplication of work and bugs, this simplified and strengthened the process of writing such drivers
+ Hardware and software functionalities are modularised in the kernel. Such as filesystems, fs type determines organisation of info on a block device to represent the tree of dirs and files. This is not a driver as there's no specific device related to the information. FS type is a software driver as it maps low-level data structs to high-level data structs. The FS decides filename rules and stored info in a dir. 
+ FS module must implement lowest sys calls to access dirs and files, by mapping filenames, paths and access modes (essentially inodes) to data structs stored in data blocks. This interface is totally separate to the physical data transfer between the disk, which is accomplished by a block device driver.
+ UNIX/Linux keeps it ability to decode FS info at lowest level 
+ Programmer shouldn't need to write a FS module as the kernel includes code for most important FS types
+ Modules are used to provide broad necessary functionality e.g. provide low-level functions needed for most filesystems to properly function


### Security Issues
+ Security checks are carried out by kernel so if kernel is insecure, the rest of the system is just as (in)secure
+ Only auth/privileged users can load modules, checks for this are carried out by init_module syscall, Only superuser or privileged intruder can exploit such privileged code.
+ Driver programmers should avoid encoding security policy within code as Security policy can be better handled at higher levels within the kernel via a sysadmin
+ Should be aware of device access types that might seriously affect the rest of the system and thus provide adequate controls to prevent further issues. E: device ops that affect global res' which could in turn damage hardware or affect other users are typically locked to privileged users and this check MUST then be carried out in the driver
+ Being able to write/create errors in C is very easy. If the language is used improperly this will more likely than not lead to security problems. (Many security problems can be caused by buffer overrun errors, where programmer forgets to check amount of data written to buffer so data is stored/overwritten beyond the intended size)
+ Errors like the buffer overrun have the ability to compromise the entire system and MUST be avoided. With dev drivers, this can be avoided by ensuring the interface for the user is narrowly defined and highly controlled
+ ###### General security ideas
	+ Any input received from user processes should be treated with great suspicion, shouldn't be trusted unless verified
	+ Treat uninitialised mem carefully, make sure to zero (or init) memory from the kernel before passing over to a user proc/device, as not doing so could result in a leak of sensitive info
	+ If your device interprets incoming data, ensure the user can't send anything that could compromise the system
	+ Think about possible effect of device operations (such as formatting a disk) that could affect the system, these MUST and SHOULD be restricted to privileged users
	+ Be careful of receiving software from 3rd parties, especially for the kernel. Avoid running kernels compiled by an untrusted friend as this could very very easily lead to unexpected back doors
	+ (Linux can be compiled to have no module support which prevents all these issues which then means that drivers need to be built directly into the kernel. There are other similar security measures that aren't as drastic)


### Licensing
+ Code is likely better off as free software
+ If code is intended for main kernel or requires patches to kernel, you MUST use a GPL-compatible license on releasing the code. Include the code as its necessary to let people rebuild the program at will

### Kernel Dev Community
+ central gathering point is the linux kernel mailing list
	+ [Majordomo Lists at VGER.KERNEL.ORG](http://vger.kernel.org/vger-lists.html#linux-kernel)
	+ [The linux-kernel mailing list FAQ](http://vger.kernel.org/lkml/)
	

##### Key words/potential further research
+ Development kernels
