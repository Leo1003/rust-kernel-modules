// SPDX-License-Identifier: GPL-2.0

//! Rust /dev/null charactor device

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{
    chrdev, c_str,
    file::{File, Operations},
    io_buffer::{IoBufferReader, IoBufferWriter},
};

module! {
    type: RustNullDev,
    name: b"rs_null",
    author: b"Leo Chen <leo881003@gmail.com>",
    description: b"A /dev/null device written in Rust",
    license: b"GPL v2",
}

struct Nulldev;

impl Operations for Nulldev {
    kernel::declare_file_operations!(read, read_iter, write, write_iter);

    fn open(_shared: &(), _file: &File) -> Result {
        Ok(())
    }

    fn read(_this: (), _file: &File, _data: &mut impl IoBufferWriter, _offset: u64) -> Result<usize> {
        Ok(0)
    }
    
    fn write(_this: (), _file: &File, buf: &mut impl IoBufferReader, _offset: u64) -> Result<usize> {
        Ok(buf.len())
    }
}

struct RustNullDev {
    _chrdev: Pin<Box<chrdev::Registration<1>>>,
}

impl kernel::Module for RustNullDev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust Null Charactor Device (init)\n");

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(name, 0, module)?;

        chrdev_reg.as_mut().register::<Nulldev>()?;

        Ok(RustNullDev {
            _chrdev: chrdev_reg,
        })
    }
}

impl Drop for RustNullDev {
    fn drop(&mut self) {
        pr_info!("Rust Null Charactor Device (exit)\n");
    }
}
