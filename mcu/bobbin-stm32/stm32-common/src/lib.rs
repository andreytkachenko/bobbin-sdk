#![no_std]
#![allow(unused_unsafe)]
#![feature(use_extern_macros)]

pub extern crate bobbin_cortexm;
pub extern crate bobbin_common;

pub mod periph;
pub mod ext;

pub use bobbin_cortexm::*;
pub use ext::*;