// SPDX-License-Identifier: GPL-2.0

//! Rust /dev/null charactor device

#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{
    chrdev, c_str,
    file::File,
    file_operations::{FileOpener, FileOperations},
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

impl FileOpener<()> for Nulldev {
    fn open(_ctx: &()) -> Result<Self::Wrapper> {
        Ok(Box::try_new(Self)?)
    }
}

impl FileOperations for Nulldev {
    type Wrapper = Box<Self>;

    kernel::declare_file_operations!(read, read_iter, write, write_iter);

    fn read(_this: &Self, _file: &File, _data: &mut impl IoBufferWriter, _offset: u64) -> Result<usize> {
        Ok(0)
    }
    
    fn write(_this: &Self, _file: &File, buf: &mut impl IoBufferReader, _offset: u64) -> Result<usize> {
        Ok(buf.len())
    }
}

struct RustNullDev {
    _chrdev: Pin<Box<chrdev::Registration<1>>>,
}

impl KernelModule for RustNullDev {
    fn init() -> Result<Self> {
        pr_info!("Rust Null Charactor Device (init)\n");

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(c_str!("rust_null_chrdev"), 0, &THIS_MODULE)?;

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
