tags: #windows 

https://learn.microsoft.com/en-us/windows-hardware/drivers/gettingstarted/

This is actually the most recent driver documentation i've been able to find so far, various dates between 2021 and 2022. Specifically December 2021 through to May 2022. Seems like Microsoft decided an upheaval of documentation was in order. 

# What is a driver?
"software component that lets the operating system and a device communicate with each other"

![[SimplifiedDriverBreakdown.PNG]]

## Expanded
+ Not all drivers are written by the device company. Usually a device is designed to a hardware standard which then means that the driver can be written by MS. Taking the hassle away from the device designer.
+ Not all drivers communicate directly with a device.

I/O requests see several drivers layered into a driver stack which all alter/convert the request. Rather than communicate directly with the device, they manipulate the request and pass it below, further into the stack. 

![[MoreComplexDriverBreakdown.PNG]]

A **Function Driver** communicates directly with the device
A **Filter Driver** performs auxiliary processing

Sometimes filters just observe and record into about I/O reqs without participating in them. Some of them can act as verifiers to make sure the other drivers handle the I/O req correctly.

Definition could be expanded by saying a driver is any software component that watches or takes part in communication between OS and driver.

## Software drivers
Some drivers aren't even associated with Hardware.

Software drivers are used when traditional/desktop software needs some kind of access to something in Kernel space. The typical setup is that you'll have the Software itself that runs in User mode, providing an interface. Then you'll have the Software driver that runs in Kernel mode and can access core OS data/data structs. These drivers **are not** related to a hardware device.

In an opposite way, some device drivers can also run in user mode. 

## Function drivers
PCI - Peripheral Component Interconnect

A function driver that connects via PCI communicates directly with the device. It obtains addresses mapped to port and mem resources on the device. The func driver communicates directly by writing to those addresses. 

Usually, a device doesn't connect directly to the PCI bus but connects to a host bus adapter which is connected to the PCI bus instead. 
+ USB Host Controller

![[FunctionDrivers_ToasterExample.PNG]]


## MS provides various built-in drivers
+ ACPI     : ACPI driver
+ Audio   : MS Audio Class Driver
+ Busses  : Native SD bus driver, Native SD storage class driver, Storage miniport driver
+ HID       : HID I2C Driver, Legacy game port & keyboard class, legacy mouse, PS/2
+ Imaging : WSD scan class driver
+ Print       : MS Plotter driver, MS PostScript, MS Universal, MS v4, MS XPS
+ Sensors  : Sensor HIID class
+ Touch
+ WPD      : Media Transfer Protocol class

## Driver Models
It seems that MS has written certain sections of drivers which manufacturers, driver writers etc can use to easily make drivers.

# General Concepts

## User mode
Windows creates process for user-mode app. This process gives the app a private virtual address space and private handle table. As each apps virtual address space is private, they can't overlap/overwrite each other. Each app runs in isolation so if a crash occurs, the crash is limited to that process/thread. This saves other apps and the OS as a whole. 

Virtual address space of a user app is also limited. This limit is set in place in order to prevent apps from altering and likely damaging critical OS data. 

## Kernel Mode 
All kernel code shares a single virtual address space. Drivers aren't isolated from each other or OS code. It's very easy to compromise OS or other driver data if the wrong virtual address is written to. Kernel driver crashes means a crash for the OS itself. 

![[UserKernelMode.PNG]]

## Virtual address spaces
When a processor reads/writes to a memory location, it uses a virtual address. During the read/write, the processor will translate that virtual address to a physical memory address.

Advantages;
+ Program can use a sequential range of virt addreses to access a large memory buffer that is not contiguous in physical memory. 
+ Program can use virtual addresses to access a memory buffer that's larger than available physical memory. As physical memory supply becomes smaller, the mem manager starts saving pages of physical memory to a disk file. Pages of data or code are moved between physical memory and the disk as required.
+ Virtual addresses used by different processes are isolated from each other. Code from different processes can't change physical memory in use by another process or the OS.

Virtual address space == range of virtual addresses available to a process.
+ 32bit processes have a Vaddress space of 2GB range, 0x00000000 through 0x7FFFFFFF
+ 64bit processes have a Vaddress space of 128TB range, 0x000'00000000 through 0x7FFF'FFFFFFFF


(contiguous = sequential)
A range of virtual addresses can also be called a range of virtual memory. 

![[VRAM.PNG]]

Drivers are a collection of callback that, once initialised, idle and wait to be called when the system needs something. This could be anything from a new device arriving, to managing an I/O request from a user app, a request from another driver or surprise removal event and more.


# Writing a Windows driver
Different frameworks for writing drivers available through MS Visual Studio.
Also holds templates. 
+ UMDF (User Mode Driver Framework)
+ KMDF (Kernel Mode Driver Framework)

Drivers are built into '.dll' the driver file itself and then '.inf' an info file about the driver for windows. This can depend on which type of driver is being created as drivers may also be built into a '.sys' file. When the sys file is made, the inf file is still there but a '.cat' file is also created which the installer uses to verify the drivers signature.

Drivers are still written in C but are categorised as C++, I assume because MS has no C category.

Seems to essentially have a similar setup to Linux Modules;
+ Config code
+ Separate kernel lib/functions

Full Driver Code from examples
![[FullDriverCodeExcerpt.PNG]]

WinDbg is a debugging tool for kernel debugging.

Various guidelines, [articles]([Creating Reliable Kernel-Mode Drivers - Windows drivers | Microsoft Docs](https://docs.microsoft.com/en-us/windows-hardware/drivers/kernel/creating-reliable-kernel-mode-drivers)) and [written guidance]([Driver Security Guidance - Windows drivers | Microsoft Docs](https://docs.microsoft.com/en-us/windows-hardware/drivers/driversecurity/)) for programming device drivers

This driver doc is sometimes written in a way that readers are encouraged to utilise MS' own sample code but modify it for their own needs. In which case, developers need to replace several unique names, ids and so on.

Also provides tools for hardware development and hardware development boards. Sharks Cove hardware dev, this can also be used driver debugging/testing.

## Testing and debugging
Debugger and driver run on separate computers. With a *host* running the debugger and *target* or *test* computer running the driver. The target computer must be [specifically configured](https://docs.microsoft.com/en-us/windows-hardware/drivers/gettingstarted/provision-a-target-computer-wdk-8-1) for driver debugging/testing.

Driver Module Framework
WFD extension that enables extra functionality for a WDF driver dev. Helps write any type of WDF driver better and faster. 

DMF Modules can be shared between different drivers. 


