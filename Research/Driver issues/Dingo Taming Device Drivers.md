Publish in 2009

[non-trivial](https://www.yourdictionary.com/nontrivial): not lightweight, any task that is not quick and easy to accomplish. May mean extremely difficult and time consuming

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

Driver stacking is common in OS I/O Frameworks and are typically built to reflect the structure of the underlying I/O bus hierarchy.

Analysed a sample of real defects found within a collection of Linux drivers. Made use of a kernel history site to easily identify and categorise a large number of driver bugs. This was an exclusive investigation for Linux as Microsoft doesn't publicly disclose details about errors in Windows. It would make an interesting comparison as Linux and Windows implement different driver architectures.

Study:
Selected 13 different drivers for different types of devices for buses. With that, they created a bug database for these drivers by analysing all changes made to them between 2002 to 2008. This led to the discovery of 498 defects in the database.

![[Table of driver analysis and faults.PNG]]


Main sources of complexity are split into the following driver faults:

### Device protocol faults 
+ Behaviour violates hardware protocol, leads to hardware failiing to provide its required service 
+ Make up 38% of overall defects
+ At least one third of faults in device-control logic are caused by poorly documented device behaviour
+ These faults are even more common when device documentation is not readily available, in this case such drivers are reverse engineered from another OS
+ Some of these faults are caused by devices whos behaviour deviates from hardware interface standards that are supposed to be in place. 
+ Similar faults are due to devices that violate their documented behaviour, this means that drivers which expect hardware to behave according to set standards/documentation will not function properly and will require workarounds to restore functionality


### Software protocol violations 
+ when driver performs an operation that violates protcol with OS. (Includes all violations of expected ordering, format or timing in interactions between OS and driver.)
+ These types of faults are particularly common in error-handling paths and code paths which handle uncommon/unusual situations like hot-unplug and power management requests (which are often insufficiently tested anyway).
+ Examples of ordering violations; Forgetting to wait for a completion callback from async data request (data protcol violation), trying to resume a suspended device before restoring its PCI power state (power management) and releasing resources in the wrong order.
+ Examples of format violations; incorrectly modifying a data structure shared with the OS, incorrectly initialising a device descriptor before passing it to the OS and falsely returning a success status from an operation that failed.
+ These faults make up 20% of overall driver defects

![[Types of software protocol violations.PNG]]


### Concurrency faults
+ when a driver incorrectly synchronises multiple threads of control executing within it, causing a race condition or deadlock
+ These faults are somewhat unique in that they aren't related to a particular aspect of driver functionality but rather to the model of computation enforced on drivers by the OS
+ Any non-trivial/complex driver is involved in several concurrent activities such as handling I/O requests, processing interrupts and dealing with power management and hot-plugging events. Most OS are designed to run such activities in separate threads that invoke the driver in parallel,
+ This multithreaded model of computation requires the driver to protect itself from race conditions using a variety of sync primitives. A kernel driver needs to keep track of the sync context in which it is invoked. 
+ These faults account for 19% of total bugs.
+ Error rates are higher in USB and IEEE 1934 drivers than in PCI drivers. USB and IEEE busses support hot-plugging, which introduces a device disconnect event to the driver interface.
+ Concurrency faults are mostly introduced in situations where a sporadic event occurs while the driver is handling a steam of data requests.

![[Types of concurrency faults.PNG]]


### Generic programming faults
+ Common coding errors
+ I.e. memory allocation errors, typos, missing return value checks and program logic errors
+ These faults account for 23% of defects.

## Dingo architecture
+ Concurrency faults and software protocol violations consitute 39% of defects in study - clearly a significant source of problems for drivers.
+ "allows driver developers to focus on the main task of a driver: controlling the hardware"

Dingo makes 2 main improvements over traditional hardware architectures
1. Reduces amount of concurrency that the driver must handle by replacing the typical multithreaded model of computation with an event-driven model - eliminates majority of concurrency faults without impacting performance
2. Provides a formal language for describing driver software protocols, which avoids confusion and ambiguity and helps developers implement the correct protocols 

Doesn't try to provide solutions with other types of defects as these are provoked factors that are outwith the influence/control of the Operating System.

Dingo specfies a model for communication between a driver and its environment.

## Event-driven architecture for drivers
Concurrency problems aren't unique to device drivers. In a multithreaded environment, concurrent activities interleave at the instruction levels which leads to non-determinism and state explosion. The result is that many programmers are typically ineffective in working with threads which is why multithreading is the leading source of bugs within a variety of applications including OS Kernels.

An alternative for multithreaded concurrency is event driven concurrency. In this model, the program executes as many event-handlers which react to environment events. These events are strictly serialised and so can replace instruction-level interleaving with event-level interleaving.

Serialisation means that the program state at the beginning of an event can only be changed by the current event handler. This simplifies reasoning about program behaviour and reduces potential for race conditions and deadlocks. 

Threads vs events has been under debate within the systems community for an extremely long time. Writers argue that the use of an event-driven model with device drivers eliminates a majority of concurrency-related bugs. Events can also be implemented in a way that doesn't complicate development or negatively impact performance.

An observation in favour of an event-driven apparoach is that modern drivers are partially event-driven for performance reasons. All perforamnce critical I/O is completed asyncronously. Async request handling leads to improved performance by parallelising I/IO and computation. The pattern of splitting long-running operations into steps of request and completion echoes that of event-driven systems. While drivers aren't fully utilising advantages found within an event-driven concurrency model, it is a style of a programming that is already familiar to developers.

## Implementation
Implemented on Linux by contructing adapters between Linux's mulithreaded driver protocols and Dingo's event-driven protocols. This means that Dingo and native Linux drivers continue to live together on the same system which offers "a gradual migration path to more reliable drivers".




## Thoughts
1. Why are we implementing concurrent drivers in space where everything could change (cc context switch, interrupts, trap calls). Doesn't that just increase complexity of how we write drivers and make them a bit more volatile than typical software. This might be a good link/reason for exo-kernels as that problem is basically almost nulled. The way I think of this is that it's almost like a bundle of wires being tied together or something. 