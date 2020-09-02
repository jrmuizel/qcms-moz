#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(stdsimd)]



extern crate libc;



pub mod chain;
pub mod iccread;
pub mod matrix;
pub mod test;
pub mod transform;
pub mod transform_avx;
pub mod transform_sse1;
pub mod transform_sse2;
pub mod transform_util;

