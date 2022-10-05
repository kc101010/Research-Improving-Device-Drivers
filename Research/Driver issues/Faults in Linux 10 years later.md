# Faults in Linux: 10 Years Later

## Abstract
2001, Chou et al - Empirical study of OS errors
Applied static analyzier to Linux version 1.0 through 2.4.1

"A major result was that drivers directory contained 7 times more of certain kinds of faults than other directories." - this led to a number of development and research efforts on improving the reliability of driver code. 

What has been the impact of these changes on code quality? Are drivers still a major problem?

Repeating Chou et al's experiments for Linux 2.6.0 to 2.6.33 (released between late 2003 and early 2010)

"We find that Linux has more than doubled in size during this person, but that the number of faults per line of code has been decreasing."

Drivers still account for a large part of kernel code and contains most faults but its fault rate is below other directories, such as `arch` (HAL) and `fs` (file systems).

## 1. intro
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

## 2. experimental protocol 
