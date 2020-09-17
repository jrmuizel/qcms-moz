/* vim: set ts=8 sw=8 noexpandtab: */
//  qcms
//  Copyright (C) 2009 Mozilla Corporation
//  Copyright (C) 1998-2007 Marti Maria
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
use crate::{
    iccread::{curveType, lutType, lutmABType, qcms_profile},
    matrix::{matrix, matrix_invert},
    s15Fixed16Number, s15Fixed16Number_to_float,
    transform_util::clamp_float,
    transform_util::{
        build_colorant_matrix, build_input_gamma_table, build_output_lut, lut_interp_linear,
        lut_interp_linear_float,
    },
};
use ::libc::{self, calloc, free, malloc, memcpy, memset};

pub type __darwin_size_t = libc::c_ulong;
pub type int32_t = i32;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcms_modular_transform {
    pub matrix: matrix,
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
    pub input_clut_table_r: *mut f32,
    pub input_clut_table_g: *mut f32,
    pub input_clut_table_b: *mut f32,
    pub input_clut_table_length: u16,
    pub r_clut: *mut f32,
    pub g_clut: *mut f32,
    pub b_clut: *mut f32,
    pub grid_size: u16,
    pub output_clut_table_r: *mut f32,
    pub output_clut_table_g: *mut f32,
    pub output_clut_table_b: *mut f32,
    pub output_clut_table_length: u16,
    pub output_gamma_lut_r: *mut u16,
    pub output_gamma_lut_g: *mut u16,
    pub output_gamma_lut_b: *mut u16,
    pub output_gamma_lut_r_length: size_t,
    pub output_gamma_lut_g_length: size_t,
    pub output_gamma_lut_b_length: size_t,
    pub transform_module_fn: transform_module_fn_t,
    pub next_transform: *mut qcms_modular_transform,
}
pub type transform_module_fn_t = Option<
    unsafe extern "C" fn(_: *mut qcms_modular_transform, _: *mut f32, _: *mut f32, _: size_t) -> (),
>;

#[inline]
unsafe extern "C" fn lerp(mut a: f32, mut b: f32, mut t: f32) -> f32 {
    return a * (1.0f32 - t) + b * t;
}

