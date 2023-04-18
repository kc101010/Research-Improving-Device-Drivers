//! Character driver example in Rust
//! Simply uses 'scull' as a name
//! 
//! 
//! 
//!

use core::result::Result::Ok;
use kernel::{file, miscdev};
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::prelude::*;

module! {
    type: Scull,
    name: "scull",
    license: "GPL",

}

struct Scull{
    _dev: Pin<Box<miscdev::Registration<Scull>>>, 
}

#[vtable]
impl file::Operations for Scull{
    fn open(_context: &Self::OpenData,_file: &file::File) -> Result<Self::Data> {
        pr_info!("File was opened\n");
        Ok(())
    }

    fn read(_data: (),_file: &file::File,_writer: &mut impl IoBufferWriter,offset: u64,
    ) -> Result<usize> {
        pr_info!("File was read\n");
        Ok(0)
    }

  
    fn write(_data: (),_file: &file::File,_reader: &mut impl IoBufferReader, _offset: u64,
    ) -> Result<usize> {
        pr_info!("File was written\n");
        Ok(_reader.len())
    }
    
}

impl kernel::Module for Scull {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World\n");
        let _reg = miscdev::Registration::<Scull>::new_pinned(fmt!("scull"), ())?;
        Ok(Scull {_dev:_reg})
    }
}

