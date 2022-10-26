["Stop Writing Rust  - No Boilerplate (14/7/2022)"](https://www.youtube.com/watch?v=Z3xPIYHKSoI)
+ ###### Rust is the fastest high level programming language (source: https://github.com/kostya/benchmarks)
	+ Application expands and won't slow down
	+ Likely no need to have a scaling plan
+ ###### Rust code is reliable from the start
	+ Some of the most popular crates have effectively been 'completed' and have not seen updates in some time due to the code having no issues and likely no rot
	+ Rust code will be backwards compatible and old code will always compile on new versions
	+ Old code will also then benefit from optimisations made to the rust toolchain, so code will improve and speed up as the language itself improves
+ ###### Compiler enforces several good practices in code
	+ No unused variables
	+ Exhaustive pattern matching
	+ Correct Concurrency
		+ once a variable is sent to another thread/channel, it's gone and a compiler error occurs if there is an attempt made to read the value
	+ Errors *must* be handled
+ ###### Rust is a productive programming language
	+ Rust is good for fast prototyping (fast, instant feedback)
	+ Type system (also gives 'superpowers?')
	+ Compiler gives clear feedback on errors and how they might/should be solved

https://www.phoronix.com/news/Rust-v8-For-Linux-Kernel
