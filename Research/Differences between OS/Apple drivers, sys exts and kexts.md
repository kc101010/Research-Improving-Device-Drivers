# [Implementing drivers, system extensions and kexts](https://developer.apple.com/documentation/kernel/implementing_drivers_system_extensions_and_kexts)

+ Dext (driverkit extension) manages communication between own hardware device and rest of system.
+ System extension implements features that need kernel-level co-operation. E.g. cusotm security/network behaviours
+ Kext (kernel extension) supports low-level services that can't be implemented using a dext or system extension

immediate thought: Apple seems to want to use a ringed level concept similar to OS/Kernel security levels. 

DriverKit and System extensions run in user space and are advised for implementing low-level services. This "improves system stability and makes it easier to develop, debug and install your code."

SysExt framework supports kernel-level features that previously required kexts. An example of what SysExts are used for is an endpoint-security system ext that monitors system events for potential security threats.

From macOS11+, if a SysExt exists then the kernel will load the SysExt over the kext.

Dexts are useful for communicating with custom hardware. 

"
A driver provides a layer of services of accessing the hardware. For example, a driver might configure the device, or it might implement a specific interface for communicating with the device.
"

Dexts ship inside an app bundle. 

When new hardware is attached, the kernel searches for any dexts that can handle the device. From that list, the kernel assembles a series of drivers to communicate with the device. Each new driver builds omn capabilities of the previous to offer new services or config options. 

Custom drivers are advised to be created only for protocols and features unique to custom hardware. It's also possible to use a codeless Dext to map hardware to one of Apples built-in drivers.

## Supporting custom hardware w/o writing driver code
Useful for hardware that entirely communicates using standards-based protocols. Shipping a codeless driver requires less effort and lets you select which driver the system uses for your hardware.

Codeless drivers can be shipped in one of the following packages.
+ codeless Dext where dirier class has no implementation
+ codeless kext which has no executable file

Codeless Dext is just a minimal exe with an empty subclass - a subclass of an existing DriverKit class with no methods. 

## Build Kexts with well-known restrictions
As Kexts run inside the kernel, they must support the same architecture and restrictions as other kernel code. 
+ Kexts on Apple silicon should support arm64e arch as it includes PACs to detect and guard against malicious/accidental changes to memory pointers (PAC = Pointer Authentication Codes). PACs within code are changed at compile time, though they might force you to change how pointers are handled in your Kexts. 
+ Kexts run under KIP (Kernel Integrity Protection). After kernel and kexts are init'd by the system. KIP locks down the kernel memory pages to prevent modifications to kernel/driver code.

# [Debugging and testing system extensions](https://developer.apple.com/documentation/driverkit/debugging_and_testing_system_extensions)

System extensions are required to have proper entitlements and code signatures. They must also meet all other critera for running on the users system. If these requirements aren't met then the activation request will fail with an error. During development, some validation checks can be disabled to simplify writing and testing code. It's important to then re-enable such validation checks before shipping. 

As system extensions run in user mode, they don't need another machine to be tested. Again, the process is just attaching a debugger onto the system extension. Running in user-mode immediatley negates/removes any overhead of crashing and restarting. 