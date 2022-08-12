
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

