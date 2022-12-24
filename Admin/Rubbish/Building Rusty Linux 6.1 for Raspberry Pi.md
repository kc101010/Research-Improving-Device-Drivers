
# Notes 
+ Running as root/sudo is a necessity.
+ make-specific
	+ `V=1` can be passed as a parameter to make for Verbose output
	+ `-j{NUM_CPU}` can be passed as a parameter to speed up the build process by utilising more CPU cores, use `lscpu` or similar to find this out
	+ `ARCH=` can be passed to specify the architecture to build for 

# Build dependencies
+ Flex - `flex`
+ Bison - `bison`
+ LLD - `lld`
+ Libelf - `libelf-dev`
+ Openssl - `libssl-dev`
+ Clang - `libclang-dev` `clang`
+ LLVM - `llvm-dev`

# Required Programs 
+ Git
+ Make

# Steps

## 1. Cloning the repository
`git clone --depth=1 --branch=rpi-6.1.y https://github.com/raspberrypi/linux.git`

## 2. Prep
First we need to specify which architecture we are compiling for;
`cd linux`
`KERNEL=kernel8`
`make LLVM=1 -j4 bcm2711_defconfig`

## 3. Building
`make LLVM=1 -j4 Image.gz modules dtbs`

Resources used
https://www.raspberrypi.com/documentation/computers/linux_kernel.html