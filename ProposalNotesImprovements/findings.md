## Research Proposal findings
### Research
+ Little or lack of research
+ Little or lack of innovation or consideration for alt solutions

### Metrics 
+ Drivers are the most likely cause of computer crashes
	+ Have a higher error rate than rest of kernel
	+ 85% of crashes in Win XP
	+ OpenBSD had higher rate of error than Linux Kernel (range of 1.2 to 6x higher)
+ Drivers typically account for largest amount of code within an OS
	+ Linux kernel: 5 mill lines, 70% of src code

### Devs
+ Device drivers are often complex to program
	+ API's are easily misused, which leads to strange behaviour/crashing
+ Driver programmers are typically more familiar with the target hardware than target OS
	+ Little knowledge/experience in kernel organisation and programming
	+ Successfully writing a well-made driver requires expertise/understanding of driver interaction with OS/Kernel
+ More difficult to test
	+ Traditional testing isn't suitable for both common and subtle low level errors

### Code 
+ Collateral Evolution
	+ Services/Functionalities/Libraries utilised by drivers are changed which requires the driver itself to be changed to restore original behaviour
	+ This doesn't improve code and accounts for 35% of changed driver code
	+ Requires code re-organisation, time, expense which makes code more error prone
	+ Linux: Issue is exacerbated due to nature of disseminating information to maintainers/devs
	+ 'Coccinelle' : tool to assist with collateral evolutions
+ OS dependency
	+ Kernel supplies support libs, access to data structs, functions
	+ 51% of driver code dedicated to init, clean up & config
	+ Code repetition: 8% if driver code is largely similar to code elsewhere
+ Tech
	+ Drivers still typically written in C, which isn't a massive change from early days of Unix
	+ Not much change/evolution in driver development itself

### Solutions
+ OOP
	+ Inheritance etc can reduce amount of code used for init
	+ Lead to use of C++, Java, Rust (possibly Kotlin or even Python)
	+ Classes can simply API implementation and/or their components
	+ Inheritance & Polymorphism can be used to reduce volume of repeating code and may simplify the process of adding to/extending APIs
	+ Problem: C is used as it provides the low-level syscalls that let driver and kernel interact. This low-level functionality would obviously need to be retained in any new OOP language used
+ OS Subsystem
	+ Nooks OS
		+ extensions run in small, light sub-kernels that have limited write ability to kernel itself
		+ provides auto recovery to track/validate changes in kernel data structs; preventing bugs in real time 
		+ Idea is that system should become more tolerante of failures
		+ low to moderate impact on driver performance
+ Automatic tools
	+ Auto tools can find errors with higher reliability & accuracy than manual checks
	+ Windows SDV
		+ ID's where API rules are broken within C programs
		+ Applied on driver to verify use of APIs
		+ Breaks C code down into a format that retains errors and can be checked more efficiently
		+ Tool checks errors in C code against the broken down code to verify existence of errors
+ Use of Modern prog langs
	+ Decaf
		+ System that converts Linux kernel drivers from C to Java. Performance critical code is left in C, remaining code runs through custom Java kernel interface
		+ Using type-safe languages such as Java will improve reliability through simplicity. It will also provide use of exceptions and debugging tools
		+ Can move large amount of driver code outside the kernel
		+ Reduce size of driver code
		+ Can evolve alongside changes to driver/kernel code and related data structs
		+ Performs within 1% of native kernel-only drivers
		+ Gradual migration away from C
		+ Java exceptions reduced amount of code and fixed 28 cases of missing error handling
		+ Updates to driver only required changes to Java code
		+ Potentially any language could be used for such a system