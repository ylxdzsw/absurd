#![allow(clippy::missing_safety_doc)]

#![cfg_attr(not(feature="std"), no_std, allow(unused_imports))]

#[cfg(feature="alloc")]
extern crate alloc;

pub mod ext {
    pub mod io;
    pub use io::*;

    pub mod uninit;
    pub use uninit::*;

    pub mod pointer;
    pub use pointer::*;

    pub mod other;
    pub use other::*;
}

pub use ext::*;

pub mod raii;
pub use raii::*;

pub mod cfor;
pub use cfor::*;

pub mod log;
pub use log::*;
