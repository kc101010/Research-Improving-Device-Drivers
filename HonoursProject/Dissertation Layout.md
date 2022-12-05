# TITLE
### Developing Device Drivers in Rust

----


# ACKNOWLEDGEMENTS
----

# ABSTRACT
---

# TABLE OF CONTENTS
---

# 1. BACKGROUND[c] 10
An introduction to the problem, a brief history and showcase of my plan(s)

## Device Drivers
###  What are they?

### They have a lot of issues 
+ Memory unsafety in C
+ Not much evolution/change
+ Not in the limelight very often

## Rust
+ History
+ Why's
+ More detailed info comes in Literature review

## Here's my proposition of developing a Linux driver in Rust
+ Demonstrate driver work written in C, undertaken before/during project.
+ Link to Rust and it's benefits - its benefits can only make improvements to drivers, right?
+ Plan to write a driver for Linux in Rust that controls a peripheral connected to  Raspberry Pi 400 
+ Potentially try one of the frameworks and compare?
	+ 'Own' method 
		+ `make VERBOSE` on C driver
		+ Take info from that
		+ Try to make Rust driver (maybe by binding C and Rust or vice versa)
	+ Apple claim any language can be used, let's look into things and see if any research or work has been produced where Rust or similar have been used, has it helped? (Should this be more supplementary over being an outright point to make/discuss?)
	+ There is previous work in making drivers for Rust but none of it seems solid or widely adopted so maybe there's other methods that can be explored
		+ Securing embedded drivers
		+ Matias Heiden
		+ Thomas & Gaynor

---


# 2. LITERATURE REVIEW [c] 20
(chasing current research)

## Operating System Drivers
+ Discussion of differences between OS drivers, how they compare and have they changed

## Rust
+ Discuss Rust as a programming language
	+ Improvements over C/C++
	+ More and more peope are calling for Rust to replace C/C++, provide examples
	+ Loose discussion on similar memory safe programming lanuages (needs research)
	+ Discuss Rust frameworks for Linux drivers 

### Catches
+ All safe programming langs provide some kind of unsafe 'loophole' - is this good or bad? Is it a good point by Stroustrup?

### Google, Android 13

### Apple re-structuring of Kernel Extensions

## Memory Safety

## Tools
+ Discuss Dingo framework for drivers
+ Loosely talk about various tools that have came up (WHOOP alongside others used in proposal)

## Miscellaneous efforts
+ Trying to harden C
	+ Only mitigates issues, bugs still possible
	+ Doesn't permanetly solve issue
+ Isolation
	+ Sandboxing
	+ Microkernel
	+ Performance issues
+ C++ wasn't suitable
	+ Rejected by torvalds for use linux
	
----

# 3. DEVELOPMENT [c] 40
----

# 4. EXPERIMENTS [c] 10
### Discuss Results of Rust driver
+ Direct comparison to C
+ Showcase where Rust prevents issues (race conditions etc)
+ Discuss any shortcomings with Rust
+ Short discussion on development experience? (Productivity etc)
----


# CONCLUSION [c] 10
### What is in store for drivers/rust
+ Focusing on Drivers/Kernel etc
	+ Addition of Rust into Linux, what has it done and what will it do?
	+ Anymore progress from Apple/MS?
	+ Are there any solutions/improvements appearing that aren't Rust?
+ Focusing on Rust;
	+ Discuss how Volvo and other automotive companies are carrying out Research regarding Rust.
	+ DRM driver w/ Linux on Apple silicon
----

# REFLECTION [c] 10
problems etc, not always needed

----


# REFERENCES 
----



# APPENDICES


-----









-----





Before you start writing, make sure you know:

-   The word count (and whether you will be penalised for being too many words over or under) [X]
-   Any compulsory sections and the structure required [X]
-   The style of writing required
-   What types of sources are permitted [X]
-   The types of methodology you are allowed to use 
-   The deadline for submission
-   The requirements for submitting your paper copy for marking, such as formatting and binding 
-   Where, when, and how you submit your dissertation [X]
