Publish in 2009

## Abstract
+ "Device drivers are notorious for being a major source of failure in operating systems"
+ 39% of bugs are caused by 2 key shortcomings in driver architecture enforced by current OS'
+ Poorly defined communication protocols between driver and OS which confuses developers and leads to protocol violations and a multithreaded model of computation that leads to numerous race conditions and deadlocks
+ Their claim: A better device driver architecture can help reduce the occurence of such faults (using dingo as proof)
+ They claim it avoids confusion & ambiguity and also helps driver writers implement correct behaviour
+ It enforces an event-driven model of computation which eliminates most concurrency-related faults
+ Improves device drivers and generates negligible overhead
+ "allows Dingo and native Linux drivers to coexist, providing a gradual migration path to more reliable device drivers"

## Intro
High volume of code with high error rate

Introduction of advanced hardware features increased driver complexity and have made them more error-prone (hot-plugging, power management, vectored I/O)

+ "We discover and demonstrate quantatively that a large proportion of these factors result from the way drivers interface with the OS and can be eliminated or mitigated by a better design of the driver-OS interface."

The 2 key shortcomings
+ Poorly-defined comm protocols between drivers and OS (confuses developers and lead to protocol violations)
+ Multithreaded model of computation (leads to numerous race conditions and deadlocks)

Dingo is a driver architecture with the aim of simplifying development and reducing number of software defects in drivers.

To reduce protocol errors, Dingo driver protocols are specified by using a state-machine-based forma language called Tingu which provides a crystal clear description of requirements for driver  behaviour which then allows driver programmers to make use of intuitive guidelines. 

Tingu specifications are supposed to serve as documentation to help developers avoid errors, they can also be used as properties to validate driver implementation (whether statically or at runtime). In this paper, only runtime validation is supported via a runtime observer that highlights protocol violations committed by the driver.

Deals with concurrency issues  by defining an event-driver model of computation where the driver interacts with the system through a series of atomic operations. Using this model, avoids the synchronisation complexity of a multithreaded model and eliminates many concurrency bugs.

Dingo arch has been implemented on Linux as well as many device drivers. 

Existing static and runtime approaches only deal with certain types of faults and come at the cost of significant performance overhead and increased design complexity. Dingo contrasts with this as the approach is to eliminate root causes that lead to faults instead of dealing with the consequences of faults. 

## Analysis of driver bugs

Working with Linux and Windows with focus on factors that lead to software faults in drivers.

"A device driver is the part of the OS that is responsible for directly controlling hardware devices." Driver perform 2 main tasks; Communicating with the hardware device (via device registers, DMA, interrupts) and communicating with the Operating System (provides services and resource management, utilises driver throughout rest of the OS such as file systems, networks).

Communicating with both hardware and OS means that a strict protocol is followed which determines correct time, order and format of interactions. 

Device protocol - regulates communication to hardware device
Software protocol - regulates communication to OS

"Each device has a unique device protocol defined by the manufacturer"

Drivers essentially hide manufacturer protocols and standardise devices into families (Ethernet, audio, etc). These 'standard' software protocols are defined by the OS, this also includes definitions for the necessary support services. These protocols and services are known as the OS driver framework. 