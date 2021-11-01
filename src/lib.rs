#![allow(clippy::missing_safety_doc)]

#![cfg_attr(not(feature="std"), no_std, allow(unused_imports))]
#![cfg_attr(feature="unstable", feature(new_uninit))]

#[cfg(feature="alloc")]
extern crate alloc;

mod ext {
    mod io;
    pub use io::*;

    mod uninit;
    pub use uninit::*;

    mod pointer;
    pub use pointer::*;

    mod other;
    pub use other::*;
}

pub use ext::*;

mod raii;
pub use raii::*;

mod cfor;
pub use cfor::*;

mod log;
pub use log::*;
