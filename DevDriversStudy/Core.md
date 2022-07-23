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
+ 