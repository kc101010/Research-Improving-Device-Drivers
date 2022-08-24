Most device drivers created through kernel modules. 
Kernel space and User space are separated.
Kernel space isn't compartmentalised so for kernel/driver code it is basically the wild west.
Uses a build system to link code and object files to .ko executables.


Most materials that I use for learning Linux modules/device drivers have not been recently created which showcases that Linux drivers have not evolved or developed much in the last decade or two.
+ [This kernel device model page](https://docs.kernel.org/driver-api/driver-model/overview.html) was drafted 26/08/2002 and later updated 31/01/2006. This was first drafted over 1 month before I was born. 
+ [Linux Device Drivers 3 Book](https://lwn.net/Kernel/LDD3/) was published/released in 2005.
+ [This YouTube channel](https://www.youtube.com/channel/UCQ-NwyLyw_-FUQrvXmyW_BA/videos) which I previously  used to learn Linux drivers during college, released these Linux Device Driver training videos in September 2014. 


(Notes taken from LDD3 study)

Linux kernel modules are typically compiled with makefiles that convert the code into an object file which is then linked to a .ko file for the kernel to use. There are various command line tools for working with modules (inserting, removing ). I think most driver work that isn't code-writing occurs through the command line.

One thing to note about Linux modules is that they must be specifically compiled for each/any kernel version supported. So a driver written/compiled for Linux 2.x must be updated for Linux 4.x or 5.x.

Modules/drivers run in kernel space which is separated from user space (desktop software) by the CPU. Faults in kernel space have a high likelyhood of affecting the system itself. 

Linux kernel runs in highest CPU level (supervisor) so everything and anything is allowed.

Linux kernel code is written to run in more than 1 context at the same time, Data structs need to be designed to hold many separate threads of execution. Here, Race Conditions can occur. This code is usually complex. 

Linux driver code has typical init and clear functions but also looks for license infomation *as a requirement*

Supports kernel and user space drivers. User space provides various advantages that can improve development and prevent system faults but there are several drawbacks that make user space drivers worthless. Even the book writes that the best use case for user space drivers is to first test new hardware but eventually transition code into kernel space.