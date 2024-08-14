#![doc = include_str!("../readme.md")]

#[cfg(all(feature = "dynlink", feature = "staticlink"))]
compile_error!("Must have one of `dynlink` or `staticlink`, cannot have both.");

#[cfg(all(not(feature = "dynlink"), not(feature = "staticlink")))]
compile_error!("Must have one of `dynlink` or `staticlink`, cannot have none.");

pub(crate) mod lib {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod ljm;

pub use ljm::*;
