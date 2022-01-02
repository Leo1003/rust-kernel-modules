// SPDX-License-Identifier: GPL-2.0

//! Rust helloworld charactor device

#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use core::cmp::min;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{
    file::File,
    file_operations::{FileOpener, FileOperations},
    io_buffer::IoBufferWriter,
    miscdev,
};

module! {
    type: RustHelloDev,
    name: b"rs_hello",
    author: b"Shao-Fu Chen <shfchen@gmail.com>",
    description: b"An example helloworld charactor device kernel module written in Rust",
    license: b"GPL v2",
}

const HELLO_MSG: &[u8] = b"Hello, world\n";

struct HelloChrdev;

impl FileOpener<()> for HelloChrdev {
    fn open(_ctx: &(), _file: &File) -> Result<Self::Wrapper> {
        pr_info!("rust device was opened!\n");
        Ok(Box::try_new(Self)?)
    }
}

impl FileOperations for HelloChrdev {
    type Wrapper = Box<Self>;

    kernel::declare_file_operations!(read, read_iter);

    fn read(_this: &Self, _file: &File, data: &mut impl IoBufferWriter, offset: u64) -> Result<usize> {
        if offset >= HELLO_MSG.len() as u64 {
            Ok(0)
        } else {
            let len = min(data.len(), HELLO_MSG.len());
            data.write_slice(&HELLO_MSG[0..len])?;
            Ok(len)
        }
    }
}

struct RustHelloDev {
    _chrdev: Pin<Box<miscdev::Registration>>,
}

impl KernelModule for RustHelloDev {
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust Hello Charactor Device (init)\n");

        let dev_reg = miscdev::Registration::new_pinned::<HelloChrdev>(name, None, ())?;

        Ok(RustHelloDev {
            _chrdev: dev_reg,
        })
    }
}

impl Drop for RustHelloDev {
    fn drop(&mut self) {
        pr_info!("Rust Hello Charactor Device (exit)\n");
    }
}

