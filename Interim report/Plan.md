# Interim report plan

Between 2.5k and 3k words.

----
## Overview (introduction and overview of project) {30%}
### Device Drivers

#### A brief explanation
+ Control computer peripherals
+ Uses 
+ Hardware
+ etc etc

#### They have a lot of issues
+ Use of an unsafe language in C
	+ Memory unsafety
	+ Related issues
+ Not much evolution/change
	+ History
	+ Linux
		+ little to no change since 2005
		+ LDD3 examples can still be run
		+ Code is exactly the same
	+ Differences between OS drivers
+ There's not a lot of focus on Drivers
	+ Presently unable to find many/any papers within the last 2 years. Latest seems to be 2019?

### Rust
+ History
+ Development
+ Applications - Able to Focus on Systems programming
+ (Fastest high level programming language)
+ Why is it useful in this project

#### Proposition: Developing a Linux driver in Rust
+ Demonstration/Discussion on driver work in C undertaken before and during project.
+ Rust has several beneficial features 
+ Plan is to write a Linux driver in a Rust that controls a peripheral connected to a Raspberry Pi 400. Even if that control is basic.
+ There is previous work in making Rust drivers though it still seems quite young and there are still many options available - though it should be noted that RustForLinux seems to be the 'official' method.

----
## Literature Review {30%} 


----
## Preliminary Work {30%}
### C Drivers
+ Read through some of LDD3
+ Tutorial on YouTube by Karthik M.
+ Able to understand basic layout of a typical Linux driver and how to add device entries
+ Demonstrate C char driver, run commands on its device entry to interact


### Research into Rust 
+ Learning Rust
+ Basics
+ Wrote a previous C program in Rust (I could compare each)
+ Researched and wrote programs to learn about Rust memory management



----
## Current progress and future work {30%}
