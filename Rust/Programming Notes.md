![[rust programming language.pdf]]

+ rustup -  toolchain installer/manager 
+ rustc - similar 'gcc' cmd, compiles project

+ ##### Cargo 
	+ is essentially an auto build system that can handle dependencies, git etc. It's set out to resemble and be used similarly to a GitHub repo
	+ The way I view it is quite similar to an IDE, building/running through cargo can provide some stats e.g. compile time
	+ Can auto track dependency versions
	+ `cargo new` - set up a new 'repo'
	+ `cargo build` - to simply compile (--release for release optimisations)
	+ `cargo run` - to compile and run
	+ `cargo check` - verify that code can compile but give no exe
	+ `cargo update` - ignore cargo lock and download latest versions of specs in `Cargo.toml`
	+ `cargo doc --open` - builds all docs provided by depends locally and opens in browser
	+ SemVer (Semantic Versioning) can set limits on which specific crate versions should be used and the range that will be utilised within the program
	+ [crates.io](https://crates.io/) is a registry of oss Rust projects that cargo uses to check, find and use dependencies for your project, it will also downloads dependencies for your dependencies!
	+ Cargo will use the same specific versions when rebuilding projects until otherwise specified to ensure that everything continues to work within the project. These are noted in `Cargo.lock` 

+ ###### Concepts 
	+ `std::io` for input/output
	+ standard lib is also known as the prelude, if something isn't included in the prelude then you must call the lib with 'use' e.g.  `use std::io`
	+ `fn` to declare functions 
	+ create new variables with `let`
		+ vars are immutable by default so  keyword  `mut`  must be used to allow the variable to be repeatedly changed
		+ `=` is used to bind a value to a variable
		+ `new`   in `String::new()`  is used to create a new empty string, it is actually an associated function part of the `String`  type and is commonly used within most types
		+ `let mut guess = String::new()` creates a mutable variable bound to a fresh empty String.
		+ `io::stdin().read_line(&mut guess)` allows for handling of user input
			+ (`std::io::Stdin`  would also work if  `io`  wasn't called)
			+ `read_line`  takes user input from stdin and appends to a string, which is why we need to pass guess, the arg must also be mutable to allow repeated changes
			+ `&`  represents a reference (allows multiple parts of code to access 1 piece of data w/o copying data into memory), references are also immutable by default
		+ `.expect("Failed to read line");`  essentially handles errors based on the state of the `Result` enum (`Ok` or `Err`).  `.expect()` can be called at any possible instance of Result.  `Result`s should likely be always use `expect()` as the compiler warns of unused results. It might be that a possible error isn't handled. `.expect()` will just crash the program with a message rather than fully handle errors cc Exceptions
		+ `{}` used in an output string acts as a placeholder similar to `%d, %c, %s` etc in C and can be used in a similar manner e.g. `println!("x = {} amd y {}", x, y);`
	+ `Ordering` is an enum that has `Less, Greater and Equal`
	+ `cmp` compares 2 vals and can be called on *anything* that can be compared
		+ so  `.cmp()` can be called by a value, you pass the value to compare
	+ `match` is an expression used (in the guessgame example) to match a compared number to select conditions provided by `Ordering`
		+ typically provides pattern matching
		+ can be used like a C switch
		+ match also compares its `cmp` finding against each other 'arm' aka condition in the statement
	+ `i32` 32-bit number
	+ `u32` unsigned 32-bit num
	+ var values can be shadowed with new values. Variable types can essentially be converted between types as you go
	+ `.trim()` clears any white space at start and end of the string and also removes inline chars "\\n \\r" etc.
	+ `.parse()` converts a string to another type
	+ `:` after variable names denotes that the var type will be annotated
		+ the annotation means that rust assumes that similar variables will be of the same type? 
	+   