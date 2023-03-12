// SPDX-License-Identifier: GPL

//! Rust Mouse

// Written by Kyle Christie
// Developing Device Drivers in Rust
// 2022 - 2023 Computing Honours @ UWS
// Supervisor: Paul Keir
// Moderator:  Stephen Devine


use kernel::prelude::*;

//module descriptor
module!{
    type: RustMouse,
    name: "rust_mouse",
    author: "Kyle Christie",
    description: "Rust Mouse Driver",
    license: "GPL",

}

//struct represents driver
struct RustMouse {

}

//driver functions
impl kernel::Module for RustMouse{

    fn init(_module: &'static ThisModule) -> Result<Self>{
        pr_info!("Hello, World!");

        Ok(RustMouse{})
    }

}

//exit function
impl Drop for RustMouse{

    fn drop(&mut self){
        pr_info!("Goodbye!");
    }

}
