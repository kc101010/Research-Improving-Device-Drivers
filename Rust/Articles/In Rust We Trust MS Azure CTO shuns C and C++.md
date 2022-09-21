[In Rust We Trust: Microsoft Azure CTO shuns C and C++](https://www.theregister.com/2022/09/20/rust_microsoft_c/)

+ MS Azure CTO, Mark Russinovich 'urged the technology industry to leave C/C++ behind.'

[Mark Russinovich: "Speaking of languages, it's time to halt starting any new projects in C/C++ and use Rust for those scenarios where a non-(garbage collection) language is required. For the sake of security and reliability. the industry should declare those languages as deprecated." ](https://twitter.com/markrussinovich/status/1571995117233504257?ref_src=twsrc%5Etfw%7Ctwcamp%5Etweetembed%7Ctwterm%5E1571995117233504257%7Ctwgr%5Ef12bc042ea564a7fb69ba9eac68185f23d7e87d0%7Ctwcon%5Es1_c10&ref_url=https%3A%2F%2Fwww.theregister.com%2F2022%2F09%2F20%2Frust_microsoft_c%2F)


+ Russinovichs C/C++ disimissal comes as Torvalds has reportedly confirmed that Rust code will appear in Linux kernel 6.1.
+ Rust has  been the most loved programming language in annual StackOverflow survey for a consistent 7 years even though it has a reputation for being difficult to learn.s
+ It's been integrated into projects at major technology companies with Apple, Amazon, Google, Meta and Microsoft using Rust in some capacity or in production
+ Cloudfare - HTTP proxy built using Rust, boosted performance reduced CPU and memory usage
+ "Rust seems less prone to potential memory corruption bugs and this makes software less vulnerable"
+ MS has discussed using Rust over C/C++ since 2019 and has been working on Project Verona, a cloud-oriented memory safe programming language.
+ [MS - 70% of patched CVEs since 2006 are due to memory safety issues.](https://github.com/Microsoft/MSRC-Security-Research/blob/master/presentations/2019_02_BlueHatIL/2019_01%20-%20BlueHatIL%20-%20Trends%2C%20challenge%2C%20and%20shifts%20in%20software%20vulnerability%20mitigation.pdf)

"Rust alone will not guarantee sotware is secure. It provides a defense against memory safety bugs but does not eliminate other classes of vulnerabilities."

It's noted in the documentation itself "Rust contains both a safe and unsafe programming language" which the article says "... they may crate unsafe code unintetionally". Rust also doesn't attack vectors such as Social Engineering but this would surely be outside the scope of a programming language?

Bjarne stroustrup response
+ Can now achieve perfect type and memory safety in ISO C++.
+ "Note that every 'safe' language, including Rust, has loopholes allowing unsafe code."
+ [Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines) - set of rules to be followed to guarantee safety and then enforce them with static analysis. Rules are needed as arbitrary C/C++ code can't be proven safe.
+ Replacing or making safe C++ code is a massive task that must be performed gradually or the large amount of unsafe C and old-style C++ code will remain forever.