#![allow(clippy::missing_safety_doc)]

#![cfg_attr(not(feature="std"), no_std, allow(unused_imports))]

mod cfor;

mod ioext;
pub use ioext::*;

mod log;

mod manytimescell;
pub use manytimescell::*;

mod misc;
pub use misc::*;

mod ptrext;
pub use ptrext::*;

mod raii;
pub use raii::*;

mod uninit;
pub use uninit::*;

mod usizetype;
