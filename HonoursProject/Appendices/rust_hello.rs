use kernel::prelude::*;

module!{
    type: RustMouse,
    name: "rust_mouse",
    author: "Kyle Christie",
    description: "Rust Mouse Driver",
    license: "GPL",

}

struct RustMouse {}

impl kernel::Module for RustMouse{
    fn init(_module: &'static ThisModule) -> Result<Self>{
        pr_info!("Hello, World!");
        Ok(RustMouse{})
    }

}

impl Drop for RustMouse{
    fn drop(&mut self){
        pr_info!("Goodbye!");
    }

}
