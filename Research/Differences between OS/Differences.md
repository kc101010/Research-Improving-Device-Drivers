# Apple (encapsulates iPad, iMac, iPhone)
+ Splits drivers into 3 separate extensions for specific use cases.
	+ Driver Extensions - handles communication between custom hardware and system, runs in user space
	+ System Extensions - features that need kernel-level co-operation, runs in user space
	+ Kernel Extensions - low-level services that can't implemented in any other extension type, runs in kernel space
+ Driver exts written in C++17.
+ System exts can allegedley be written in language with Swift used as an example.

### Debugging
System extensions can have debuggers attached to them. No risk of system crash/restart due to user mode. System extensions need to have proper entitlements and code signatures as well as meeting criteria to run on users system.

(Entitlement is basically permission from Apple to deploy drivers, there are still options for developing and testing)

# Linux
+ Kernel modules run purely in kernel space as user-space drivers are practically worthless.
+ Written in C.

### Debugging 
(taking notes on Linux debugging - will return to this section later)


# Windows
+ Drivers are able to run in user-mode and kernel-mode. Desktop software (in user space) can make use of drivers in kernel space to access kernel information.
+ Written in C. 
+ Various frameworks are provided to help write drivers but MS advises to use pre-existing drivers where possible. (UMDF, KMDF).

### Debugging
WinDbg - debugging tool for kernel

2 separate machines required with the target computer being specifically configured for driver debugging/testing. So system issues are limited to target computer. 

Host - runs debugger
Target/Test - runs driver






# Thoughts
+ Apple is very clearly in the lead and is actively trying to better their drivers. Its not 100% perfect but the changes and improvements made are highly beneficial such as introducing the use of Swift and potentially any programming language. The only gripe I have with Apple is how tight they are with security but for me, that might just need to be a trade-off. 
+ Windows might have a couple of good features (i.e. frameworks and use of different modes) but isn't in the same position as Apple. Drivers are still written in C and still need to be tested across 2 machines.
+ Linux has been running in the same way for a long, long time and this continues into the present.  There seems to be no use of user mode so all drivers likely run in kernel mode. *INSERT COMMENT ON DEBUGGING*. While Linux drivers might be the most accessible due to large amounts of documentation and how easy it is to get your hands on, the trade-off is that you'd be developing in what is essentially an old system that hasn't majorly changed in the last 20 years or so. 