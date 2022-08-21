tags: #windows 


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
