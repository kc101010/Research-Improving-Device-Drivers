Edited 19/8/22
Changes:
+ Change timeframes for project to be slightly more realistic

### Potential questions to work on/cover
+ How can Operating System models/layouts improve device drivers?
+ How do OS Models/Layouts affect device drivers?
+ Is the problem with device drivers caused by the OS?

### 1.2 Aims
+ Gain an understanding of the potential issues surrounding device drivers.
+ Investigate further issues within an OS which could stem from device drivers.
+ Discuss the implementation of device drivers across various operating systems.
+ Discuss various developments and technologies which may resolve or alleviate issues with drivers.

### 1.3 Objectives
+ Establish whether drivers continue to act as the leading cause of error within operating systems.
+ Attempt to highlight various improvements which could be made to future driver maintainability.
+ Test that such improvements would continue to be viable in present-day and/or future operating systems.

#### What I'd like to do/gain throughout this project

#### Pre (Summer - Early Sep)
+ Generate lead-up to project
+ Provide ground work 
+ Write plan, goals
+ Gain basic experience in target langs (Rust/Carbon) for end driver dev

#### Start (Timeframe could be Sep - Nov)
+ Learning experiences with drivers
	+ Gain insight into how the top Operating Systems employ drivers and the differences between each
	+ Just how important is the kernel (lead into abstractions, breaking drivers off into modern languages)
+ Conducting further research
	+ Attempting to focus on research produced between 2010 and present. Or as recent as feasibly possible.
	+ More findings on collateral evolution, is it a real issue that severly effects drivers
+ Find/Research dev sources of drivers i.e. Linux Kernel, MS, Apple
#### Middle (Timeframe could be Nov - Jan)
+ Research what tools are used within driver development
	+ Testing
	+ V&V?
	+ Integration?
	+ Debugging
	+ etc
+ Gain insight into how different companies/communities build drivers
	+ What is a typical SLDC of a driver
	+ Is there a typical driver SDLC
	+ Common practices
+ Gain insight into the knowledge/expertise required to build a sucessful driver
	+ This where questions/findings like "Driver writers specialise in a device over kernel" can asked and tested
	+ This is also where interviews/focus groups/surveys can be employed to inform this question
#### End (Timeframe could be Jan - Submission/End of Year)
+ Using the knowlege gained, attempt to write some sort of simple driver
	+ Rather than write in C/C++ as is tradition, attempt to write in Rust/Carbon in order to create a modern comparison and test if these languages are the way
	+ Involve Arduino?
	+ Write same driver in C/C++ then in Rust/Carbon etc, measure performance
	+ Test Rust driver with Nooks and similar tools to compare error rates/performance
	+ Potential of splitting critical code into C/C++ and non-critical code into Rust/Carbon etc, drawing from Decaf
	+ Look into potential of Carbon in driver use as it aims to hold high interopability with C++
	+ Possibly write my own findings/about my own personal experience, compare to other research/findings?
+ Try to draw conclusions between previous research and my own
	+ Give my own answer to my Aims and Objectives
	+ Provide insight/comparison between findings in 2000's, whatever new research I find and present day
+ Provide loose ground work to continue research into potential masters/personal/work project
+ If present findings cannot at all be found, provide basis for present/modern findings and be able to inform further independant work



----

## Project argument

REFER BACK TO PROJECT PROPOSAL BEFORE FINALISING PLEASE

+ Device Drivers are problematic - why?
	+ Use of programming langs where mem unsafety is easy/common
	+ They haven't changed much (Windows and Linux have not changed since the early 2000s)
	+  It's not an area that has much focus - as far as I see
	+ Developers might struggle to write the right code but they also work in a harash environment
	+ Tools that are worked on/proposed don't seem to actually get much use
+ Here are various tools etc that are trying to fix this
	+ Apples re-structuring kernel exts
	+ Rust
	+ Some other tools etc 
+ Here's what devs think
	+ Jon Blow & Exo-kernels
+ Here's a few reasons how I propose we try to fix drivers
	+ I do think that Rust is part of the way forward
	+ I also think Apples way is another part, likely the best solution so far
	+ I think there also certain tools that could do with widespread usage
	+ Possibly another way forward is using such tools in the same way that app software uses debuggers etc?
+ Here's me doing 1 or 2 of my proposed fixes
	+ Let's write the same driver in Rust & C and see how it goes!
	+ Let's write a driver/extension for Apple and compare that to the C/Rust test.
	+ Let's conduct some tests/analysis etc on the Rust, C and Apple drivers and compare all 3
+ Here's the results of those fixes
	+ How was the experience between the drivers
	+ Compare/measure errors/problems based on tests and analysis
+ Here's what else could be in the running for drivers
	+ Future research
	+ Recent work
	+ My own personal opinion