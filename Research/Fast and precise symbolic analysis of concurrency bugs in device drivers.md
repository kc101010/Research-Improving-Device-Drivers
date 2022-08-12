
## Abstract 
Concurrency errors make drivers hard to develop and debug w/o automated tool support.
WHOOP, automated approach to statically analyse drivers for data races.
+ uses symbolic pairwise lockset analysis
+ detect all potential races in a driver

Helps accelerate CORRAL which is an industrial strength bug finder for concurrenct programs. WHOOP and CORRAL were combined to analyse 16 drivers from Lin4.0 kernel.

## Intro
"complex pieces of system-level software" working between hardware and software to interface between OS and computer devices. 
"Notoriously hard to develop and debug"

A driver retains undiscovered error
(THIS PAPER ALSO USES THE SAME SOURCES AS I DID WITH RMIC   )