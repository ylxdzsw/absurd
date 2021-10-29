#![allow(clippy::missing_safety_doc)]
#![no_std]

#[cfg(feature="std")]
extern crate std;

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


// mod structs;
// mod syntax;
// mod sys;
// mod terminal;
// mod utils;

// pub use structs::*;
// pub use syntax::*;
// pub use sys::*;

// pub trait UnwrapUnchecked {
//     type O;
//     unsafe fn unwrap_unchecked(self) -> Self::O;
// }
// pub use utils::UnwrapUnchecked;
// impl<T, E> UnwrapUnchecked for Result<T, E> {
//     type O = T;
//     unsafe fn unwrap_unchecked(self) -> T {
//         self.unwrap_or_else(|_| core::hint::unreachable_unchecked())
//     }
// }
// impl<T> UnwrapUnchecked for Option<T> {
//     type O = T;
//     unsafe fn unwrap_unchecked(self) -> T {
//         self.unwrap_or_else(|| core::hint::unreachable_unchecked())
//     }
// }

// pub use terminal::*;
