[Linus Torvalds: Rust will go into Linux 6.1](https://www.zdnet.com/article/linus-torvalds-rust-will-go-into-linux-6-1/)

Rust is being implemented into the Linux kernel.
This is something that's been in the works since the 2020 Linux Plumbers conference where devs started considering Rust for new Linux inline code.

Google supports Rust for Android (a Linux distro) and tried pushing Rust for Linux in April 2021. 

"We feel that Rust is now ready to join C as a practical language for implementing the kernel. It can help us reduce the number of potential bugs and security vulnerabilities in privileged code while playing nicely with the core kernel and preserving its performance characteristics." Wedson Almeida Filho - Googles Anroid Team

Was decided that Rust has enough support in Clang for it to be used in the Linux kernel. 

Also helped Rusts case that Rust on Linux has gotten much more mature while Andreas Hindborg wrote a first-rate driver (SSD NVMe Driver for Linux in Rust) which convinved maintainers that Rust could move forward on Linux. 

This upcoming release will simply contain the core Rust infrastructure which means it will have no serious use case but still opens the door up for using Rust in the Linux kernel. 