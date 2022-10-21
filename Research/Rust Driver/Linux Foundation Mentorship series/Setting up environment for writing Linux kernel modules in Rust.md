# [Linux Foundation Mentorship Series: Setting up environment for writing Linux kernel modules in Rust](https://youtu.be/tPs1uRqOnlk)

29/9/22

Goal: make Rust a first-class language for Linux kernel development

Code is from development branch but can be hopefully be used  in mainline kernel.
3 Previous sessions in this series.

Plan
+ Start from fresh OS install
+ Fetch source code
+ Install tools and libraries
+ Build everything 
+ Boot with QEMU
+ Attach GDB
Can also show off rust-analyzer

## Installation

Cloned Rust-for-Linux repo
Cloned mirror/busybox repo
Config the kernel

`make LLVM=1 allnoconfig qemu-busybox-min.config rust.config `

# Running and checking for PRs

`make LLVM=1 -j6 rustfmtcheck` - run to check code against rust formatting
`make LLVM=1 -j6 rustdoc` - run to generate and verify rust documentation, incl user written docs
`make LLVM=1 -j6 CLIPPY=1` - runs linker to verify libs and headers (?)
`make LLVM=1 -j6 rusttest` - runs out-of-kernel tests

Uses Kunit for testing.