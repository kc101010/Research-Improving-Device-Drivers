[What science can tell us about C and C++'s security · Alex Gaynor](https://alexgaynor.net/2020/may/27/science-on-memory-unsafety-and-security/)

Gaynor, A. (2022)  "What science can tell us about C and C++'s security" [Accessed 21 December 2022]


A large codebase with millions of lines of code which is written in a memory-unsafe programming language can expect at least 65% of security vulnerabilities to be caused by memory unsafety.

# Results

## Android
use-after-free, double-free and heap buffer overflows generally constitute more than 65% of High & Critical security bugs in Chrome and Android.

## Android bluetooth & media components
use-after-free, integer overflows and out of bounds read/writes comprise 90% of vulnerablities with out of bounds issues being most common.

## iOS and macOS
iOS12 saw Apple fix 261 CVEs, 173 (66.3%) of all vulnerablities related to memory unsafety. 
All of Mojave saw Apple fix 298 CVEs, 213 related to memory unsafety which is 71.5% of all vulnerabilities.

## Chrome
Chromium project finds that 70% of serious security bugs are memory safety problems.

## Microsoft 
Around 70% of Microsoft CVE Vulnerablities each year continue to be memory safety issues.

## Firefox CSS subsystem
73.9% of bugs within this component related to memory unsafety.

## Ubuntu Linux Kernel
65% of CVEs behind the last 6 months of Ubuntu security updates to this kernel have been memory unsafety.

"These numbers are in line with what we've seen in 0days that have been discovered being exploited."

These statistics were observed and reproduced across large code bases, built by different companies, started development at diffferent times and use different development methodologies. The one common property of these codebases is that they are written in a memory-unsafe language such as C or C++. 

Gaynor is prepared to conclude that the use of memory-unsafe languages is bad for security however we know how to eliminate memory-unsafety - by using memory-safe programming languages. This conclusion has not been made and Gaynor would like to remain rational and ensure that the evidence merits the conclusion.

'Unsafe deserialisation' is a potential vulnerability in memory safe languages. Gaynor concludes that the magnitude of memory-unsafe vulnerabilities is higher than memory-safe vulnerablities. "... the empirical research supports the proposition that using memory-safe programming languages for these projects would result in a game-changing reduction in total number of vulnerabilities."

