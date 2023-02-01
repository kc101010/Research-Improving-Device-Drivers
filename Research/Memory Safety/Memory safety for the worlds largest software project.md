Ojeda, M. (2022) "Memory Safety for the World's Largest Software Project". [Online] Internet Security Research Group. Available: https://www.memorysafety.org/blog/memory-safety-in-linux-kernel/ [Accessed 1 February 2023]


## Rust for Linux & Prossimo
+ project aims to introduce a new system programming language into the Linux kernel.
+ Rusts key property: guarantees no undefined behaviour takes place (as long as unsafe code is sound) - particularly with memory management
+ So no use-after-free issues, no double frees, no data races

Prossimo - Internet Security Research Group project
+ Goal: Improve internets security-sensitive software infrastructure by addressing memory safety issues in C and C++ by using memory safe languages.
+ A critical example lies in the Linux kernel, which is widely used.

## Origins of Rust and Kernel
There has always been want to develop Linux kernel code in Rust with several attempts. Earliest known attempt was by Taesoo Kim in 2013, before Rust 1.0 was released. 

Rust for Linux was created to provide Rust support within the kernel itself, work occurred on the project in 2020. 

August 2020, "Barriers to in-tree Rust" talk at the Linux Plumbers Conference.

Shortly after that presentation, initial Rust support was submitted to the kernel which included Kbuild integration, initial built-in module support and the start of the `kernel` crate which contained abstractions from Alex Gaynor and Geoffrey Thomas. 

Eventually Wedson Almeida Filho from Google joined, currently a maintainer and main contributor to abstractions and drivers. Next, the Internet Security Research Group offered full time support to the project with a year of funding from Google. 

## Progress in 2022
Various technical achievements

Several industry entities reached out with interest in the project including Google, Arm, Microsoft, Red Hat alongside private companies. Also been in contact with several academics including researchers at the University of Washington who published ["An incremental path towards a safer OS kernel"](https://sigops.org/s/conferences/hotos/2021/papers/hotos21-s09-li.pdf)

Project was presented in various ways/venues, various smmits and meetups.

## Looking to 2023
Several milestones with hopes to achieve:
+ More users/use cases inside kernel - incl. example drivers
+ Splitting kernel crate and managing dependcies to  permit better development
+ Extend current integration of kernel documentation, testing, other tools
+ Find more subsystem maintainers, companies and researchers to involve. 
+ Stabilisation of most remaining Rust features (I assume that are used within the project)
+ Merging into the mainline kernel.
