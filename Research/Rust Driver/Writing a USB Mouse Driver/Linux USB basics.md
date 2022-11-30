https://docs.kernel.org/driver-api/usb/writing_usb_driver.html


## Intro
Linux supports almost all USB class devices (incl keyboard, mice, modems, printers, speakers) alongside specific vendor devices (USB to serial converters, digital cameras, mp3 players etc)

Unsupported devices are usually vendor-specific and implement their own protocol(s) which then requires a custom driver. The availability of protocol design/implementation changes from vendor to vendor. Sometimes requiring reverse-engineering

## USB basics
Helpful resources
+ USB protocol specification
+ USB working devices list
+ [Linux USB (linux-usb.org)](http://www.linux-usb.org/)
+ [usb_linux_programming_guide.pdf (psi.ch)](https://lmu.web.psi.ch/docu/manuals/software_manuals/linux_sl/usb_linux_programming_guide.pdf)
+ [Front Page | USB-IF](https://www.usb.org/)
+ 

USB Request Blocks (aka USB urbs) are essential to USB drivers.

Has a registration structure `usb_driver` struct which works in a similar way to the file operations struct.
+ provides info about supported devices
+ which functions to call if a support device is used

Probe and disconnect function pointers are called when a device that matches info provided in `id_table` variable is seen or removed.

Var name is also used in informational messages printed to the system log.

Fops and minor variables are options? (talking about device/driver ids, major/minor numbers?)

Most USB drivers hook into another kernel subsystem e.g. SCSI, network or TTY. These drivers register themselves with the other kernel subsystem and any user-space interactions are provided through that interface. Drivers that don't have a matching kernel subsystem then need a method of interacting with user space (examples lie in MP3 players or scanners). USB subsystem provides a way of registering a minor device njumber and set of  `file_operations` func pointers that enable the user-space interactions. 

USB driver can then be registed by calling `usb_register()` typically in the drivers init function. When the driver is unloaded, it needs to deregister itself from the USB subsystem with a call to `usb_deregister()`

In order to enable the linux-hotplug system (to auto-load the driver when its connected) a `MODULE_DEVICE_TABLE` is necessary. This way you can specify a vendor and product ID (possibly a method to denote which devices should be supported/handled)

## Device operation
When a device is connected with a matching device ID registered with USB core, the probe function is called. The `usb_device` struct, interface number and interface ID are passed as parameters.

Now, the driver needs to verify that the device can be accepted. Returns 0 if the device is accepted. If this device isn't accepted (or if an error occurs) then an error code is returned instead.

Next, the driver should start any setups/configurations such as creating buffers for data storage and initalising USB urbs to interact with the device.

When the device is removed from the USB bus, the disconnect function is called with the device pointer. This is necessary so the driver can clean any private data that was previously allocated and to shut down any pending urbs currently in the USB system.

With the device plugged into the system and driver bound to the device, any functions from the `file_operations` struct that were passed to the USB subsystem will now be called from a user program trying to talk to the device.  The first called function will be `open` as the program will try to open the device for I/O. 

Next, the read and write functions are called in order to send and recieve data to the device.

Thoughts;
+ I *may* have to write my own bindings/handlers to allow USB functionality via Rust. Using unsafe would be easiest, yes, but such bindings would likely be more safe - will need to continue research and make decisions as required.


----








