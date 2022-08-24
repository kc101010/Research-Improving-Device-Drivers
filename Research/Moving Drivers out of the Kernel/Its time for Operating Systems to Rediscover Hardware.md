# [USENIX ATC '21/OSDI '21 Joint Keynote Address-It's Time for Operating Systems to Rediscover Hardware](https://www.youtube.com/watch?v=36myc8wQhLo)

USENIX OSDI/ATC 2021

OSDI = Operating System Design & Implementation
Hetergenous = not from the same source, individual

## Operating Systems
The reason that there isn't much interest in OS is because Linux 'just works'.

OSDI has a hugely rich design space but very little published work. It's not prioritised as much as it should be. Good work is needed in OSDI as hardware is changing in a way that changes the way we should think about Operating Systems. Why we need to care about OSDI - need to look carefully at hardware.

### The OS runs on the hardware
People assign vague definitions to OS

Functional definition:
The part of software running on the machine that securely handles hardware resources, abstracting hardware in useful ways for applications, protecting apps from each other, protecting the os, protecting data using hardware facilities available.

ccNUMA machine is a fantasy and doesn't match computer hardware. 

Today's Operating Systems copy/look like the basic UNIX model. The kernel is a shared memory multithreaded program that runs in a single address space, allocates VRAM to processes above itself and to system daemons. Inside the kernel are calls and device drivers. Kernel and User space boundary can moved around. 

CPUs have different processors within the unit itself that each have a different view of physical addresses. 

Linux operates within a small portion of a much larger CPU with other 'components' running in other areas of said CPU. "This is an OS that has been congealed". Linux itself isn't handling bootloading, power management and in some cases, security. 

Cross SoC attack.

Linux assumes it is the only operating system. 

Hardware is being tailored to the Operating System and its structure to a point where Linux handles just a small portion of hardware, the rest is handled by hardware itself. Hardware designers have responded to limits created by OS' by hiding more and more OS functionality from OS itself - the OS is incapable of dealing with the kind of hardware in use.  

### New OS papers
Make the same hardware assumptions as Linux. 

### OS Denial and ignorance
Too many people don't know what modern hardware looks like. People are ignorant to this. People find it too comfortable to focus on what Linux does welll and the subset of hardware on which it does it.

+ BespinOS
+ HarmonyOS

### Suggestions
1. Program for an entire SoC (Linux doesn't consider the full picture)

Complexity (complex things) is part of computing science 
+ real-world problem that can be solved
+ can build better tools

#### What you might find
Mind the Gap: Reconnecting Architecture and OS Research (2011)
+ Architecture & OS people find it difficult to communicate and problems are created
+ Consequence: Arch people try to solve problems without engaging OS people
+ Hardware is designed to sandbox Linux in a corner where it can't cause too much trouble to the rest of the system
+ Arch people see that the OS is causing problems and they can't change the OS and take matters into their own hands, designing those problems into other parts of the system

Its not really an Operating system because it isn't really managing the machine. 

2. Build our own computers (SoCs are now being tailored to deal with Linux)

Explore alternative hardware designs that are not constrained by a need to support an OS that isn't acting like an OS. 

It's easier to get hardware built cc Enzian. 
2 node numa server machine.
Most interesting part is BMC (Board Management Computer) which is standard for a server machine. Has massive control over rest of the system. 

### Summary 
OS research is really needed right now.
Hardware's different in scary ways.
Denial and ignorance towards how OS' work with hardware and how hardware has been tailored to just deal with an OS. 

"50 year old view of hardware that doesn't apply anymore"

### QnA
+ How to overcome initial entry barrier; we should be working on hard problems - it shouldn't necessarily be easy. "Systems hard, must work harder"

https://research.vmware.com/projects/bespin
https://en.wikipedia.org/wiki/HarmonyOS
https://people.inf.ethz.ch/troscoe/



### Thoughts
Hardware is much more complex and is being hidden away from everyone including OS designers but at the same time, Linux isn't properly using the hardware in the way that it should so each problem exacerbates each other and creates and constant catch 22 which will degrade both how OS' work and how hardware is designed.  


