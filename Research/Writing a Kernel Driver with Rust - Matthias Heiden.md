["Writing a Kernel Driver with Rust" - Matthias Heiden, October 2020)](https://www.youtube.com/watch?v=wREGR7QQHco)
["Blog: Writing a kernel driver with Rust - Matthias Heiden, 15 July 2020"](https://not-matthias.github.io/posts/kernel-driver-with-rust/)
["Github: kernel-driver-with-rust by not-matthias"](https://github.com/not-matthias/kernel-driver-with-rust)
+ Writing rust drivers for Windows
+ driver_entry() function similar to driver_init()
+ Kernel drivers are somewhat similar to DLLs
+ Reserve use of dylib for larger drivers
+ Need to change linker settings and tell driver where to find/call Kernel APIs
+ Rust vs C/C++
	+ Rust is a little more efficient in how functions are written, e.g. providing structures to work with processes
+ Write own allocator for allocating memory within kernel
+ rust-bindgen
	+ auto generate rust bindings to C/C++
+ Standard library is unavailable while operating in the kernel
+ ##### Cons of using rust with kernel driver
	+ No official bindings, Tedious work especially if writing own
	+ bindgen isn't perfect, isn't a great translation between Rust/C,C++
	+ unicode strings, having no util module makes handling these very tedious
	+ raw ptrs, can lead to vulns and crashes
+ ##### Pros 
	+ Crashes less, provides more speed as less time is spent on crash screen/recovering/restarting
	+ Custom allocator for using content from extended library, very big point
	+ Drop, don't need to worry about deallocation
	+ Option/Result, normal null pointers
	+ crates.io and docs.rs, rust docs/packages 
+ ##### QnA
	+ There are some winapi aspects that are available in both user and kernel mode
	+ MS-related low-level system research? Not in relation to Kernel but want to use Rust more alongside C++
	+ How was the kernel driver tested? Manually/Scripted. IDE started as Admin. Windows has a special test mode. Debugview tool. Tested within a VM. There are kernel debuggers but they aren't built into the IDE. More work than typical user app. 
	+ Is there any way of conducting automated/unit testing? Couldn't find a simply way that wasn't implementing by himself
	+ Many crashes/BSODs during development. 
	