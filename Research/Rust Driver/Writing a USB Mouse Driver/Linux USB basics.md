https://docs.kernel.org/driver-api/usb/writing_usb_driver.html

USB urbs (USB Request blocks) are essential USB drivers

Has a registration structure `usb_driver` struct which works in a similar way to the file operations struct.

Most USB drivers hook into another kernel subsystem such as the SCSI, network or TTY subsystem. These drivers register with the other kernel subsystem. 

The USB driver is register with a call to `usb_register()` usually in the drivers init function. 

Driver is deregistered/unloaded with `usb_deregister()`

Hotplugging requires the creation of a `MODULE_DEVICE_TABLE`.