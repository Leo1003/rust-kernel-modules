// SPDX-License-Identifier: GPL-2.0

//! Rust helloworld charactor device

#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use core::cmp::min;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{
    chrdev, c_str,
    file::File,
    file_operations::{FileOpener, FileOperations},
    io_buffer::IoBufferWriter,
};

module! {
    type: RustHelloDev,
    name: b"rs_hello",
    author: b"Leo Chen <leo881003@gmail.com>",
    description: b"An example helloworld device kernel module written in Rust",
    license: b"GPL v2",
}

const HELLO_MSG: &[u8] = b"Hello, world\n";

struct HelloChrdev;

impl FileOpener<()> for HelloChrdev {
    fn open(_ctx: &()) -> Result<Self::Wrapper> {
        pr_info!("rust device was opened!\n");
        Ok(Box::try_new(Self)?)
    }
}

impl FileOperations for HelloChrdev {
    type Wrapper = Box<Self>;

    kernel::declare_file_operations!(read);

    fn read(_this: &Self, _file: &File, data: &mut impl IoBufferWriter, offset: u64) -> Result<usize> {
        if offset > HELLO_MSG.len() as u64 {
            Ok(0)
        } else {
            let len = min(data.len(), HELLO_MSG.len());
            data.write_slice(&HELLO_MSG[0..len])?;
            Ok(len)
        }
    }
}

struct RustHelloDev {
    _chrdev: Pin<Box<chrdev::Registration<2>>>,
}

impl KernelModule for RustHelloDev {
    fn init() -> Result<Self> {
        pr_info!("Rust Hello Charactor Device (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(c_str!("rust_hello_chrdev"), 0, &THIS_MODULE)?;
        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<HelloChrdev>()?;
        chrdev_reg.as_mut().register::<HelloChrdev>()?;

        Ok(RustHelloDev {
            _chrdev: chrdev_reg,
        })
    }
}

impl Drop for RustHelloDev {
    fn drop(&mut self) {
        pr_info!("Rust Hello Charactor Device (exit)\n");
    }
}
