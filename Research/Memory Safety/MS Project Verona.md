https://www.microsoft.com/en-us/research/project/project-verona/
"a programming language for the modern cloud"

+ Software vulnerabilities can quickly erode the trust that is necessary in order to adopt cloud technology.
+ There is a push in the software industry that aims to stop memory safety vulnerabilities.
+ Project verona is one such project that aims to achieve the above goal for cloud infrastructure.
+ Combined research in compilers, programming language semantics and type systems.
+ Aims to provide secure foundations for placing trust in the cloud.

+ Collaboration between Microsoft Research and Imperial College London.
+ Research revolves around language and runtime design for safe scalable memory management and compartmentalisation.

Core research questions
+ If we design a language without concurrent mutation, can we build scalable memory management?
+ Can linear regions be used to remove the restrictions of per-object linearity without sacrificing memory management?
+ Can language level regions be used to support compartmentilsations?



# Microsoft: Here's why we love programming language Rust and kicked off Project Verona

https://www.zdnet.com/article/microsoft-heres-why-we-love-programming-language-rust-and-kicked-off-project-verona/


Tung, L. (2020) "Microsoft: Here's why we love programming language Rust and kicked off Project Verona". [Online] ZDNet. [Accessed 23 December 2022] Available: https://www.zdnet.com/article/microsoft-heres-why-we-love-programming-language-rust-and-kicked-off-project-verona/

+ MS trying to eliminate memory-focused bugs that may appear in software written in unsafe languages i.e. C++
+ MS Rust Expert, Ryan Levick; Such bugs are expensive to fix and are more common when implementing Patches

+ Systems languages include;
	+ C++ (not memory safe)
	+ Google Go (memory safe)
	+ Mozilla Rust (memory safe)
There are other memory safe languages, such as Swift, Kotlin, Java but these can't be used for Systems programming.

+ MS still writes a lot of its software in C++ and C, they have 'reached a wall' in terms of addressing memory issues.

+ "It's becoming harder and harder and more costly to address these issues over time" -  Levick
+ "70% of our very serious, mission-critical bugs that happen in our software deal with memory safety and incorrect usage of memory."

Project verona
+ New, safe infrastructure programming language
+ Developed by MS research
+ Influenced by Rust, supported by C# lead maintainer Mads Torgersen

+ Levick believes that the best alternative to C++ is Rust.
+ "... in theory, Microsoft could get rid of 70% of its most serious bugs, which are also expensive to fix."
+  'While Rust has a steep learning curve, the Azure team being Krustlet preferred Rust over Go because Rust was able to pick up bugs that Go could not.'



