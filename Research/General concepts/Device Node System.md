Debian wiki (2020) "Device nodes" [Online] Available: https://wiki.debian.org/device_node

Device Node
+ special file type 
+ facilitates transparent communication between user applications and computer hardware

Device nodes correspond to resources already allocated by  the OS kernel. Such resources are identified by a major number and minor number both of which are stored as part of the structure of a node. Assignment of such numbers may vary between operating systems. 

Usually, the major num ids the device driver while the minor num ids a specific device that the driver can control. These numbers are usually passed to the driver.

https://learn.microsoft.com/en-us/windows-hardware/drivers/gettingstarted/device-nodes-and-device-stacks


https://www.ibm.com/docs/en/aix/7.2?topic=management-device-nodes

+ Devices are organised into clusters known as nodes where each node is a logical subsystem of devices.
+ Hierarchical design where each lower level (child) depends on that higher than itself (parent)