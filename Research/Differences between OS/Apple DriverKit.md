[DriverKit](https://developer.apple.com/documentation/driverkit)

Device drivers that run in user space
Seems to be a universal framework across iOS, iPadOS and macOS?

"framework defines the fundamental behaviours for device drivers in macOS and iPadOS"

Uses C++ classes to define driver structure and support event handling and memory allocation. Provides facilities to access driver I/O registry entry.  There are other driver kits for specific devices/hardware such as USB, HID, Networking, PCI, Serial and Audio.

DriverKit drivers run in User space rather than Kernel extensions. This improves system stability and security. Possible to create a driver as an app extension and deliver from inside the existing app.

Thought: Seems to be a recent tool/development as there is a note that the kit is available to iPadOS devices w/ M1 CPU.

![[user and kernel space.PNG]]

