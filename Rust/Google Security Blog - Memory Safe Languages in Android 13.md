Vander Stoep, J. (2022) "Memory Safe Languages in Android 13". Google Security Blog. Google. [Online] Available: https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html [Accessed 2 December 2022]

+ mem safety vulns account for more than 65% of vulns across products and across industry
+ Android has seen a significant drop in mem safety vulns and an associated drop in vuln severity
+ Between 2019 and 2022, the annual num of mem safety vulns dropped from 223 to 85
+ Drop coincides with moving away from memory unsafe languages
+ "Android 13 is the first Android release where a majority of new code added to the release is in a memory safe language"

![[BarChartVulns.png]]


![[PieChartLangs.png]]

+ Amount of new memory-unsafe code has decreased, so has the number of mem safety vulnerabilities
+ Between 2019 and 2022, this number has dropped from 76% to 35% of Androids total vulnerabilities
+ 2022 - first year where mem safety vulns aren't a majority of Androids vulnerabilities


![[NewSafeCodeVsVulns.png]]

+ % of vulns caused by mem safety issues looks to correlate fairly closely with the language used for new code

+ Google has invested in tools to improve C/C++ safety and increased fuzzing coverage on existing code base such tools are critically important for the respective code
+ These tools alone don't account for the large shift in vulnerabilities, other projects that don't use these technologies haven't seen amajor shift in vulnerability composition - "We believe Androids ongoing shift from memory-unsafe to memory-safe languages is a major factor"

+ Rust was announced in Android 12 as an alt to C/C++
+ Goal is to shift development of new code to memory safe languages over time (rather than convert existing C/C++ code to Rust)

Android 13
+ 21% of all new native code is in Rust
+ Approx 1.5million total lines of Rust in AOSP across a handful of new features

![[NewNativeCode.png]]

## Security impact
"To date, there have been 0 memory safety vulnerabilities discovered in Androids rust code"

This isn't expected to remain 0 but is a significant result (especially given the volume of code and Rusts use in security-sensitive components). So Rust is fulfilling its intended purpose of preventing Androids most common source of vulnerabilities

'it's likely that using Rust has already prevented hundreds of vulnerabilities from reaching production'

## Unsafe Rust
An escape hatch is required in memory-safe languages in order to carry out Systems programming as OS development 'requires accessing resources that the compiler can't reason about.'

In Java, Android uses JNI to access low-level resources and care must be taken to avoid creating unsafe behaviour. 
+ It is much more simpler to review small snippets of C/C++ for safety over entire programs

Rust
+ unsafe escape hatch for interacting with system resources non-Rust code
+ '... Rust code is proving to be significantly safer than pure C/C++ implementations'

Use of unsafe in Android rust code appears to be working as intended
+ Used rarely
+ When used, is encapsulating behaviour which is easier to reason about and review for safety

Safety measures slow memory-unsafe languages
+ "Using memory unsafe code often means that we have to make tradeoffs between security and performance" i.e. adding sandboxing, sanitizers, runtime mitigations, hardware protections
+ However these negatively impact code size, memory and performance
+ Use of Rust allows optimisation of both security and system health with fewer compromises

## Non-memory-safety vulnerablities
2022,mem safe vulns represent 36% of vulnerabilities in security bulletin
+ mem safe vulns accounted for 86% of critical severity security vulns
+ 89% of remotely exploitable vulns


Generally, vulnerablities have a well defined scope of impact though Memory safety vulns are much more versatile. Obtaining code execution in a process grants access not only to the specific resource but everything that the processs can access which includes an attack surface to other processes. "Memory safety vulnerabilities are often flexible enough to allow chaining multiple vulnerabilities together"

The majority of exploits chains used by google use one or more memory safety vulnerability.

A decrease in the most severe vulns has seen an increase in less severe vuln types.
+ Around 15% of 2022 vulns are DoS vulnerabilities
+ This represents a drop in security risk


## Conclusion

+ 'Rust use is growing in the Android platform'
+ Google is looking to use Rust anywhere in the codebase where native code is required
+ With Linux 6.1 support for Rust, Google is starting to apply Rust in kernel drivers
+ Android is moving from C/C++ to Java/Kotlin/Rust and the num of mem safe vulns are expected to fall