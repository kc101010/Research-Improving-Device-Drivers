https://www.techopedia.com/definition/27006/exokernel

Techopedia (2018) "Exokernel". [Online] Available: https://www.techopedia.com/definition/27006/exokernel [Accessed 24 August 2022]

# Exokernel
Type of OS developed at MIT that tries to provide application-level management of hardware resources. Designed to separate resource protection from management to facilitate application-specific customization. They're typically small in size due to limited functionality.

OS is positioned between applications and physical hardware so they impact performance, functionality and scope of applications. Exokernel essentially wants to do away with hardware abstractions that applications should be built on. Force as little abstractions as is possible and give liberty to use abstractions as/when needed. 

It has a small kernel where all hardware abstractions are moved into untrusted libraries aka library operating systems. Main goal: Ensure no forced abstraction. 

Features:
+ Better support for application control
+ Separate security from management
+ Abstractions moved securely to untrusted lib OS
+ Provides low-level interface
+ Lib OS' offer portabilty and compatability

Benefits:
+ Improved application performance
+ More efficient use of hardware resources via precise resource management
+ Easier development/testing of new OS'
+ Each user-space application can apply its own optimised memory management.

Drawbacks:
+ Less consistency
+ Complex design of exokernel interfaces

In an exokernel, the app is in charge of its own paging, scheduling, context switching, page faults. Application deals with everything itself.

Lib OS: Things like POSIX are ripped out and instead provided to and used with applications that need it.

Design philosophy is to expose hardware as much as possible, allocation (mem, cpu, disk), revocation (memory, cpu), naming