unsafe extern "C" fn build_lut_matrix(mut lut: *mut lutType) -> matrix {
    let mut result: matrix = matrix {
        m: [[0.; 3]; 3],
        invalid: false,
    };
    if !lut.is_null() {
        result.m[0usize][0usize] = s15Fixed16Number_to_float((*lut).e00);
        result.m[0usize][1usize] = s15Fixed16Number_to_float((*lut).e01);
        result.m[0usize][2usize] = s15Fixed16Number_to_float((*lut).e02);
        result.m[1usize][0usize] = s15Fixed16Number_to_float((*lut).e10);
        result.m[1usize][1usize] = s15Fixed16Number_to_float((*lut).e11);
        result.m[1usize][2usize] = s15Fixed16Number_to_float((*lut).e12);
        result.m[2usize][0usize] = s15Fixed16Number_to_float((*lut).e20);
        result.m[2usize][1usize] = s15Fixed16Number_to_float((*lut).e21);
        result.m[2usize][2usize] = s15Fixed16Number_to_float((*lut).e22);
        result.invalid = 0i32 != 0
    } else {
        memset(
            &mut result as *mut matrix as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<matrix>(),
        );
        result.invalid = 1i32 != 0
    }
    return result;
}
unsafe extern "C" fn build_mAB_matrix(mut lut: *mut lutmABType) -> matrix {
    let mut result: matrix = matrix {
        m: [[0.; 3]; 3],
        invalid: false,
    };
    if !lut.is_null() {
        result.m[0usize][0usize] = s15Fixed16Number_to_float((*lut).e00);
        result.m[0usize][1usize] = s15Fixed16Number_to_float((*lut).e01);
        result.m[0usize][2usize] = s15Fixed16Number_to_float((*lut).e02);
        result.m[1usize][0usize] = s15Fixed16Number_to_float((*lut).e10);
        result.m[1usize][1usize] = s15Fixed16Number_to_float((*lut).e11);
        result.m[1usize][2usize] = s15Fixed16Number_to_float((*lut).e12);
        result.m[2usize][0usize] = s15Fixed16Number_to_float((*lut).e20);
        result.m[2usize][1usize] = s15Fixed16Number_to_float((*lut).e21);
        result.m[2usize][2usize] = s15Fixed16Number_to_float((*lut).e22);
        result.invalid = 0i32 != 0
    } else {
        memset(
            &mut result as *mut matrix as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<matrix>(),
        );
        result.invalid = 1i32 != 0
    }
    return result;
}
//Based on lcms cmsLab2XYZ
unsafe extern "C" fn qcms_transform_module_LAB_to_XYZ(
    mut transform: *mut qcms_modular_transform,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut length: size_t,
) {
    // lcms: D50 XYZ values
    let mut WhitePointX: f32 = 0.9642f32;
    let mut WhitePointY: f32 = 1.0f32;
    let mut WhitePointZ: f32 = 0.8249f32;
    let mut i: size_t = 0;
    while i < length {
        let fresh0 = src;
        src = src.offset(1);
        let mut device_L: f32 = *fresh0 * 100.0f32;
        let fresh1 = src;
        src = src.offset(1);
        let mut device_a: f32 = *fresh1 * 255.0f32 - 128.0f32;
        let fresh2 = src;
        src = src.offset(1);
        let mut device_b: f32 = *fresh2 * 255.0f32 - 128.0f32;
        let mut y: f32 = (device_L + 16.0f32) / 116.0f32;
        let mut X: f32 = if y + 0.002f32 * device_a <= 24.0f32 / 116.0f32 {
            (108.0f64 / 841.0f64) * ((y + 0.002f32 * device_a) as f64 - 16.0f64 / 116.0f64)
        } else {
            ((y + 0.002f32 * device_a)
                * (y + 0.002f32 * device_a)
                * (y + 0.002f32 * device_a)
                * WhitePointX) as f64
        } as f32;
        let mut Y: f32 = if y <= 24.0f32 / 116.0f32 {
            (108.0f64 / 841.0f64) * (y as f64 - 16.0f64 / 116.0f64)
        } else {
            (y * y * y * WhitePointY) as f64
        } as f32;
        let mut Z: f32 = if y - 0.005f32 * device_b <= 24.0f32 / 116.0f32 {
            (108.0f64 / 841.0f64) * ((y - 0.005f32 * device_b) as f64 - 16.0f64 / 116.0f64)
        } else {
            ((y - 0.005f32 * device_b)
                * (y - 0.005f32 * device_b)
                * (y - 0.005f32 * device_b)
                * WhitePointZ) as f64
        } as f32;
        let fresh3 = dest;
        dest = dest.offset(1);
        *fresh3 = (X as f64 / (1.0f64 + 32767.0f64 / 32768.0f64)) as f32;
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 = (Y as f64 / (1.0f64 + 32767.0f64 / 32768.0f64)) as f32;
        let fresh5 = dest;
        dest = dest.offset(1);
        *fresh5 = (Z as f64 / (1.0f64 + 32767.0f64 / 32768.0f64)) as f32;
        i = i + 1
    }
}
//Based on lcms cmsXYZ2Lab
unsafe extern "C" fn qcms_transform_module_XYZ_to_LAB(
    mut transform: *mut qcms_modular_transform,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut length: size_t,
) {
    // lcms: D50 XYZ values
    let mut WhitePointX: f32 = 0.9642f32;
    let mut WhitePointY: f32 = 1.0f32;
    let mut WhitePointZ: f32 = 0.8249f32;
    let mut i: size_t = 0;
    while i < length {
        let fresh6 = src;
        src = src.offset(1);
        let mut device_x: f32 =
            (*fresh6 as f64 * (1.0f64 + 32767.0f64 / 32768.0f64) / WhitePointX as f64) as f32;
        let fresh7 = src;
        src = src.offset(1);
        let mut device_y: f32 =
            (*fresh7 as f64 * (1.0f64 + 32767.0f64 / 32768.0f64) / WhitePointY as f64) as f32;
        let fresh8 = src;
        src = src.offset(1);
        let mut device_z: f32 =
            (*fresh8 as f64 * (1.0f64 + 32767.0f64 / 32768.0f64) / WhitePointZ as f64) as f32;
        let mut fx: f32 =
            if device_x <= 24.0f32 / 116.0f32 * (24.0f32 / 116.0f32) * (24.0f32 / 116.0f32) {
                (841.0f64 / 108.0f64 * device_x as f64) + 16.0f64 / 116.0f64
            } else {
                (device_x as f64).powf(1.0f64 / 3.0f64)
            } as f32;
        let mut fy: f32 =
            if device_y <= 24.0f32 / 116.0f32 * (24.0f32 / 116.0f32) * (24.0f32 / 116.0f32) {
                (841.0f64 / 108.0f64 * device_y as f64) + 16.0f64 / 116.0f64
            } else {
                (device_y as f64).powf(1.0f64 / 3.0f64)
            } as f32;
        let mut fz: f32 =
            if device_z <= 24.0f32 / 116.0f32 * (24.0f32 / 116.0f32) * (24.0f32 / 116.0f32) {
                (841.0f64 / 108.0f64 * device_z as f64) + 16.0f64 / 116.0f64
            } else {
                (device_z as f64).powf(1.0f64 / 3.0f64)
            } as f32;
        let mut L: f32 = 116.0f32 * fy - 16.0f32;
        let mut a: f32 = 500.0f32 * (fx - fy);
        let mut b: f32 = 200.0f32 * (fy - fz);
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = L / 100.0f32;
        let fresh10 = dest;
        dest = dest.offset(1);
        *fresh10 = (a + 128.0f32) / 255.0f32;
        let fresh11 = dest;
        dest = dest.offset(1);
        *fresh11 = (b + 128.0f32) / 255.0f32;
        i = i + 1
    }
}
unsafe extern "C" fn qcms_transform_module_clut_only(
    mut transform: *mut qcms_modular_transform,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut length: size_t,
) {
    let mut xy_len: i32 = 1i32;
    let mut x_len: i32 = (*transform).grid_size as i32;
    let mut len: i32 = x_len * x_len;
    let mut r_table: *mut f32 = (*transform).r_clut;
    let mut g_table: *mut f32 = (*transform).g_clut;
    let mut b_table: *mut f32 = (*transform).b_clut;
    let mut i: size_t = 0;
    while i < length {
        debug_assert!((*transform).grid_size as i32 >= 1i32);
        let fresh12 = src;
        src = src.offset(1);
        let mut linear_r: f32 = *fresh12;
        let fresh13 = src;
        src = src.offset(1);
        let mut linear_g: f32 = *fresh13;
        let fresh14 = src;
        src = src.offset(1);
        let mut linear_b: f32 = *fresh14;
        let mut x: i32 = (linear_r * ((*transform).grid_size as i32 - 1i32) as f32).floor() as i32;
        let mut y: i32 = (linear_g * ((*transform).grid_size as i32 - 1i32) as f32).floor() as i32;
        let mut z: i32 = (linear_b * ((*transform).grid_size as i32 - 1i32) as f32).floor() as i32;
        let mut x_n: i32 = (linear_r * ((*transform).grid_size as i32 - 1i32) as f32).ceil() as i32;
        let mut y_n: i32 = (linear_g * ((*transform).grid_size as i32 - 1i32) as f32).ceil() as i32;
        let mut z_n: i32 = (linear_b * ((*transform).grid_size as i32 - 1i32) as f32).ceil() as i32;
        let mut x_d: f32 = linear_r * ((*transform).grid_size as i32 - 1i32) as f32 - x as f32;
        let mut y_d: f32 = linear_g * ((*transform).grid_size as i32 - 1i32) as f32 - y as f32;
        let mut z_d: f32 = linear_b * ((*transform).grid_size as i32 - 1i32) as f32 - z as f32;
        let mut r_x1: f32 = lerp(
            *r_table.offset(((x * len + y * x_len + z * xy_len) * 3i32) as isize),
            *r_table.offset(((x_n * len + y * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut r_x2: f32 = lerp(
            *r_table.offset(((x * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            *r_table.offset(((x_n * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut r_y1: f32 = lerp(r_x1, r_x2, y_d);
        let mut r_x3: f32 = lerp(
            *r_table.offset(((x * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            *r_table.offset(((x_n * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut r_x4: f32 = lerp(
            *r_table.offset(((x * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut r_y2: f32 = lerp(r_x3, r_x4, y_d);
        let mut clut_r: f32 = lerp(r_y1, r_y2, z_d);
        let mut g_x1: f32 = lerp(
            *g_table.offset(((x * len + y * x_len + z * xy_len) * 3i32) as isize),
            *g_table.offset(((x_n * len + y * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut g_x2: f32 = lerp(
            *g_table.offset(((x * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            *g_table.offset(((x_n * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut g_y1: f32 = lerp(g_x1, g_x2, y_d);
        let mut g_x3: f32 = lerp(
            *g_table.offset(((x * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            *g_table.offset(((x_n * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut g_x4: f32 = lerp(
            *g_table.offset(((x * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut g_y2: f32 = lerp(g_x3, g_x4, y_d);
        let mut clut_g: f32 = lerp(g_y1, g_y2, z_d);
        let mut b_x1: f32 = lerp(
            *b_table.offset(((x * len + y * x_len + z * xy_len) * 3i32) as isize),
            *b_table.offset(((x_n * len + y * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut b_x2: f32 = lerp(
            *b_table.offset(((x * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            *b_table.offset(((x_n * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut b_y1: f32 = lerp(b_x1, b_x2, y_d);
        let mut b_x3: f32 = lerp(
            *b_table.offset(((x * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            *b_table.offset(((x_n * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut b_x4: f32 = lerp(
            *b_table.offset(((x * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut b_y2: f32 = lerp(b_x3, b_x4, y_d);
        let mut clut_b: f32 = lerp(b_y1, b_y2, z_d);
        let fresh15 = dest;
        dest = dest.offset(1);
        *fresh15 = clamp_float(clut_r);
        let fresh16 = dest;
        dest = dest.offset(1);
        *fresh16 = clamp_float(clut_g);
        let fresh17 = dest;
        dest = dest.offset(1);
        *fresh17 = clamp_float(clut_b);
        i = i + 1
    }
}
unsafe extern "C" fn qcms_transform_module_clut(
    mut transform: *mut qcms_modular_transform,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut length: size_t,
) {
    let mut xy_len: i32 = 1i32;
    let mut x_len: i32 = (*transform).grid_size as i32;
    let mut len: i32 = x_len * x_len;
    let mut r_table: *mut f32 = (*transform).r_clut;
    let mut g_table: *mut f32 = (*transform).g_clut;
    let mut b_table: *mut f32 = (*transform).b_clut;
    let mut i: size_t = 0;
    while i < length {
        debug_assert!((*transform).grid_size as i32 >= 1i32);
        let fresh18 = src;
        src = src.offset(1);
        let mut device_r: f32 = *fresh18;
        let fresh19 = src;
        src = src.offset(1);
        let mut device_g: f32 = *fresh19;
        let fresh20 = src;
        src = src.offset(1);
        let mut device_b: f32 = *fresh20;
        let mut linear_r: f32 = lut_interp_linear_float(
            device_r,
            (*transform).input_clut_table_r,
            (*transform).input_clut_table_length as i32,
        );
        let mut linear_g: f32 = lut_interp_linear_float(
            device_g,
            (*transform).input_clut_table_g,
            (*transform).input_clut_table_length as i32,
        );
        let mut linear_b: f32 = lut_interp_linear_float(
            device_b,
            (*transform).input_clut_table_b,
            (*transform).input_clut_table_length as i32,
        );
        let mut x: i32 = (linear_r * ((*transform).grid_size as i32 - 1i32) as f32).floor() as i32;
        let mut y: i32 = (linear_g * ((*transform).grid_size as i32 - 1i32) as f32).floor() as i32;
        let mut z: i32 = (linear_b * ((*transform).grid_size as i32 - 1i32) as f32).floor() as i32;
        let mut x_n: i32 =
            (linear_r * ((*transform).grid_size as i32 - 1i32) as f32).floor() as i32;
        let mut y_n: i32 =
            (linear_g * ((*transform).grid_size as i32 - 1i32) as f32).floor() as i32;
        let mut z_n: i32 = (linear_b * ((*transform).grid_size as i32 - 1i32) as f32).ceil() as i32;
        let mut x_d: f32 = linear_r * ((*transform).grid_size as i32 - 1i32) as f32 - x as f32;
        let mut y_d: f32 = linear_g * ((*transform).grid_size as i32 - 1i32) as f32 - y as f32;
        let mut z_d: f32 = linear_b * ((*transform).grid_size as i32 - 1i32) as f32 - z as f32;
        let mut r_x1: f32 = lerp(
            *r_table.offset(((x * len + y * x_len + z * xy_len) * 3i32) as isize),
            *r_table.offset(((x_n * len + y * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut r_x2: f32 = lerp(
            *r_table.offset(((x * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            *r_table.offset(((x_n * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut r_y1: f32 = lerp(r_x1, r_x2, y_d);
        let mut r_x3: f32 = lerp(
            *r_table.offset(((x * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            *r_table.offset(((x_n * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut r_x4: f32 = lerp(
            *r_table.offset(((x * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut r_y2: f32 = lerp(r_x3, r_x4, y_d);
        let mut clut_r: f32 = lerp(r_y1, r_y2, z_d);
        let mut g_x1: f32 = lerp(
            *g_table.offset(((x * len + y * x_len + z * xy_len) * 3i32) as isize),
            *g_table.offset(((x_n * len + y * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut g_x2: f32 = lerp(
            *g_table.offset(((x * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            *g_table.offset(((x_n * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut g_y1: f32 = lerp(g_x1, g_x2, y_d);
        let mut g_x3: f32 = lerp(
            *g_table.offset(((x * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            *g_table.offset(((x_n * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut g_x4: f32 = lerp(
            *g_table.offset(((x * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut g_y2: f32 = lerp(g_x3, g_x4, y_d);
        let mut clut_g: f32 = lerp(g_y1, g_y2, z_d);
        let mut b_x1: f32 = lerp(
            *b_table.offset(((x * len + y * x_len + z * xy_len) * 3i32) as isize),
            *b_table.offset(((x_n * len + y * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut b_x2: f32 = lerp(
            *b_table.offset(((x * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            *b_table.offset(((x_n * len + y_n * x_len + z * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut b_y1: f32 = lerp(b_x1, b_x2, y_d);
        let mut b_x3: f32 = lerp(
            *b_table.offset(((x * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            *b_table.offset(((x_n * len + y * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut b_x4: f32 = lerp(
            *b_table.offset(((x * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) * 3i32) as isize),
            x_d,
        );
        let mut b_y2: f32 = lerp(b_x3, b_x4, y_d);
        let mut clut_b: f32 = lerp(b_y1, b_y2, z_d);
        let mut pcs_r: f32 = lut_interp_linear_float(
            clut_r,
            (*transform).output_clut_table_r,
            (*transform).output_clut_table_length as i32,
        );
        let mut pcs_g: f32 = lut_interp_linear_float(
            clut_g,
            (*transform).output_clut_table_g,
            (*transform).output_clut_table_length as i32,
        );
        let mut pcs_b: f32 = lut_interp_linear_float(
            clut_b,
            (*transform).output_clut_table_b,
            (*transform).output_clut_table_length as i32,
        );
        let fresh21 = dest;
        dest = dest.offset(1);
        *fresh21 = clamp_float(pcs_r);
        let fresh22 = dest;
        dest = dest.offset(1);
        *fresh22 = clamp_float(pcs_g);
        let fresh23 = dest;
        dest = dest.offset(1);
        *fresh23 = clamp_float(pcs_b);
        i = i + 1
    }
}
/* NOT USED
static void qcms_transform_module_tetra_clut(struct qcms_modular_transform *transform, float *src, float *dest, size_t length)
{
    size_t i;
    int xy_len = 1;
    int x_len = transform->grid_size;
    int len = x_len * x_len;
    float* r_table = transform->r_clut;
    float* g_table = transform->g_clut;
    float* b_table = transform->b_clut;
    float c0_r, c1_r, c2_r, c3_r;
    float c0_g, c1_g, c2_g, c3_g;
    float c0_b, c1_b, c2_b, c3_b;
    float clut_r, clut_g, clut_b;
    float pcs_r, pcs_g, pcs_b;
    for (i = 0; i < length; i++) {
        float device_r = *src++;
        float device_g = *src++;
        float device_b = *src++;
        float linear_r = lut_interp_linear_float(device_r,
                transform->input_clut_table_r, transform->input_clut_table_length);
        float linear_g = lut_interp_linear_float(device_g,
                transform->input_clut_table_g, transform->input_clut_table_length);
        float linear_b = lut_interp_linear_float(device_b,
                transform->input_clut_table_b, transform->input_clut_table_length);

        int x = floorf(linear_r * (transform->grid_size-1));
        int y = floorf(linear_g * (transform->grid_size-1));
        int z = floorf(linear_b * (transform->grid_size-1));
        int x_n = ceilf(linear_r * (transform->grid_size-1));
        int y_n = ceilf(linear_g * (transform->grid_size-1));
        int z_n = ceilf(linear_b * (transform->grid_size-1));
        float rx = linear_r * (transform->grid_size-1) - x;
        float ry = linear_g * (transform->grid_size-1) - y;
        float rz = linear_b * (transform->grid_size-1) - z;

        c0_r = CLU(r_table, x, y, z);
        c0_g = CLU(g_table, x, y, z);
        c0_b = CLU(b_table, x, y, z);
        if( rx >= ry ) {
            if (ry >= rz) { //rx >= ry && ry >= rz
                c1_r = CLU(r_table, x_n, y, z) - c0_r;
                c2_r = CLU(r_table, x_n, y_n, z) - CLU(r_table, x_n, y, z);
                c3_r = CLU(r_table, x_n, y_n, z_n) - CLU(r_table, x_n, y_n, z);
                c1_g = CLU(g_table, x_n, y, z) - c0_g;
                c2_g = CLU(g_table, x_n, y_n, z) - CLU(g_table, x_n, y, z);
                c3_g = CLU(g_table, x_n, y_n, z_n) - CLU(g_table, x_n, y_n, z);
                c1_b = CLU(b_table, x_n, y, z) - c0_b;
                c2_b = CLU(b_table, x_n, y_n, z) - CLU(b_table, x_n, y, z);
                c3_b = CLU(b_table, x_n, y_n, z_n) - CLU(b_table, x_n, y_n, z);
            } else {
                if (rx >= rz) { //rx >= rz && rz >= ry
                    c1_r = CLU(r_table, x_n, y, z) - c0_r;
                    c2_r = CLU(r_table, x_n, y_n, z_n) - CLU(r_table, x_n, y, z_n);
                    c3_r = CLU(r_table, x_n, y, z_n) - CLU(r_table, x_n, y, z);
                    c1_g = CLU(g_table, x_n, y, z) - c0_g;
                    c2_g = CLU(g_table, x_n, y_n, z_n) - CLU(g_table, x_n, y, z_n);
                    c3_g = CLU(g_table, x_n, y, z_n) - CLU(g_table, x_n, y, z);
                    c1_b = CLU(b_table, x_n, y, z) - c0_b;
                    c2_b = CLU(b_table, x_n, y_n, z_n) - CLU(b_table, x_n, y, z_n);
                    c3_b = CLU(b_table, x_n, y, z_n) - CLU(b_table, x_n, y, z);
                } else { //rz > rx && rx >= ry
                    c1_r = CLU(r_table, x_n, y, z_n) - CLU(r_table, x, y, z_n);
                    c2_r = CLU(r_table, x_n, y_n, z_n) - CLU(r_table, x_n, y, z_n);
                    c3_r = CLU(r_table, x, y, z_n) - c0_r;
                    c1_g = CLU(g_table, x_n, y, z_n) - CLU(g_table, x, y, z_n);
                    c2_g = CLU(g_table, x_n, y_n, z_n) - CLU(g_table, x_n, y, z_n);
                    c3_g = CLU(g_table, x, y, z_n) - c0_g;
                    c1_b = CLU(b_table, x_n, y, z_n) - CLU(b_table, x, y, z_n);
                    c2_b = CLU(b_table, x_n, y_n, z_n) - CLU(b_table, x_n, y, z_n);
                    c3_b = CLU(b_table, x, y, z_n) - c0_b;
                }
            }
        } else {
            if (rx >= rz) { //ry > rx && rx >= rz
                c1_r = CLU(r_table, x_n, y_n, z) - CLU(r_table, x, y_n, z);
                c2_r = CLU(r_table, x_n, y_n, z) - c0_r;
                c3_r = CLU(r_table, x_n, y_n, z_n) - CLU(r_table, x_n, y_n, z);
                c1_g = CLU(g_table, x_n, y_n, z) - CLU(g_table, x, y_n, z);
                c2_g = CLU(g_table, x_n, y_n, z) - c0_g;
                c3_g = CLU(g_table, x_n, y_n, z_n) - CLU(g_table, x_n, y_n, z);
                c1_b = CLU(b_table, x_n, y_n, z) - CLU(b_table, x, y_n, z);
                c2_b = CLU(b_table, x_n, y_n, z) - c0_b;
                c3_b = CLU(b_table, x_n, y_n, z_n) - CLU(b_table, x_n, y_n, z);
            } else {
                if (ry >= rz) { //ry >= rz && rz > rx
                    c1_r = CLU(r_table, x_n, y_n, z_n) - CLU(r_table, x, y_n, z_n);
                    c2_r = CLU(r_table, x, y_n, z) - c0_r;
                    c3_r = CLU(r_table, x, y_n, z_n) - CLU(r_table, x, y_n, z);
                    c1_g = CLU(g_table, x_n, y_n, z_n) - CLU(g_table, x, y_n, z_n);
                    c2_g = CLU(g_table, x, y_n, z) - c0_g;
                    c3_g = CLU(g_table, x, y_n, z_n) - CLU(g_table, x, y_n, z);
                    c1_b = CLU(b_table, x_n, y_n, z_n) - CLU(b_table, x, y_n, z_n);
                    c2_b = CLU(b_table, x, y_n, z) - c0_b;
                    c3_b = CLU(b_table, x, y_n, z_n) - CLU(b_table, x, y_n, z);
                } else { //rz > ry && ry > rx
                    c1_r = CLU(r_table, x_n, y_n, z_n) - CLU(r_table, x, y_n, z_n);
                    c2_r = CLU(r_table, x, y_n, z) - c0_r;
                    c3_r = CLU(r_table, x_n, y_n, z_n) - CLU(r_table, x_n, y_n, z);
                    c1_g = CLU(g_table, x_n, y_n, z_n) - CLU(g_table, x, y_n, z_n);
                    c2_g = CLU(g_table, x, y_n, z) - c0_g;
                    c3_g = CLU(g_table, x_n, y_n, z_n) - CLU(g_table, x_n, y_n, z);
                    c1_b = CLU(b_table, x_n, y_n, z_n) - CLU(b_table, x, y_n, z_n);
                    c2_b = CLU(b_table, x, y_n, z) - c0_b;
                    c3_b = CLU(b_table, x_n, y_n, z_n) - CLU(b_table, x_n, y_n, z);
                }
            }
        }

        clut_r = c0_r + c1_r*rx + c2_r*ry + c3_r*rz;
        clut_g = c0_g + c1_g*rx + c2_g*ry + c3_g*rz;
        clut_b = c0_b + c1_b*rx + c2_b*ry + c3_b*rz;

        pcs_r = lut_interp_linear_float(clut_r,
                transform->output_clut_table_r, transform->output_clut_table_length);
        pcs_g = lut_interp_linear_float(clut_g,
                transform->output_clut_table_g, transform->output_clut_table_length);
        pcs_b = lut_interp_linear_float(clut_b,
                transform->output_clut_table_b, transform->output_clut_table_length);
        *dest++ = clamp_float(pcs_r);
        *dest++ = clamp_float(pcs_g);
        *dest++ = clamp_float(pcs_b);
    }
}
*/
unsafe extern "C" fn qcms_transform_module_gamma_table(
    mut transform: *mut qcms_modular_transform,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut length: size_t,
) {
    let mut out_r: f32 = 0.;
    let mut out_g: f32 = 0.;
    let mut out_b: f32 = 0.;
    let mut i: size_t = 0;
    while i < length {
        let fresh24 = src;
        src = src.offset(1);
        let mut in_r: f32 = *fresh24;
        let fresh25 = src;
        src = src.offset(1);
        let mut in_g: f32 = *fresh25;
        let fresh26 = src;
        src = src.offset(1);
        let mut in_b: f32 = *fresh26;
        out_r = lut_interp_linear_float(in_r, (*transform).input_clut_table_r, 256i32);
        out_g = lut_interp_linear_float(in_g, (*transform).input_clut_table_g, 256i32);
        out_b = lut_interp_linear_float(in_b, (*transform).input_clut_table_b, 256i32);
        let fresh27 = dest;
        dest = dest.offset(1);
        *fresh27 = clamp_float(out_r);
        let fresh28 = dest;
        dest = dest.offset(1);
        *fresh28 = clamp_float(out_g);
        let fresh29 = dest;
        dest = dest.offset(1);
        *fresh29 = clamp_float(out_b);
        i = i + 1
    }
}
unsafe extern "C" fn qcms_transform_module_gamma_lut(
    mut transform: *mut qcms_modular_transform,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut length: size_t,
) {
    let mut out_r: f32 = 0.;
    let mut out_g: f32 = 0.;
    let mut out_b: f32 = 0.;
    let mut i: size_t = 0;
    while i < length {
        let fresh30 = src;
        src = src.offset(1);
        let mut in_r: f32 = *fresh30;
        let fresh31 = src;
        src = src.offset(1);
        let mut in_g: f32 = *fresh31;
        let fresh32 = src;
        src = src.offset(1);
        let mut in_b: f32 = *fresh32;
        out_r = lut_interp_linear(
            in_r as f64,
            (*transform).output_gamma_lut_r,
            (*transform).output_gamma_lut_r_length as i32,
        );
        out_g = lut_interp_linear(
            in_g as f64,
            (*transform).output_gamma_lut_g,
            (*transform).output_gamma_lut_g_length as i32,
        );
        out_b = lut_interp_linear(
            in_b as f64,
            (*transform).output_gamma_lut_b,
            (*transform).output_gamma_lut_b_length as i32,
        );
        let fresh33 = dest;
        dest = dest.offset(1);
        *fresh33 = clamp_float(out_r);
        let fresh34 = dest;
        dest = dest.offset(1);
        *fresh34 = clamp_float(out_g);
        let fresh35 = dest;
        dest = dest.offset(1);
        *fresh35 = clamp_float(out_b);
        i = i + 1
    }
}
unsafe extern "C" fn qcms_transform_module_matrix_translate(
    mut transform: *mut qcms_modular_transform,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut length: size_t,
) {
    let mut mat: matrix = matrix {
        m: [[0.; 3]; 3],
        invalid: false,
    };
    /* store the results in column major mode
     * this makes doing the multiplication with sse easier */
    mat.m[0usize][0usize] = (*transform).matrix.m[0usize][0usize];
    mat.m[1usize][0usize] = (*transform).matrix.m[0usize][1usize];
    mat.m[2usize][0usize] = (*transform).matrix.m[0usize][2usize];
    mat.m[0usize][1usize] = (*transform).matrix.m[1usize][0usize];
    mat.m[1usize][1usize] = (*transform).matrix.m[1usize][1usize];
    mat.m[2usize][1usize] = (*transform).matrix.m[1usize][2usize];
    mat.m[0usize][2usize] = (*transform).matrix.m[2usize][0usize];
    mat.m[1usize][2usize] = (*transform).matrix.m[2usize][1usize];
    mat.m[2usize][2usize] = (*transform).matrix.m[2usize][2usize];
    let mut i: size_t = 0;
    while i < length {
        let fresh36 = src;
        src = src.offset(1);
        let mut in_r: f32 = *fresh36;
        let fresh37 = src;
        src = src.offset(1);
        let mut in_g: f32 = *fresh37;
        let fresh38 = src;
        src = src.offset(1);
        let mut in_b: f32 = *fresh38;
        let mut out_r: f32 = mat.m[0usize][0usize] * in_r
            + mat.m[1usize][0usize] * in_g
            + mat.m[2usize][0usize] * in_b
            + (*transform).tx;
        let mut out_g: f32 = mat.m[0usize][1usize] * in_r
            + mat.m[1usize][1usize] * in_g
            + mat.m[2usize][1usize] * in_b
            + (*transform).ty;
        let mut out_b: f32 = mat.m[0usize][2usize] * in_r
            + mat.m[1usize][2usize] * in_g
            + mat.m[2usize][2usize] * in_b
            + (*transform).tz;
        let fresh39 = dest;
        dest = dest.offset(1);
        *fresh39 = clamp_float(out_r);
        let fresh40 = dest;
        dest = dest.offset(1);
        *fresh40 = clamp_float(out_g);
        let fresh41 = dest;
        dest = dest.offset(1);
        *fresh41 = clamp_float(out_b);
        i = i + 1
    }
}
unsafe extern "C" fn qcms_transform_module_matrix(
    mut transform: *mut qcms_modular_transform,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut length: size_t,
) {
    let mut mat: matrix = matrix {
        m: [[0.; 3]; 3],
        invalid: false,
    };
    /* store the results in column major mode
     * this makes doing the multiplication with sse easier */
    mat.m[0usize][0usize] = (*transform).matrix.m[0usize][0usize];
    mat.m[1usize][0usize] = (*transform).matrix.m[0usize][1usize];
    mat.m[2usize][0usize] = (*transform).matrix.m[0usize][2usize];
    mat.m[0usize][1usize] = (*transform).matrix.m[1usize][0usize];
    mat.m[1usize][1usize] = (*transform).matrix.m[1usize][1usize];
    mat.m[2usize][1usize] = (*transform).matrix.m[1usize][2usize];
    mat.m[0usize][2usize] = (*transform).matrix.m[2usize][0usize];
    mat.m[1usize][2usize] = (*transform).matrix.m[2usize][1usize];
    mat.m[2usize][2usize] = (*transform).matrix.m[2usize][2usize];
    let mut i: size_t = 0;
    while i < length {
        let fresh42 = src;
        src = src.offset(1);
        let mut in_r: f32 = *fresh42;
        let fresh43 = src;
        src = src.offset(1);
        let mut in_g: f32 = *fresh43;
        let fresh44 = src;
        src = src.offset(1);
        let mut in_b: f32 = *fresh44;
        let mut out_r: f32 = mat.m[0usize][0usize] * in_r
            + mat.m[1usize][0usize] * in_g
            + mat.m[2usize][0usize] * in_b;
        let mut out_g: f32 = mat.m[0usize][1usize] * in_r
            + mat.m[1usize][1usize] * in_g
            + mat.m[2usize][1usize] * in_b;
        let mut out_b: f32 = mat.m[0usize][2usize] * in_r
            + mat.m[1usize][2usize] * in_g
            + mat.m[2usize][2usize] * in_b;
        let fresh45 = dest;
        dest = dest.offset(1);
        *fresh45 = clamp_float(out_r);
        let fresh46 = dest;
        dest = dest.offset(1);
        *fresh46 = clamp_float(out_g);
        let fresh47 = dest;
        dest = dest.offset(1);
        *fresh47 = clamp_float(out_b);
        i = i + 1
    }
}
unsafe extern "C" fn qcms_modular_transform_alloc() -> *mut qcms_modular_transform {
    return calloc(1, ::std::mem::size_of::<qcms_modular_transform>())
        as *mut qcms_modular_transform;
}
unsafe extern "C" fn qcms_modular_transform_release(mut transform: *mut qcms_modular_transform) {
    let mut next_transform: *mut qcms_modular_transform = 0 as *mut qcms_modular_transform;
    while !transform.is_null() {
        next_transform = (*transform).next_transform;
        // clut may use a single block of memory.
        // Perhaps we should remove this to simply the code.
        if (*transform)
            .input_clut_table_r
            .offset((*transform).input_clut_table_length as i32 as isize)
            == (*transform).input_clut_table_g
            && (*transform)
                .input_clut_table_g
                .offset((*transform).input_clut_table_length as i32 as isize)
                == (*transform).input_clut_table_b
        {
            if !(*transform).input_clut_table_r.is_null() {
                free((*transform).input_clut_table_r as *mut libc::c_void);
            }
        } else {
            if !(*transform).input_clut_table_r.is_null() {
                free((*transform).input_clut_table_r as *mut libc::c_void);
            }
            if !(*transform).input_clut_table_g.is_null() {
                free((*transform).input_clut_table_g as *mut libc::c_void);
            }
            if !(*transform).input_clut_table_b.is_null() {
                free((*transform).input_clut_table_b as *mut libc::c_void);
            }
        }
        if (*transform).r_clut.offset(1isize) == (*transform).g_clut
            && (*transform).g_clut.offset(1isize) == (*transform).b_clut
        {
            if !(*transform).r_clut.is_null() {
                free((*transform).r_clut as *mut libc::c_void);
            }
        } else {
            if !(*transform).r_clut.is_null() {
                free((*transform).r_clut as *mut libc::c_void);
            }
            if !(*transform).g_clut.is_null() {
                free((*transform).g_clut as *mut libc::c_void);
            }
            if !(*transform).b_clut.is_null() {
                free((*transform).b_clut as *mut libc::c_void);
            }
        }
        if (*transform)
            .output_clut_table_r
            .offset((*transform).output_clut_table_length as i32 as isize)
            == (*transform).output_clut_table_g
            && (*transform)
                .output_clut_table_g
                .offset((*transform).output_clut_table_length as i32 as isize)
                == (*transform).output_clut_table_b
        {
            if !(*transform).output_clut_table_r.is_null() {
                free((*transform).output_clut_table_r as *mut libc::c_void);
            }
        } else {
            if !(*transform).output_clut_table_r.is_null() {
                free((*transform).output_clut_table_r as *mut libc::c_void);
            }
            if !(*transform).output_clut_table_g.is_null() {
                free((*transform).output_clut_table_g as *mut libc::c_void);
            }
            if !(*transform).output_clut_table_b.is_null() {
                free((*transform).output_clut_table_b as *mut libc::c_void);
            }
        }
        if !(*transform).output_gamma_lut_r.is_null() {
            free((*transform).output_gamma_lut_r as *mut libc::c_void);
        }
        if !(*transform).output_gamma_lut_g.is_null() {
            free((*transform).output_gamma_lut_g as *mut libc::c_void);
        }
        if !(*transform).output_gamma_lut_b.is_null() {
            free((*transform).output_gamma_lut_b as *mut libc::c_void);
        }
        free(transform as *mut libc::c_void);
        transform = next_transform
    }
}
/* Set transform to be the next element in the linked list. */
unsafe extern "C" fn append_transform(
    mut transform: *mut qcms_modular_transform,
    mut next_transform: *mut *mut *mut qcms_modular_transform,
) {
    **next_transform = transform;
    while !transform.is_null() {
        *next_transform = &mut (*transform).next_transform;
        transform = (*transform).next_transform
    }
}
/* reverse the transformation list (used by mBA) */
unsafe extern "C" fn reverse_transform(
    mut transform: *mut qcms_modular_transform,
) -> *mut qcms_modular_transform {
    let mut prev_transform: *mut qcms_modular_transform = 0 as *mut qcms_modular_transform;
    while !transform.is_null() {
        let mut next_transform: *mut qcms_modular_transform = (*transform).next_transform;
        (*transform).next_transform = prev_transform;
        prev_transform = transform;
        transform = next_transform
    }
    return prev_transform;
}
unsafe extern "C" fn qcms_modular_transform_create_mAB(
    mut lut: *mut lutmABType,
) -> *mut qcms_modular_transform {
    let mut current_block: u64;
    let mut first_transform: *mut qcms_modular_transform = 0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform = &mut first_transform;
    let mut transform: *mut qcms_modular_transform = 0 as *mut qcms_modular_transform;
    if !(*lut).a_curves[0usize].is_null() {
        let mut clut_length: usize = 0;
        let mut clut: *mut f32 = 0 as *mut f32;
        // If the A curve is present this also implies the
        // presence of a CLUT.
        if (*lut).clut_table.is_null() {
            current_block = 7590209878260659629;
        } else {
            // Prepare A curve.
            transform = qcms_modular_transform_alloc();
            if transform.is_null() {
                current_block = 7590209878260659629;
            } else {
                append_transform(transform, &mut next_transform);
                (*transform).input_clut_table_r = build_input_gamma_table((*lut).a_curves[0usize]);
                (*transform).input_clut_table_g = build_input_gamma_table((*lut).a_curves[1usize]);
                (*transform).input_clut_table_b = build_input_gamma_table((*lut).a_curves[2usize]);
                (*transform).transform_module_fn = Some(
                    qcms_transform_module_gamma_table
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                );
                if (*lut).num_grid_points[0usize] as i32 != (*lut).num_grid_points[1usize] as i32
                    || (*lut).num_grid_points[1usize] as i32
                        != (*lut).num_grid_points[2usize] as i32
                {
                    //XXX: We don't currently support clut that are not squared!
                    current_block = 7590209878260659629;
                } else {
                    // Prepare CLUT
                    transform = qcms_modular_transform_alloc();
                    if transform.is_null() {
                        current_block = 7590209878260659629;
                    } else {
                        append_transform(transform, &mut next_transform);
                        clut_length = (::std::mem::size_of::<f32>() as f64
                            * ((*lut).num_grid_points[0usize] as f64).powf(3f64)
                            * 3f64) as usize;
                        clut = malloc(clut_length) as *mut f32;
                        if clut.is_null() {
                            current_block = 7590209878260659629;
                        } else {
                            memcpy(
                                clut as *mut libc::c_void,
                                (*lut).clut_table as *const libc::c_void,
                                clut_length,
                            );
                            (*transform).r_clut = clut.offset(0isize);
                            (*transform).g_clut = clut.offset(1isize);
                            (*transform).b_clut = clut.offset(2isize);
                            (*transform).grid_size = (*lut).num_grid_points[0usize] as u16;
                            (*transform).transform_module_fn = Some(
                                qcms_transform_module_clut_only
                                    as unsafe extern "C" fn(
                                        _: *mut qcms_modular_transform,
                                        _: *mut f32,
                                        _: *mut f32,
                                        _: size_t,
                                    )
                                        -> (),
                            );
                            current_block = 10652014663920648156;
                        }
                    }
                }
            }
        }
    } else {
        current_block = 10652014663920648156;
    }
    match current_block {
        10652014663920648156 => {
            if !(*lut).m_curves[0usize].is_null() {
                // M curve imples the presence of a Matrix
                // Prepare M curve
                transform = qcms_modular_transform_alloc();
                if transform.is_null() {
                    current_block = 7590209878260659629;
                } else {
                    append_transform(transform, &mut next_transform);
                    (*transform).input_clut_table_r =
                        build_input_gamma_table((*lut).m_curves[0usize]);
                    (*transform).input_clut_table_g =
                        build_input_gamma_table((*lut).m_curves[1usize]);
                    (*transform).input_clut_table_b =
                        build_input_gamma_table((*lut).m_curves[2usize]);
                    (*transform).transform_module_fn = Some(
                        qcms_transform_module_gamma_table
                            as unsafe extern "C" fn(
                                _: *mut qcms_modular_transform,
                                _: *mut f32,
                                _: *mut f32,
                                _: size_t,
                            ) -> (),
                    );
                    // Prepare Matrix
                    transform = qcms_modular_transform_alloc();
                    if transform.is_null() {
                        current_block = 7590209878260659629;
                    } else {
                        append_transform(transform, &mut next_transform);
                        (*transform).matrix = build_mAB_matrix(lut);
                        if (*transform).matrix.invalid {
                            current_block = 7590209878260659629;
                        } else {
                            (*transform).tx = s15Fixed16Number_to_float((*lut).e03);
                            (*transform).ty = s15Fixed16Number_to_float((*lut).e13);
                            (*transform).tz = s15Fixed16Number_to_float((*lut).e23);
                            (*transform).transform_module_fn = Some(
                                qcms_transform_module_matrix_translate
                                    as unsafe extern "C" fn(
                                        _: *mut qcms_modular_transform,
                                        _: *mut f32,
                                        _: *mut f32,
                                        _: size_t,
                                    )
                                        -> (),
                            );
                            current_block = 980989089337379490;
                        }
                    }
                }
            } else {
                current_block = 980989089337379490;
            }
            match current_block {
                7590209878260659629 => {}
                _ => {
                    if !(*lut).b_curves[0usize].is_null() {
                        // Prepare B curve
                        transform = qcms_modular_transform_alloc();
                        if !transform.is_null() {
                            append_transform(transform, &mut next_transform);
                            (*transform).input_clut_table_r =
                                build_input_gamma_table((*lut).b_curves[0usize]);
                            (*transform).input_clut_table_g =
                                build_input_gamma_table((*lut).b_curves[1usize]);
                            (*transform).input_clut_table_b =
                                build_input_gamma_table((*lut).b_curves[2usize]);
                            (*transform).transform_module_fn = Some(
                                qcms_transform_module_gamma_table
                                    as unsafe extern "C" fn(
                                        _: *mut qcms_modular_transform,
                                        _: *mut f32,
                                        _: *mut f32,
                                        _: size_t,
                                    )
                                        -> (),
                            );
                            if (*lut).reversed {
                                // mBA are identical to mAB except that the transformation order
                                // is reversed
                                first_transform = reverse_transform(first_transform)
                            }
                            return first_transform;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    // B curve is mandatory
    qcms_modular_transform_release(first_transform);
    return 0 as *mut qcms_modular_transform;
}
unsafe extern "C" fn qcms_modular_transform_create_lut(
    mut lut: *mut lutType,
) -> *mut qcms_modular_transform {
    let mut first_transform: *mut qcms_modular_transform = 0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform = &mut first_transform;

    let mut in_curve_len: usize = 0;
    let mut clut_length: usize = 0;
    let mut out_curve_len: usize = 0;
    let mut in_curves: *mut f32 = 0 as *mut f32;
    let mut clut: *mut f32 = 0 as *mut f32;
    let mut out_curves: *mut f32 = 0 as *mut f32;
    let mut transform: *mut qcms_modular_transform = qcms_modular_transform_alloc();
    if !transform.is_null() {
        append_transform(transform, &mut next_transform);
        (*transform).matrix = build_lut_matrix(lut);
        if !(*transform).matrix.invalid {
            (*transform).transform_module_fn = Some(
                qcms_transform_module_matrix
                    as unsafe extern "C" fn(
                        _: *mut qcms_modular_transform,
                        _: *mut f32,
                        _: *mut f32,
                        _: size_t,
                    ) -> (),
            );
            // Prepare input curves
            transform = qcms_modular_transform_alloc();
            if !transform.is_null() {
                append_transform(transform, &mut next_transform);
                in_curve_len =
                    ::std::mem::size_of::<f32>() * (*lut).num_input_table_entries as usize * 3;
                in_curves = malloc(in_curve_len) as *mut f32;
                if !in_curves.is_null() {
                    memcpy(
                        in_curves as *mut libc::c_void,
                        (*lut).input_table as *const libc::c_void,
                        in_curve_len,
                    );
                    (*transform).input_clut_table_r =
                        in_curves.offset(((*lut).num_input_table_entries as i32 * 0i32) as isize);
                    (*transform).input_clut_table_g =
                        in_curves.offset(((*lut).num_input_table_entries as i32 * 1i32) as isize);
                    (*transform).input_clut_table_b =
                        in_curves.offset(((*lut).num_input_table_entries as i32 * 2i32) as isize);
                    (*transform).input_clut_table_length = (*lut).num_input_table_entries;
                    // Prepare table
                    clut_length = (::std::mem::size_of::<f32>() as f64
                        * ((*lut).num_clut_grid_points as f64).powf(3f64)
                        * 3f64) as usize;
                    clut = malloc(clut_length) as *mut f32;
                    if !clut.is_null() {
                        memcpy(
                            clut as *mut libc::c_void,
                            (*lut).clut_table as *const libc::c_void,
                            clut_length,
                        );
                        (*transform).r_clut = clut.offset(0isize);
                        (*transform).g_clut = clut.offset(1isize);
                        (*transform).b_clut = clut.offset(2isize);
                        (*transform).grid_size = (*lut).num_clut_grid_points as u16;
                        // Prepare output curves
                        out_curve_len = ::std::mem::size_of::<f32>()
                            * (*lut).num_output_table_entries as usize
                            * 3;
                        out_curves = malloc(out_curve_len) as *mut f32;
                        if !out_curves.is_null() {
                            memcpy(
                                out_curves as *mut libc::c_void,
                                (*lut).output_table as *const libc::c_void,
                                out_curve_len,
                            );
                            (*transform).output_clut_table_r = out_curves
                                .offset(((*lut).num_output_table_entries as i32 * 0i32) as isize);
                            (*transform).output_clut_table_g = out_curves
                                .offset(((*lut).num_output_table_entries as i32 * 1i32) as isize);
                            (*transform).output_clut_table_b = out_curves
                                .offset(((*lut).num_output_table_entries as i32 * 2i32) as isize);
                            (*transform).output_clut_table_length = (*lut).num_output_table_entries;
                            (*transform).transform_module_fn = Some(
                                qcms_transform_module_clut
                                    as unsafe extern "C" fn(
                                        _: *mut qcms_modular_transform,
                                        _: *mut f32,
                                        _: *mut f32,
                                        _: size_t,
                                    )
                                        -> (),
                            );
                            return first_transform;
                        }
                    }
                }
            }
        }
    }
    qcms_modular_transform_release(first_transform);
    return 0 as *mut qcms_modular_transform;
}
#[no_mangle]
pub unsafe extern "C" fn qcms_modular_transform_create_input(
    mut in_0: *mut qcms_profile,
) -> *mut qcms_modular_transform {
    let mut current_block: u64;
    let mut first_transform: *mut qcms_modular_transform = 0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform = &mut first_transform;
    if !(*in_0).A2B0.is_null() {
        let mut lut_transform: *mut qcms_modular_transform =
            qcms_modular_transform_create_lut((*in_0).A2B0);
        if lut_transform.is_null() {
            current_block = 8903102000210989603;
        } else {
            append_transform(lut_transform, &mut next_transform);
            current_block = 10692455896603418738;
        }
    } else if !(*in_0).mAB.is_null()
        && (*(*in_0).mAB).num_in_channels as i32 == 3i32
        && (*(*in_0).mAB).num_out_channels as i32 == 3i32
    {
        let mut mAB_transform: *mut qcms_modular_transform =
            qcms_modular_transform_create_mAB((*in_0).mAB);
        if mAB_transform.is_null() {
            current_block = 8903102000210989603;
        } else {
            append_transform(mAB_transform, &mut next_transform);
            current_block = 10692455896603418738;
        }
    } else {
        let mut transform: *mut qcms_modular_transform = qcms_modular_transform_alloc();
        if transform.is_null() {
            current_block = 8903102000210989603;
        } else {
            append_transform(transform, &mut next_transform);
            (*transform).input_clut_table_r = build_input_gamma_table((*in_0).redTRC);
            (*transform).input_clut_table_g = build_input_gamma_table((*in_0).greenTRC);
            (*transform).input_clut_table_b = build_input_gamma_table((*in_0).blueTRC);
            (*transform).transform_module_fn = Some(
                qcms_transform_module_gamma_table
                    as unsafe extern "C" fn(
                        _: *mut qcms_modular_transform,
                        _: *mut f32,
                        _: *mut f32,
                        _: size_t,
                    ) -> (),
            );
            if (*transform).input_clut_table_r.is_null()
                || (*transform).input_clut_table_g.is_null()
                || (*transform).input_clut_table_b.is_null()
            {
                current_block = 8903102000210989603;
            } else {
                transform = qcms_modular_transform_alloc();
                if transform.is_null() {
                    current_block = 8903102000210989603;
                } else {
                    append_transform(transform, &mut next_transform);
                    (*transform).matrix.m[0usize][0usize] = 1f32 / 1.999969482421875f32;
                    (*transform).matrix.m[0usize][1usize] = 0.0f32;
                    (*transform).matrix.m[0usize][2usize] = 0.0f32;
                    (*transform).matrix.m[1usize][0usize] = 0.0f32;
                    (*transform).matrix.m[1usize][1usize] = 1f32 / 1.999969482421875f32;
                    (*transform).matrix.m[1usize][2usize] = 0.0f32;
                    (*transform).matrix.m[2usize][0usize] = 0.0f32;
                    (*transform).matrix.m[2usize][1usize] = 0.0f32;
                    (*transform).matrix.m[2usize][2usize] = 1f32 / 1.999969482421875f32;
                    (*transform).matrix.invalid = 0i32 != 0;
                    (*transform).transform_module_fn = Some(
                        qcms_transform_module_matrix
                            as unsafe extern "C" fn(
                                _: *mut qcms_modular_transform,
                                _: *mut f32,
                                _: *mut f32,
                                _: size_t,
                            ) -> (),
                    );
                    transform = qcms_modular_transform_alloc();
                    if transform.is_null() {
                        current_block = 8903102000210989603;
                    } else {
                        append_transform(transform, &mut next_transform);
                        (*transform).matrix = build_colorant_matrix(in_0);
                        (*transform).transform_module_fn = Some(
                            qcms_transform_module_matrix
                                as unsafe extern "C" fn(
                                    _: *mut qcms_modular_transform,
                                    _: *mut f32,
                                    _: *mut f32,
                                    _: size_t,
                                ) -> (),
                        );
                        current_block = 10692455896603418738;
                    }
                }
            }
        }
    }
    match current_block {
        10692455896603418738 => return first_transform,
        _ => {
            qcms_modular_transform_release(first_transform);
            return 0 as *mut qcms_modular_transform;
        }
    };
}
unsafe extern "C" fn qcms_modular_transform_create_output(
    mut out: *mut qcms_profile,
) -> *mut qcms_modular_transform {
    let mut current_block: u64;
    let mut first_transform: *mut qcms_modular_transform = 0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform = &mut first_transform;
    if !(*out).B2A0.is_null() {
        let mut lut_transform: *mut qcms_modular_transform =
            qcms_modular_transform_create_lut((*out).B2A0);
        if lut_transform.is_null() {
            current_block = 15713701561912628542;
        } else {
            append_transform(lut_transform, &mut next_transform);
            current_block = 13131896068329595644;
        }
    } else if !(*out).mBA.is_null()
        && (*(*out).mBA).num_in_channels as i32 == 3i32
        && (*(*out).mBA).num_out_channels as i32 == 3i32
    {
        let mut lut_transform_0: *mut qcms_modular_transform =
            qcms_modular_transform_create_mAB((*out).mBA);
        if lut_transform_0.is_null() {
            current_block = 15713701561912628542;
        } else {
            append_transform(lut_transform_0, &mut next_transform);
            current_block = 13131896068329595644;
        }
    } else if !(*out).redTRC.is_null() && !(*out).greenTRC.is_null() && !(*out).blueTRC.is_null() {
        let mut transform: *mut qcms_modular_transform = qcms_modular_transform_alloc();
        if transform.is_null() {
            current_block = 15713701561912628542;
        } else {
            append_transform(transform, &mut next_transform);
            (*transform).matrix = matrix_invert(build_colorant_matrix(out));
            (*transform).transform_module_fn = Some(
                qcms_transform_module_matrix
                    as unsafe extern "C" fn(
                        _: *mut qcms_modular_transform,
                        _: *mut f32,
                        _: *mut f32,
                        _: size_t,
                    ) -> (),
            );
            transform = qcms_modular_transform_alloc();
            if transform.is_null() {
                current_block = 15713701561912628542;
            } else {
                append_transform(transform, &mut next_transform);
                (*transform).matrix.m[0usize][0usize] = 1.999969482421875f32;
                (*transform).matrix.m[0usize][1usize] = 0.0f32;
                (*transform).matrix.m[0usize][2usize] = 0.0f32;
                (*transform).matrix.m[1usize][0usize] = 0.0f32;
                (*transform).matrix.m[1usize][1usize] = 1.999969482421875f32;
                (*transform).matrix.m[1usize][2usize] = 0.0f32;
                (*transform).matrix.m[2usize][0usize] = 0.0f32;
                (*transform).matrix.m[2usize][1usize] = 0.0f32;
                (*transform).matrix.m[2usize][2usize] = 1.999969482421875f32;
                (*transform).matrix.invalid = 0i32 != 0;
                (*transform).transform_module_fn = Some(
                    qcms_transform_module_matrix
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                );
                transform = qcms_modular_transform_alloc();
                if transform.is_null() {
                    current_block = 15713701561912628542;
                } else {
                    append_transform(transform, &mut next_transform);
                    build_output_lut(
                        (*out).redTRC,
                        &mut (*transform).output_gamma_lut_r,
                        &mut (*transform).output_gamma_lut_r_length,
                    );
                    build_output_lut(
                        (*out).greenTRC,
                        &mut (*transform).output_gamma_lut_g,
                        &mut (*transform).output_gamma_lut_g_length,
                    );
                    build_output_lut(
                        (*out).blueTRC,
                        &mut (*transform).output_gamma_lut_b,
                        &mut (*transform).output_gamma_lut_b_length,
                    );
                    (*transform).transform_module_fn = Some(
                        qcms_transform_module_gamma_lut
                            as unsafe extern "C" fn(
                                _: *mut qcms_modular_transform,
                                _: *mut f32,
                                _: *mut f32,
                                _: size_t,
                            ) -> (),
                    );
                    if (*transform).output_gamma_lut_r.is_null()
                        || (*transform).output_gamma_lut_g.is_null()
                        || (*transform).output_gamma_lut_b.is_null()
                    {
                        current_block = 15713701561912628542;
                    } else {
                        current_block = 13131896068329595644;
                    }
                }
            }
        }
    } else {
        debug_assert!(false, "Unsupported output profile workflow.");
        return 0 as *mut qcms_modular_transform;
    }
    match current_block {
        13131896068329595644 => return first_transform,
        _ => {
            qcms_modular_transform_release(first_transform);
            return 0 as *mut qcms_modular_transform;
        }
    };
}
/* Not Completed
// Simplify the transformation chain to an equivalent transformation chain
static struct qcms_modular_transform* qcms_modular_transform_reduce(struct qcms_modular_transform *transform)
{
    struct qcms_modular_transform *first_transform = NULL;
    struct qcms_modular_transform *curr_trans = transform;
    struct qcms_modular_transform *prev_trans = NULL;
    while (curr_trans) {
        struct qcms_modular_transform *next_trans = curr_trans->next_transform;
        if (curr_trans->transform_module_fn == qcms_transform_module_matrix) {
            if (next_trans && next_trans->transform_module_fn == qcms_transform_module_matrix) {
                curr_trans->matrix = matrix_multiply(curr_trans->matrix, next_trans->matrix);
                goto remove_next;
            }
        }
        if (curr_trans->transform_module_fn == qcms_transform_module_gamma_table) {
            bool isLinear = true;
            uint16_t i;
            for (i = 0; isLinear && i < 256; i++) {
                isLinear &= (int)(curr_trans->input_clut_table_r[i] * 255) == i;
                isLinear &= (int)(curr_trans->input_clut_table_g[i] * 255) == i;
                isLinear &= (int)(curr_trans->input_clut_table_b[i] * 255) == i;
            }
            goto remove_current;
        }

next_transform:
        if (!next_trans) break;
        prev_trans = curr_trans;
        curr_trans = next_trans;
        continue;
remove_current:
        if (curr_trans == transform) {
            //Update head
            transform = next_trans;
        } else {
            prev_trans->next_transform = next_trans;
        }
        curr_trans->next_transform = NULL;
        qcms_modular_transform_release(curr_trans);
        //return transform;
        return qcms_modular_transform_reduce(transform);
remove_next:
        curr_trans->next_transform = next_trans->next_transform;
        next_trans->next_transform = NULL;
        qcms_modular_transform_release(next_trans);
        continue;
    }
    return transform;
}
*/
unsafe extern "C" fn qcms_modular_transform_create(
    mut in_0: *mut qcms_profile,
    mut out: *mut qcms_profile,
) -> *mut qcms_modular_transform {
    let mut current_block: u64;
    let mut first_transform: *mut qcms_modular_transform = 0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform = &mut first_transform;
    if (*in_0).color_space == 0x52474220u32 {
        let mut rgb_to_pcs: *mut qcms_modular_transform = qcms_modular_transform_create_input(in_0);
        if !rgb_to_pcs.is_null() {
            append_transform(rgb_to_pcs, &mut next_transform);
            if (*in_0).pcs == 0x4c616220u32 && (*out).pcs == 0x58595a20u32 {
                let mut lab_to_pcs: *mut qcms_modular_transform = qcms_modular_transform_alloc();
                if lab_to_pcs.is_null() {
                    current_block = 8418824557173580938;
                } else {
                    append_transform(lab_to_pcs, &mut next_transform);
                    (*lab_to_pcs).transform_module_fn = Some(
                        qcms_transform_module_LAB_to_XYZ
                            as unsafe extern "C" fn(
                                _: *mut qcms_modular_transform,
                                _: *mut f32,
                                _: *mut f32,
                                _: size_t,
                            ) -> (),
                    );
                    current_block = 10599921512955367680;
                }
            } else {
                current_block = 10599921512955367680;
            }
            match current_block {
                8418824557173580938 => {}
                _ =>
                // This does not improve accuracy in practice, something is wrong here.
                //if (in->chromaticAdaption.invalid == false) {
                //	struct qcms_modular_transform* chromaticAdaption;
                //	chromaticAdaption = qcms_modular_transform_alloc();
                //	if (!chromaticAdaption)
                //		goto fail;
                //	append_transform(chromaticAdaption, &next_transform);
                //	chromaticAdaption->matrix = matrix_invert(in->chromaticAdaption);
                //	chromaticAdaption->transform_module_fn = qcms_transform_module_matrix;
                //}
                {
                    if (*in_0).pcs == 0x58595a20u32 && (*out).pcs == 0x4c616220u32 {
                        let mut pcs_to_lab: *mut qcms_modular_transform =
                            qcms_modular_transform_alloc();
                        if pcs_to_lab.is_null() {
                            current_block = 8418824557173580938;
                        } else {
                            append_transform(pcs_to_lab, &mut next_transform);
                            (*pcs_to_lab).transform_module_fn = Some(
                                qcms_transform_module_XYZ_to_LAB
                                    as unsafe extern "C" fn(
                                        _: *mut qcms_modular_transform,
                                        _: *mut f32,
                                        _: *mut f32,
                                        _: size_t,
                                    )
                                        -> (),
                            );
                            current_block = 7175849428784450219;
                        }
                    } else {
                        current_block = 7175849428784450219;
                    }
                    match current_block {
                        8418824557173580938 => {}
                        _ => {
                            if (*out).color_space == 0x52474220u32 {
                                let mut pcs_to_rgb: *mut qcms_modular_transform =
                                    qcms_modular_transform_create_output(out);
                                if !pcs_to_rgb.is_null() {
                                    append_transform(pcs_to_rgb, &mut next_transform);
                                    // Not Completed
                                    //return qcms_modular_transform_reduce(first_transform);
                                    return first_transform;
                                }
                            } else {
                                debug_assert!(false, "output color space not supported");
                            }
                        }
                    }
                }
            }
        }
    } else {
        debug_assert!(false, "input color space not supported");
    }
    qcms_modular_transform_release(first_transform);
    return 0 as *mut qcms_modular_transform;
}
unsafe extern "C" fn qcms_modular_transform_data(
    mut transform: *mut qcms_modular_transform,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut len: size_t,
) -> *mut f32 {
    while !transform.is_null() {
        // Keep swaping src/dest when performing a transform to use less memory.
        let mut new_src: *mut f32 = dest;
        let transform_fn: transform_module_fn_t = (*transform).transform_module_fn;
        if transform_fn
            != Some(
                qcms_transform_module_gamma_table
                    as unsafe extern "C" fn(
                        _: *mut qcms_modular_transform,
                        _: *mut f32,
                        _: *mut f32,
                        _: size_t,
                    ) -> (),
            )
            && transform_fn
                != Some(
                    qcms_transform_module_gamma_lut
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                )
            && transform_fn
                != Some(
                    qcms_transform_module_clut
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                )
            && transform_fn
                != Some(
                    qcms_transform_module_clut_only
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                )
            && transform_fn
                != Some(
                    qcms_transform_module_matrix
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                )
            && transform_fn
                != Some(
                    qcms_transform_module_matrix_translate
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                )
            && transform_fn
                != Some(
                    qcms_transform_module_LAB_to_XYZ
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                )
            && transform_fn
                != Some(
                    qcms_transform_module_XYZ_to_LAB
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                )
        {
            debug_assert!(false, "Unsupported transform module");
            return 0 as *mut f32;
        }
        if (*transform).grid_size as i32 <= 0i32
            && (transform_fn
                == Some(
                    qcms_transform_module_clut
                        as unsafe extern "C" fn(
                            _: *mut qcms_modular_transform,
                            _: *mut f32,
                            _: *mut f32,
                            _: size_t,
                        ) -> (),
                )
                || transform_fn
                    == Some(
                        qcms_transform_module_clut_only
                            as unsafe extern "C" fn(
                                _: *mut qcms_modular_transform,
                                _: *mut f32,
                                _: *mut f32,
                                _: size_t,
                            ) -> (),
                    ))
        {
            debug_assert!(false, "Invalid transform");
            return 0 as *mut f32;
        }
        (*transform)
            .transform_module_fn
            .expect("non-null function pointer")(transform, src, dest, len);
        dest = src;
        src = new_src;
        transform = (*transform).next_transform
    }
    // The results end up in the src buffer because of the switching
    return src;
}
#[no_mangle]
pub unsafe extern "C" fn qcms_chain_transform(
    mut in_0: *mut qcms_profile,
    mut out: *mut qcms_profile,
    mut src: *mut f32,
    mut dest: *mut f32,
    mut lutSize: size_t,
) -> *mut f32 {
    let mut transform_list: *mut qcms_modular_transform = qcms_modular_transform_create(in_0, out);
    if !transform_list.is_null() {
        let mut lut: *mut f32 = qcms_modular_transform_data(transform_list, src, dest, lutSize / 3);
        qcms_modular_transform_release(transform_list);
        return lut;
    }
    return 0 as *mut f32;
}
