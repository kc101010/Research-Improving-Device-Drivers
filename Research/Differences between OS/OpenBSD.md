## [Getting started with OpenBSD device driver development](https://www.openbsd.org/papers/eurobsdcon2017-device-drivers.pdf)
[Getting started with OpenBSD device driver development, by Stefan Sperling (EuroBSDcon 2017)](https://www.youtube.com/watch?v=W5qhWw07qpU)

Sperling, S. (2017) "Getting started with OpenBSD device driver development". [Online] Available: https://www.youtube.com/watch?v=W5qhWw07qpU [Accessed  7 September 2022]

+ "Vendor drivers are often poorly written, we need to be able to write our own"
+ "Source code is not documentation" - most of their documentation comes from Linux source code and similar systems so OpenBSD devs would like dedicated hardware docs to work with.

 Differentiates between Firmware and Drivers?
 + "Device driver code runs in our kernel"
 + "Firmware code runs on a peripheral device, not in our kernel" - not on main CPU but on device
 + "Firmware often runs with high privileges, use at your own risk"

Slide 8 holds some advice/info that might actually be useful for the project itself

## Process
+ Pick smaller, more reasonable projects to start

Development might take several months (though it seems like development here is much, much more hands on). A lot of bugfixing and then working.

### Tips for development environment
Use at least 2 machines as you will regularly crash the kernel
+ One for code, docs, compilation
+ One for rebooting, running modified kernel, crash

## Technical topics
There is a device framework.

Device tree which has 1 instance of your driver per device

Each driver instance has a global software state which holds any flags/vars that should be shared between functions.

Device ID's are added to a list file, not too disimilar when compared to Linux but certainly not the same. This list file essentially holds a list of all/most devices.

Software interfaces for hardware:
Device register - small unit of memory inside peripheral device. Each register has an address and specific purpose. Contents control/report behavioural aspects of the device.

Examples
+ config reg (config device aspect)
+ control reg (tell device to do something)
+ statius reg (info provided by device)

It's necessary to have register documentation which maps out registers on the device and how to use them. This means that references/data sheets should be used. 

## Thoughts:
OpenBSD definitely seems to be a DIY OS. With advice like "Try to obtain these device cheaply and test them yourself". I'd say it be daunting for a OpenBSD newbie to get involved. 

This seems to be much more in-depth than typical driver documentation offered from other sources. I think I'm going to try and find different docs to work from as I care more a bit more about overall structure and a synopsis of whats available above anything else. At least for this comparison. 

Documentation doesn't seem as official when compared to Linux docs.

This feels like a bit of a totally different world. 