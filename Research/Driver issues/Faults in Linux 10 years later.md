# Faults in Linux: 10 Years Later

## Abstract
2001, Chou et al - Empirical study of OS errors
Applied static analyzier to Linux version 1.0 through 2.4.1

"A major result was that drivers directory contained 7 times more of certain kinds of faults than other directories." - this led to a number of development and research efforts on improving the reliability of driver code. 

What has been the impact of these changes on code quality? Are drivers still a major problem?

Repeating Chou et al's experiments for Linux 2.6.0 to 2.6.33 (released between late 2003 and early 2010)

"We find that Linux has more than doubled in size during this person, but that the number of faults per line of code has been decreasing."

Drivers still account for a large part of kernel code and contains most faults but its fault rate is below other directories, such as `arch` (HAL) and `fs` (file systems).

## 1. Intro
Linux widely used ranging from embedded, to PCs to supercomputers.

OS w/ monolithic kernel.

Linux controls hardware so its correctness is essential. 

It's important to be able to continually asses and control code quality as Linux is open source and has a large developer base.

Chou et al have been cited over 360 times according to Google scholar. Been followed by development of a series of strategies for automatically finding faults in systems code. 

Statistics have been used for a variety of purposes including providing evidence that driver code is unreliable and and evidence that certain OS subsystems are more reliable than others. 

( Papers to potentially follow up on, mentioned in this paper;
+ Depoutovich and Stumm. Otherworld - giving applications a chance to survive OS kernel crashes. 2010.
+ Herder, Bos, Gras, Homburg, Tanenbaum. Fault isolation in device drivers. 2009.
+ Swift, Annamali, Bershad, Levy. Recovering device drivers. 2006.
+ Lawall. Brunel. Hanse. Stuart. Muller. Palix. WYSIWIB: A declarative approach to finding protocols and bugs in Linux code. (2009)
+ Lu. Park. Seo. Zhou. Learning from mitakes: A comprehensive study on real world concurrency bug characteristics. (2008)
+ 
+ 
)

At the time, Linux was young. At the time of this report, Linux is available to a broader audience and some aspects have since changed. 'The development model has also changed substantially'.

A number of fault finding tools targeting Linux code have been developed. Patches were regularly submitted for faults found using checkpatch, Coccinelle, Coverity, smatch and sparse

Herodotos, tracking faults across multiple versions of a software project.


**Contributions**
The faults considered 10 years ago by Chou et al are still relevant because such faults are still being introduced and fixed in both new and existing files. 

The rate of introduction of such faults continues to rise although the rate of their elimination is rising slightly faster resulting in a kernel that is becoming more reliable with respect to these kinds of faults. This in constrast to previous results for earlier versions of Linux that found that the number of faults was rising with the code size. 

The rate of the considered kinds of faults is falling in the `drivers` directory, suggesting Chou et als and others work has succeeded in directing attention to driver code. `arch` and `fs` now show higher fault rate and thus it may be worthwhile to direct research efforts to the problems of such code.

Found that fault kinds are more likely to have visible impact during execution have a much shorter average lifespan, as little as 1 year.

Fault finding tools are now regularly used in Linux development but seem to only have a small impact on the faults considered in this paper. So research is needed on how such tools can be better integrated into the development process. Proposes that approaches are also needed to automate the fixing of faults, not just the fault finding process.

## 2. Experimental protocol 
Lab sciences; experimental protocol - giving all info required to reproduce an experiment.

### 2.1 Fault finding checkers
(need to come back to section 2 )

## 3. Evolution of linux
Evolution in code size of Linux kernel betwen V1.0, 1994 and V2.6.33, 2010. 

Code sizes computed using David A Wheelers SLOCCount (v2.26) and include only the ANSI C code. 

![[Figure 1 Linux Dir Sizes.png]]

Figure 1 shows that in 1994, Linux had less than 1 million lines of code. After many updates over 16 years, the Linux kernel eventually reached around 8 million lines of code. In the graph, Drivers without staging are the largest contributor to code volume. 

[Without staging](https://lwn.net/Articles/324279/) means that the drivers are already included and deployed within the main Linux Kernel. With staging means that the drivers are part of the Linux Staging tree and aren't included in the main Linux Kernel as they are "not yet mature enough to  be used by end users.". So the largest contributor to code between 1994 and 2010 was drivers that are included with the main Linux kernel or standard Linux drivers.

Drivers includes the Staging Directory (drivers/staging) which "has made up 57% of the source code since Linux 2.6.30."

"... changes have resulted in code growth from 2(Million Lines of Code) in 2001 to more than 8(Million Lines of Code) in 2021"


## 4. Linux 2.4.1
Latest version considered by Chou et al.

### 4.1 What code is analyzed?
Chou et al focused only on x86 code - finding that 70% of Linux 2.4.1 code is dedicated to drivers (though we don't know which drivers, filesystems and so on were included.)

Authors considered 3 possibiities when using SLOCCount.
1.  All code in kernel source tree (All code)
2. Set `.c` files compiled while using the default x86 configuration (Min x86)
3. All 2.4.1 code except `arch` and `include` subdirs that are specific to non-x86 architectures (Max x86)

Authors found that that Max x86 option gave a result closes to the finding by Chou et al though the proportion of driver code is slightly higher than 70%. They found it was reasonable as some driver code is associated with specific architecures and can't be compiled for x86. This also demonstrates that we don't know the precise set of files used in Chou et als tests.

In these new experiments, authors considered the entire kernel source as "every line of code can be assumed to be relevant to some user".

### 4.2 How many faults are there? 
Authors checkers obtained 600 reports which represent 467 faults with the remainder representing false positives. Chou et als checkers obtained 1,025 faults in Linux 2.4.1 with 602 faults with remainder characterised as low false positive checkers.  

Upon comparing checkers from own results and chou et als results, authors found that
+ In most cases, they found fewer faults (possibly due to differing definitions/criteria)
	+ Fewer IsNull than INull
	+ Fewre faults in var
+ In some cases, they find more faults.
	+ Far more NullRef faults
	+ Slighly more Free faults

### 4.3 Where are the faults?
Chou et al found largest number of faults are in drivers and that the largest number of these faults are of categories Block, Null and Inull. Authors also observed largest number of faults in drivers directory with largest number of these also being in BlockLock, Null and Inull (though these were in different portions).

Widely cited results, drivers dir contains almost 7 times as many lock faults as all other directories combined. Authors find that drivers have over 8 times as many lock faults.

Authors also found that drivers has 8 times more Free faults than all other directories combined. Chou et al only found a fault rate of 1.75 times in this case.

In both papers, absolute number of Free faults is rather small.

Both papers observed a high fault rate in `arch` for *Null* of around 4.8 times of all other directories combined.

Authors - unlike Chou et al - found a high rate of Var faults in `arch` and `other`. Regarding `arch`,  all Var faults found are for architectures other than x86.

