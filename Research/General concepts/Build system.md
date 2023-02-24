https://medium.com/swlh/a-brief-introduction-to-build-systems-1e45cb1cf667

Zhang, C. (2020) "A Brief Introduction to Build Systems". [Online] The Startup. Medium. Available: https://medium.com/swlh/a-brief-introduction-to-build-systems-1e45cb1cf667 [Accessed 24 February 2023]

'build' - process of converting/translating source code into executable binary files.
'build system' - a collection of software tools used to facilitate the build process

Build systems are commonplace, applied over 30 years without changing much.

Common build systems used presently are as follows:
+ Make
+ GNU Make
+ CMake
+ QMake
+ Ninja




Building refers to the process of converting (or translating) source code into executable binary files. Thus, a build system is "a collection of software tools used to facilitate the build process" (Zhang, 2020). Build systems have been used for over 30 years and have not seen major change. There are several build systems that can be used in the present including: Make, GNU  Make, CMake, QMake and Ninja. 


----
## Makefile

Free Software Foundation, Inc. (2022) "GNU make" 0.76 ed. Ch 1: Overview of make. Ch 2: An Introduction to Makefiles. [Online] Available: https://www.gnu.org/software/make/manual/make.html#Introduction [Accessed 24 February 2023]

"describes the relationships among files in your program and provides commands for updating each file."

+ Makefile controls Make, describing how make should compile and link a program via a written set of rules.
+ Makefile can also describe how make should run various commands (such as a cleanup operation) as part of the build process.
+ Running the makefile typically builds a piece of software and even updates files when new additions are made. 

A makefile controls the build system, 'make', and "describes the relationships among files in your program and provides commands for updating each file." (2022, Free Software Foundation, Inc.). The makefile contains a pre-written set of rules which control how make compiles and links a program. It is also possible to run various external commands via make, a cleanup operation as an example. When the makefile is run, it typically builds a piece of software, even updating specific files when additions are made.