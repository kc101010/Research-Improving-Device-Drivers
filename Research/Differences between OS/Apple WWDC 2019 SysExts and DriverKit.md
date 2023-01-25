# [System Extensions and DriverKit](https://developer.apple.com/videos/play/wwdc2019/702/)
## Modernising kernel extensions and device drivers

Auricchio, J. Douglas, S. Deandrea, S. (2019) "System Extensions and DriverKit: Modernizing kernel extensions and device drivers" [Online] Apple Inc. Available: https://developer.apple.com/videos/play/wwdc2019/702/ [Accessed 24 August 2022]

Core OS group
macOS 10.15 Catalina

Extending the Operating System

Problems with Kexts
+ Difficult to develop and debug
+ Data security/privacy risk
+ Reliability risk

System Extensions and DriverKit
+ More reliable
+ More secure
+ Easier to develop

System Extension
+ Part of an app
+ similar to Kext
+ Runs in userspace
+ 3 types
	+ Network
	+ Endpoint Security -- AntiVirus/Data loss protection
	+ Driver - USB/Serial/NIC/HID
+ No restrictions on mem alloc, sync, latency
+ Use any framework in the macOS SDK
+ Use any language including Swift (Research to see if Rust has been used!!)
+ Easier to debug, doesn't stop kernel
+ No restarts
+ Build, test and debug on 1 machine with full debugger support
+ Improves security, privacy, reliability


Driver Extensions still have some restrictions
+ Still restricted due to direct hardware control
+ DriverKit frameworks in tailored runtime
+ DriverKit API is C++17


"In userspace, outside the kernel" - "Comfortable, modern programming environment"

Kernel is an unforgiving environment
+ Must never stop running, wait or crash
+ Code needs to be fast, predictable, careful with resources and bug-free
+ It's difficult to write and debug kernel code
+ Kexts only support C/C++
+ Kexts need 2-machine debugging with overhead with limited debugger support
+ Compromised kexts have no security rules, successful attackers gain free reign in kernel 
+ Any bug in a kext could also be a critical reliability problem 
+ kexts deprecated after catalina 10.15


Entitlements
A driver needs
Base entitlement - `com.apple.developer.driverkit`
Transport entitlement (attaching a device), specific to device type e.g. - `com.apple.developer.driverkit.transport.usb`
Family entitlement (provide service to OS) e.g. -  `com.apple.developer.driverkit.family.hid.device`

Code signing and approval process
