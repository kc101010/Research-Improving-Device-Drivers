
### 1.2 Aims
+ Gain an understanding of the potential issues surrounding device drivers.
+ Investigate further issues within an OS which could stem from device drivers.
+ Discuss the implementation of device drivers across various operating systems.
+ Discuss various developments and technologies which may resolve or alleviate issues with drivers.

### 1.3 Objectives
+ Establish whether drivers continue to act as the leading cause of error within operating systems.
+ Attempt to highlight various improvements which could be made to future driver maintainability.
+ Test that such improvements would continue to be viable in present-day and/or future operating systems.

# What I'd like to do/gain throughout this project

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



# Core Objectives
+ Literature Review
	+ Rust
	+ Linux Driver Development
	+ Research Projects related to Driver dev
+ Develop some kind of Rust driver


# Project Plan

## October
+ Work on literature review - remember, I've already got a decent chunk of papers read through so most vital & relevant ones now.
+ Start organising literature review for interim and final reports.
+ Conduct Research into how Rust drivers are written and produced - I have decent enough know-how on the writing and use of Linux drivers that I can switch to focus on Rust a bit more, it is more relevant to the project.
+ Start work on the interim report - ideally ASAP. Might be best to focus on this after Virtualisation CW1.

## November
+ ***==!! Interim report due 25th @ 23:59.==
	+ Draft to supervisor due 1 week before submission. 
	+ Email backup to Mark
	+ 2,500 - 3000 words
	+ Full details on trello
+ Very quickly after interim is when I should start adding to the dissertation
+ Start driver development at mid-way/end of month (provided Research went well last month)



## December

## January

## February

## March

## April


----
## Research Questions

+ Can applying Rust improve reliability of Linux Device Drivers?
+ Can Rust improve reliability of Linux Device Drivers? 
+ Can Rust be used to write Device Drivers?
+ Driver Development in Rust
Appropriate to Undergrad level.
Don't always need question in title.

### Like
+ How do we use Rust to improve reliability of Linux Device Drivers? / How do we use Rust to improve reliability of Linux Kernel Modules?
+ Would the use of Rust be a sufficient enough improvement in device drivers?
+ Can tools (static analysis etc) be sucessfully applied to improve device drivers?
+ Can moving driver code into user-space improve reliability in device drivers?
+ How do we apply Rust to prevent memory un-safety in Device Drivers?
+ What are the problems facing modern device drivers and how can we solve these?

### Meh
+ How do we improve the reliability of Device Drivers?
+ How can Operating System models/layouts improve device drivers?
+ How do OS Models/Layouts affect device drivers?

### No like
+ Is the problem with device drivers caused by the OS? 

