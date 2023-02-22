(Info that is not on slides)

## Device Drivers & Problems (2 min)

Drivers are found within a majority of OS' if not all
Refer to slides
Unfortunately they do suffer from range of issues - one of the most major being the continued use of C

Memory safety: will discuss this more broadly

## Project Aim (1 min)

Original intent was to create a generic USB mouse... it was then found that RFL project is maybe not quite at that stage so the aim has shifted to highlight the benefits for Linux but is not too dissimilar. Will expand on some of this later...

## Rust (3 min)

One of the main components in this project, especially with regard to development, is Rust for Linux. As a bit of background....
+ Builds on groundwork laid by gaynor and thomas in 2019
+ Started 2020
+ Recently incorporated into Linux with kernel version 6.1
+ Development continues, updates provided via Zulip and their own mailing list

Rust is a relatively young system language focused on memory safety
+ Invented by Graydon Hoare, released 2015
+ Several features, attractive to this project
	+ Powerful compiler w/ strong type system, also enforces good code practise
	+ Compiler provides clear feedback to developer
	+ Old code can benefit from new optimisations
	+ Implements Ownership, Borrowing system and lifetimes - ensures all obj references are valid 

... though not perfect, crit by stroustrup
+ Unsafe for Rust is necessary
	+ Allows interaction with system resources
	+ Underlying hardware is not mutable
+ Haskell similarly has backdoors so having a backdoor isn't some kind of gotcha and is sometimes critical

## Memory Safety (3 min)
So memory unsafety can be defined as....
which is why mem safety is so important.

Memory safety vulns aren't as well-defined as other vulnerabilties and are often flexible thus are commonly used as attack surfaces -  google

Left: Table of stats from an article by Alex Gaynor.

The average % of vulns of these code bases is 71%

## The Great Below (5 min)

### Progress
Since the interim report I have....

+ Improved interim report content
+ Dissertation writing, almost halfway through
+ Development has continued but hit a bit of a hurdle that is yet to be resolved

### Development
Detail development steps....
+ Recompiling existing kernel to Linux 6.1
+ Install Rust (if not already installed)
+ Recompiling that kernel to enable Rust
+ After this, tested Rust samples to verify all works

I have found this isn't perfect and you can run into minor problems but generally, the process works.

Demo before ending with questions.

### Findings
2 most interesting?

#### Google
Annual number of memory safety vulns drop from 223 to 85 between 2019 & 2022
Ref slide; 2022 was the first year where the majority of vulnerabilitie aren't related to memory safety

Drop coincides with less memory unsafe code, Android 13 is the first release where most new code is in a mem safe lang

Adoped Rust in Android 12 as alt to C/C++ and, as of the security blog, no vulns in android rust.

Google found
+ Rust allows them to optimise security and system health with less compromises
+ Developers don't need to think about trade-offs

#### Exo-kernel
USENIX ATC '21 - Time for operating systems to rediscover hardware
Some in OS Design & Impl feel that their field has a lot of opportunity but not much interest.
Feel that hardware should be considered mrore carefully - thats cutting it short but is general jist.

An example of this is the exo-kernel. 
Wants to break down hardware abstractions, force as little abstr as possible but still make them available as needed.

Hardware abstr are moved into untrusted libraries with the main goal of ensuring no forced abstraction. 

Drawbacks are less consistency, exokernel interfaces are complex design

Design philosophy: expose hardware as much as possible.

## Development Demonstration (~1 min)

## Underneath it all: Questions (5 mins)

