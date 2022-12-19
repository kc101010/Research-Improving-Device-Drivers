Li, A. (2022) "Google reports decline in Android memory safety vulnerabilities as Rust usage grows". 9to5Google [Online] Available: https://9to5google.com/2022/12/01/android-memory-safety-rust/ [Accessed 2 December 2022]

Google - Android Open Source Project (AOSP) support fo Rust

Number of annual memory safety vulnerabilities fell from 223 to 85 between 2019 and 2022
Were 76% of Androids total vulnerabilities and are now 35% 

"2022 is the first year where memory safety vulnerabilities do not represent a majority of Androids vulnerabilities."

Android 13
+ First release with majority of new code added to release is in a mem safe language
+ Rust encapsulates 21% of all new native code within various features
+ "zero memory safety vulnerabilities discovered in Androids Rust code"

Googles future plans
+ Moving away from C/C++ is difficult but there is progress
+ To meet goals in improving security, stability, quality Rust needs to be able to be used anywhere that native code is required
+ Implementing userspace HALs in Rust
+ Adding support for Rust in trusted applications
+ Migrated VM firmware in AVF (Android Virtualisation) to Rust



Thoughts:
+ Should track down and use articles source(s)
+ Article has very, very good figures (Which I assume are taken from original source doc)