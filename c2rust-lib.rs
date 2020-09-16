#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(stdsimd)]
// These are needed for the neon intrinsics implementation
// and can be removed once the MSRV is high enough
#![feature(platform_intrinsics)]
#![feature(simd_ffi)]
#![feature(link_llvm_intrinsics)]
#![feature(aarch64_target_feature)]
#![feature(arm_target_feature)]

extern crate libc;

pub mod chain;
pub mod gtest;
pub mod iccread;
pub mod matrix;
pub mod transform;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod transform_avx;
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub mod transform_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod transform_sse2;
pub mod transform_util;
