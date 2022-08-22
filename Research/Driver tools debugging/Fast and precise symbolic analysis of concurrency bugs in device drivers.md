
## Abstract 
Concurrency errors make drivers hard to develop and debug w/o automated tool support.
WHOOP, automated approach to statically analyse drivers for data races.
+ uses symbolic pairwise lockset analysis
+ detect all potential races in a driver

Helps accelerate CORRAL which is an industrial strength bug finder for concurrenct programs. WHOOP and CORRAL were combined to analyse 16 drivers from Lin4.0 kernel.

## Intro
"complex pieces of system-level software" working between hardware and software to interface between OS and computer devices. 
"Notoriously hard to develop and debug"

A driver retains undiscovered errors after shipping
(THIS PAPER ALSO USES THE SAME SOURCES AS I DID WITH RMIC)

Concurrency bugs account for 10% of total bugs in Linux drivers so they are a significant issue. Most of these bugs were data races or deadlocks in various config functions and hot-plugging handlers.

Concurrency bugs are made worse as drivers operate in a complex and hostile environment (LDD3)
+ OS can invoke drivers from multiple threads
+ Hardware dev can issue int requests that can block running processes & need context switch
+ User may remove device while an operation is still running (aka hotplugging)
These can all cause data races if the right control mechanisms for shared resources aren't in place.

Data races are a source of undefined behaviour in C (LDD3) that leads to bugs that can be hard to narrow down and recreate, isolate and fix especially in OS'.

Sparse, coccinelle, lockdep are able to find deadlocks in kernel source code but can't find data races. Other techniques for race detection aren't entirely sound or precise as they can miss genuine bugs or report false bugs. They also sacrifice precision for scalability. It's necessary that new tools are developed that are more efficient and precise. 

WHOOP is an automated approach for static data race analysis in drivers, using symbolic pairwise lockset analysis. It proves that a drive is race free by
1. Deriving a sound sequential program that over-approximates the original concurrent program
2. instruments the program to record locksets
3. uses the locksets to assert that all accesses to the same shared resource are always protected by a common lock. 

Applying analysis to the sequential program avoids holding a collecting of thread [interleavings](https://www.techopedia.com/definition/5683/interleaving#:~:text=Interleaving%20divides%20memory%20into%20small%20chunks.%20It%20is,overall%20performance%20of%20the%20processor%20and%20system%20increases.) , existing & successful sequential verification techniques can also be re-used.

The guarantees provided by the symbolic analysis can be used to reduce comparisons that then accelarates CORRAL (precise bug-finder by MS to analyse drivers & other concurrent programs). WHOOP and CORRAL were tested on 16 Linux 4.0 drivers, using both led to a analysis speedup between 1.5 to 10 times faster. There were a couple of rare instances of greater speedups of 12 times and 20 times. 

## Background

Modern OS' facilitate responsiveness and performance by providing various sources of concurrency. Several entry points from the same driver can be called concurrenctly, a driver process can block which causes the driver to switch to execution to another process and hardware interrupts can be handled concurrently. These forms of concurrency execution are prone to data races. 

Data Race: occurs when 2 separate threads access a location of shared memory. At least one is a write, at least one is non-atomic (unchangeable). There is no mechanism to prevent simultaneous access. Races are commonly avoided by protecting programs statements that access a shared resource with locks which form critical sections. 

Carelessly using locks also has downfalls. Coarse-grained locking can hurt performance due to limited opportunity for concurrency. Fine-grained locking can easily lead to deadlocks.

#### Lockset Analysis
Lightweight race detection method proposed via Eraser (a dynamic data race detector)

Works by tracking a set of locks consistently used to protect a memory location during program execution. An empty lockset suggests that a memory location might be accessed at the same time by 2 or more threads. Thus, the analysis reports a potential race on that memory location. 

In super basic terms, lockset analysis checks threads and the variables they write to. If these threads can't consistently protect said variables then the tool reports a warning. 

Pros:
+ Easy to implement
+ Lightweight
+ Good potential scalability

Cons:
+ Possible to report false alarms (locking inconsistency will not always lead to a data race)
+ Code coverage is limited by execution paths explored under a given scheduler

To conteract the cons, techniques like Locksmith and RELAY explored applying lockset analysis statically with dataflow analysis. 

#### WHOOP - Symbolic Pairwise Lockset Analysis
Data race analysis in device drivers (Can be applied directly to driver source code)

##### The approach
+ For a given driver, consider each pair of entry points that may execute concurrently. 
+ For each pair, use symbolic verification to check whether it's possible for the pair to race on a shared memory location. 
+ Model effects of additional threads by treating the driver share state abstractly - when an entry point reads from shared state, an [nondeterministic](https://en.wikipedia.org/wiki/Nondeterministic_algorithm) value is returned.

Restriction to pairs of entry points rather than analyzing all entries at the same time exploits the fact that data races happen between pairs of threads and limits complexity of the generated verification conditions. Trade-off is that a quadratic number of entry pairs must be checked.

(Pairwise Analysis also has the advantage of being able to run the analysis for a pair in paralell, for performance reasons.)

