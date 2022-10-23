https://youtu.be/79PSagCD_AY


Rust - programming language
1.0 released in 2015
Developed between 2006 and 2015
(so one of the youngest programming languages)

----
Rust underwent several changes especially with regards to features
These different changes in Rust made it feel like a different language through the years

----
Graydon Hoare - original inventor of Rust, Mozilla employee

' I have been writing "a compiled, concurrent, safe systems programming language" for the past four and a half years '

----
Auto - early rust once had type inference

Unsafe allows breaking of rules because the machine is not immutable by default and does not have null pointers. Unsafe is thus important because you need to obey the rules of the machine. 

---
~2010
around 90% of featues 'working' in rough form
around 70% of runtime working

this edition had a garbage collector as they assumed you needed it for mem safety
channels were included, as the devs assumed it was necessary for safety

Rust then becomes a Mozilla project

Compiler was used to alleviate mistakes made by devs/humans - compiler doesn't need to sleep, it won't get tired etc etc

Team and improvements steadily started to grow

Graydon then steps down from the project

----
Cargo - build tool and package manager system
Crates.io - open-source repository of open-source code

toml configuration file for cargo, is where dependencies are added so cargo then automtically handles said dependency. Then downloads dependencies of dependencies which is very convenient within projects. Is very powerful for code re-use within an ecosystem.

---
Rusts community grows an mostly sees members come from C++, scripting languages and functional programming. In some ways, Rust is a mix of all 3.

"A language that was extremely low-level and performant"

----
2015 - Rust releases


---
Tools

Cargo
rustfmt
IDE integration