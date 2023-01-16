# Programming guide for Linux USB Device Drivers
[usb_linux_programming_guide.pdf (psi.ch)](https://lmu.web.psi.ch/docu/manuals/software_manuals/linux_sl/usb_linux_programming_guide.pdf)

Fliegl, D. (2000) Programming Guide for Linux USB Device Drivers. [Online] Available: https://lmu.web.psi.ch/docu/manuals/software_manuals/linux_sl/usb_linux_programming_guide.pdf [Accessed 21 November 2022]

## Universal Serial Bus
1994
Compaq, Intel, Microsoft and NEC start to specify the USB. Originally designed with the following intentions;
+ Connection of the PC to the telephone
+ Ease-of-use
+ Port expansion

Specification V1 was first released jan 1996, latest official v1.1 released sep 1998

USB is strictly hierarchical and is controlled by 1 host. Host uses a master/slave protocol to communicate with attached USB devices. Every kind of communication is initiated by the host and devices can't establish any direct communication to oAfter building with Rust support and ither devices.

USB was designed as a compromise of costs and performance. The use of a master/slave protocol solves problems like collision avoidance or distributed bus arbitration. 

[What is UHCI (Universal Host Controller Interface)? (computerhope.com)](https://www.computerhope.com/jargon/u/uhci.htm)

A device can be self-powered, bus powered or both. USB can provide a power supply up to 500mA for devices. Devices that support both power options can switch to self-powered mode when attaching an external power supply.

![](USB%20topology.PNG)

Max comm speed can differ between particular USB devices. The specification decides between low speed and full speed devices. Low speed devices (mice, keyboards, joysticks) communicate at 1.5MBit/s and have only limited capabilities. Full speed devices (audio and video systems) can use up to 90% of the 12Mbit/s which is around 10Mbit/s including protocol overhead.

### Hubs
+ Number of ports at rear panel of computer. These can be used to attach normal devices or a hub.
+ A hub is a USB device which extends the number of ports to connect other USB devices. 
+ Hubs can be self or bus powered full speed devices
+ Physical ports of host controller are normally handled by a virtual root hub. This hub is simulated by the host controllers device driver and helps unify the bus topology. This means every port can be handled in the same way by the USB subsystem hub driver

### Data Flow Types
USB communication runs in 2 directions using 3 different transfer types
+ Downstream/OUT - data directed from host to device
+ Upstream/IN - data directed from device to host

Variants
+ Control transfers - used for configuration/control
+ Bulk transfers - send packets up to the full bus bandwidth, used by scanners/scsi adapters
+ Interrupt transfers - Similar to bulk but polled periodically
+ Isochronous transfers - send/recieve data streams in realtime w guaranteed bus bandwith w/o any reliabilty

### Enumeration & Device Descriptors
When a device is attached to the bus it will be enumerated by the USB subsystem. A unique device number is assigned and then the device descriptor is read.

Descriptor is a data struct that stores device info and properties. (USB standard defines a hierarchy of descriptors). 

![](USBDescriptorHierarchy.PNG)

**Device Descriptor**
Describes general info about a USB device, includes info that applies globally to the device and its configurations.

**Configuration Descriptor**
Information about a specific dev configuration. A USB dev has one or more config descriptors. Each config has one or more interfaces with each interface having 0 or more endpoints. 

**Interface Descriptor**
Describes a specific interface within a configuration. An interface may include alt settings that allow endpoints or their characteristics to be varied after device configuration.

**Endpoint Descriptor**
Contains info required by host to determine bandwidth requirements of each endpoint. An endpoint represents a logical data source or sink of a USB device. Endpoint zero is for all standard control transfers - there is never a descriptor for this endpoint. USB spec also uses pipe as a term for endpoint.

**String Descriptor**
Optional, provide additional info in Unicode format. Can be used for vendor and device names or serial numbers.  

### Device Classes
![](Device%20Classes.PNG)

Standard descriptors contain classification fields
+ Class
+ Sub-class
+ Protocol

These may be used by a host system to associate a device or interface to a driver. 

A class spec serves as a framework which defines the minimum operatoins of all devices/interfaces which identify themselves as members of the class

**HID - Human Interface Devices**
Mainly made up of devices used by humans to control computer system operation.

Includes Keyboards, Pointing Devices (Mice, Trackballs, Joysticks), Front-panel controls and controls that may be found on devices (throttles, steering wheels, rudder pedals)

## USB Device Drivers

In some cases the whole USB device is handled by a single driver. In other cases, each interface of the device has a separate driver.

![](LinuxUSBSubsystem.PNG)

### Linux USB subsystem
USB core, specific API to support USB devices and host controllers. Purpose is to abstract all hardware or device dependent parts.

USB core contains reoutines common to all USB drivers and host controller drivers. These functions can be grouped into an upper and lower API layer.

### USB driver framework
Drivers are registered and deregistered at the subsystem. A driver must register 2 entry points and its name. For specific devices, a driver may register a couple of file operations and a minor number. 

In this case, the specified minor number and the 15 following numbers are assigned to the driver so it is possible to serve up to 16 similar USB devices by 1 driver. The major number of all USB devices is 180.

**Data structures**
All USB related functions/data structs follow same naming convention and start with `usb_` 

+ `name` - typically the name of the module
+ `probe` -  the entry point of the probe function
+ `disconnect` - the entry point of the disconnect function
+ `driver_list` - for internal subsystem use - init to `{NULL, NULL}`
+ `fops` - the usual list of file operations for a driver
+ `minor` - base minor num assigned to device (value must be a multiple of 16)
+ `serialize`
+ `ioctl`
+ `id_table`

**Framework entry points**
The framework adds 2 entry points to normal device drivers










Thoughts;
This is likely to be a good resource but I should look for something more recent - it might be that I need specification and info about USB 3? Not sure and its totally possible that all or most concepts here can translate over.