[Is it time to retire C and C++ for Rust in new programs?](https://www.theregister.com/2022/09/28/is_it_time_to_retire_c/)

Rust has become more popular, [Slashdata counts Rust users as almost tripling in the last 2 years. ](https://slashdata-website-cms.s3.amazonaws.com/sample_reports/VZtJWxZw5Q9NDSAQ.pdf#%5B%7B%22num%22%3A122%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C480%2C540%2C0%5D)

Same story about MS Azure CTO bashing C/C++ for Rust. Writer believes this is due to Rust soon being incorporated into Linux kernel 6.1.

"there are excellent reasons to retire C and C++ in favor of Rust"
"... was designed with performance and safety in mind"
"... C family is all about speed and more speed. Security came a long way second."

It's possible to write safe C/C++ code, use a more secure variant of the language or use more secure programmming guidelines it just that it isn't easy. 
"Both languages make it much too easy to make memory errors."

"Memory errors occur very commonly in C and C++ applications and ... can be hard to reproduce, hard to debug and potentially expensive to correct as well" - Naveen Gv, Intel technical consulting engineer

Both languages are memory-unsafe but give developers fine-grained control of applications memory. "One memory snowball slip-up can lead to an avalanche of errors."

Rust, is a memory-safe language. It's still possible to make security blunders but its harder to make simple memory missteps that plague C and C++ applications. It's also easier to write concurrent programs in Rust.

It will take years and decades but Rust (or another language will take over from C/C++. It won't be possible to ignore security forever.