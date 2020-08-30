use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceilf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn floorf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn lut_interp_linear(value: libc::c_double, table: *mut uint16_t,
                         length: libc::c_int) -> libc::c_float;
    #[no_mangle]
    fn lut_interp_linear_float(value: libc::c_float,
                               table: *mut libc::c_float, length: libc::c_int)
     -> libc::c_float;
    #[no_mangle]
    fn build_input_gamma_table(TRC: *mut curveType) -> *mut libc::c_float;
    #[no_mangle]
    fn build_colorant_matrix(p: *mut qcms_profile) -> matrix;
    #[no_mangle]
    fn build_output_lut(trc: *mut curveType,
                        output_gamma_lut: *mut *mut uint16_t,
                        output_gamma_lut_length: *mut size_t);
    #[no_mangle]
    fn matrix_invert(mat: matrix) -> matrix;
}
pub type __darwin_size_t = libc::c_ulong;
pub type int32_t = libc::c_int;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct precache_output {
    pub ref_count: libc::c_int,
    pub data: [uint8_t; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _qcms_profile {
    pub class_type: uint32_t,
    pub color_space: uint32_t,
    pub pcs: uint32_t,
    pub rendering_intent: qcms_intent,
    pub redColorant: XYZNumber,
    pub blueColorant: XYZNumber,
    pub greenColorant: XYZNumber,
    pub redTRC: *mut curveType,
    pub blueTRC: *mut curveType,
    pub greenTRC: *mut curveType,
    pub grayTRC: *mut curveType,
    pub A2B0: *mut lutType,
    pub B2A0: *mut lutType,
    pub mAB: *mut lutmABType,
    pub mBA: *mut lutmABType,
    pub chromaticAdaption: matrix,
    pub output_table_r: *mut precache_output,
    pub output_table_g: *mut precache_output,
    pub output_table_b: *mut precache_output,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matrix {
    pub m: [[libc::c_float; 3]; 3],
    pub invalid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lutmABType {
    pub num_in_channels: uint8_t,
    pub num_out_channels: uint8_t,
    pub num_grid_points: [uint8_t; 16],
    pub e00: s15Fixed16Number,
    pub e01: s15Fixed16Number,
    pub e02: s15Fixed16Number,
    pub e03: s15Fixed16Number,
    pub e10: s15Fixed16Number,
    pub e11: s15Fixed16Number,
    pub e12: s15Fixed16Number,
    pub e13: s15Fixed16Number,
    pub e20: s15Fixed16Number,
    pub e21: s15Fixed16Number,
    pub e22: s15Fixed16Number,
    pub e23: s15Fixed16Number,
    pub reversed: bool,
    pub clut_table: *mut libc::c_float,
    pub a_curves: [*mut curveType; 10],
    pub b_curves: [*mut curveType; 10],
    pub m_curves: [*mut curveType; 10],
    pub clut_table_data: [libc::c_float; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curveType {
    pub type_0: uint32_t,
    pub count: uint32_t,
    pub parameter: [libc::c_float; 7],
    pub data: [uInt16Number; 0],
}
pub type uInt16Number = uint16_t;
pub type s15Fixed16Number = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lutType {
    pub num_input_channels: uint8_t,
    pub num_output_channels: uint8_t,
    pub num_clut_grid_points: uint8_t,
    pub e00: s15Fixed16Number,
    pub e01: s15Fixed16Number,
    pub e02: s15Fixed16Number,
    pub e10: s15Fixed16Number,
    pub e11: s15Fixed16Number,
    pub e12: s15Fixed16Number,
    pub e20: s15Fixed16Number,
    pub e21: s15Fixed16Number,
    pub e22: s15Fixed16Number,
    pub num_input_table_entries: uint16_t,
    pub num_output_table_entries: uint16_t,
    pub input_table: *mut libc::c_float,
    pub clut_table: *mut libc::c_float,
    pub output_table: *mut libc::c_float,
    pub table_data: [libc::c_float; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XYZNumber {
    pub X: s15Fixed16Number,
    pub Y: s15Fixed16Number,
    pub Z: s15Fixed16Number,
}
pub type qcms_intent = libc::c_uint;
pub const QCMS_INTENT_DEFAULT: qcms_intent = 0;
pub const QCMS_INTENT_MAX: qcms_intent = 3;
pub const QCMS_INTENT_ABSOLUTE_COLORIMETRIC: qcms_intent = 3;
pub const QCMS_INTENT_SATURATION: qcms_intent = 2;
pub const QCMS_INTENT_RELATIVE_COLORIMETRIC: qcms_intent = 1;
pub const QCMS_INTENT_PERCEPTUAL: qcms_intent = 0;
pub const QCMS_INTENT_MIN: qcms_intent = 0;
pub type qcms_profile = _qcms_profile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qcms_modular_transform {
    pub matrix: matrix,
    pub tx: libc::c_float,
    pub ty: libc::c_float,
    pub tz: libc::c_float,
    pub input_clut_table_r: *mut libc::c_float,
    pub input_clut_table_g: *mut libc::c_float,
    pub input_clut_table_b: *mut libc::c_float,
    pub input_clut_table_length: uint16_t,
    pub r_clut: *mut libc::c_float,
    pub g_clut: *mut libc::c_float,
    pub b_clut: *mut libc::c_float,
    pub grid_size: uint16_t,
    pub output_clut_table_r: *mut libc::c_float,
    pub output_clut_table_g: *mut libc::c_float,
    pub output_clut_table_b: *mut libc::c_float,
    pub output_clut_table_length: uint16_t,
    pub output_gamma_lut_r: *mut uint16_t,
    pub output_gamma_lut_g: *mut uint16_t,
    pub output_gamma_lut_b: *mut uint16_t,
    pub output_gamma_lut_r_length: size_t,
    pub output_gamma_lut_g_length: size_t,
    pub output_gamma_lut_b_length: size_t,
    pub transform_module_fn: transform_module_fn_t,
    pub next_transform: *mut qcms_modular_transform,
}
pub type transform_module_fn_t
    =
    Option<unsafe extern "C" fn(_: *mut qcms_modular_transform,
                                _: *mut libc::c_float, _: *mut libc::c_float,
                                _: size_t) -> ()>;
#[inline]
unsafe extern "C" fn s15Fixed16Number_to_float(mut a: s15Fixed16Number)
 -> libc::c_float {
    return a as libc::c_float / 65536.0f32;
}
#[inline]
unsafe extern "C" fn lerp(mut a: libc::c_float, mut b: libc::c_float,
                          mut t: libc::c_float) -> libc::c_float {
    return a * (1.0f32 - t) + b * t;
}
#[inline]
unsafe extern "C" fn clamp_float(mut a: libc::c_float) -> libc::c_float {
    /* One would naturally write this function as the following:
  if (a > 1.)
    return 1.;
  else if (a < 0)
    return 0;
  else
    return a;

  However, that version will let NaNs pass through which is undesirable
  for most consumers.
  */
    if a as libc::c_double > 1.0f64 {
        return 1.0f64 as libc::c_float
    } else if a >= 0 as libc::c_int as libc::c_float {
        return a
    } else {
        // a < 0 or a is NaN
        return 0 as libc::c_int as libc::c_float
    };
}
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
//memcpy
unsafe extern "C" fn build_lut_matrix(mut lut: *mut lutType) -> matrix {
    let mut result: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    if !lut.is_null() {
        result.m[0 as libc::c_int as usize][0 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e00);
        result.m[0 as libc::c_int as usize][1 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e01);
        result.m[0 as libc::c_int as usize][2 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e02);
        result.m[1 as libc::c_int as usize][0 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e10);
        result.m[1 as libc::c_int as usize][1 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e11);
        result.m[1 as libc::c_int as usize][2 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e12);
        result.m[2 as libc::c_int as usize][0 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e20);
        result.m[2 as libc::c_int as usize][1 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e21);
        result.m[2 as libc::c_int as usize][2 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e22);
        result.invalid = 0 as libc::c_int != 0
    } else {
        memset(&mut result as *mut matrix as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<matrix>() as libc::c_ulong);
        result.invalid = 1 as libc::c_int != 0
    }
    return result;
}
unsafe extern "C" fn build_mAB_matrix(mut lut: *mut lutmABType) -> matrix {
    let mut result: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    if !lut.is_null() {
        result.m[0 as libc::c_int as usize][0 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e00);
        result.m[0 as libc::c_int as usize][1 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e01);
        result.m[0 as libc::c_int as usize][2 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e02);
        result.m[1 as libc::c_int as usize][0 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e10);
        result.m[1 as libc::c_int as usize][1 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e11);
        result.m[1 as libc::c_int as usize][2 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e12);
        result.m[2 as libc::c_int as usize][0 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e20);
        result.m[2 as libc::c_int as usize][1 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e21);
        result.m[2 as libc::c_int as usize][2 as libc::c_int as usize] =
            s15Fixed16Number_to_float((*lut).e22);
        result.invalid = 0 as libc::c_int != 0
    } else {
        memset(&mut result as *mut matrix as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<matrix>() as libc::c_ulong);
        result.invalid = 1 as libc::c_int != 0
    }
    return result;
}
//Based on lcms cmsLab2XYZ
unsafe extern "C" fn qcms_transform_module_LAB_to_XYZ(mut transform:
                                                          *mut qcms_modular_transform,
                                                      mut src:
                                                          *mut libc::c_float,
                                                      mut dest:
                                                          *mut libc::c_float,
                                                      mut length: size_t) {
    let mut i: size_t = 0;
    // lcms: D50 XYZ values
    let mut WhitePointX: libc::c_float = 0.9642f32;
    let mut WhitePointY: libc::c_float = 1.0f32;
    let mut WhitePointZ: libc::c_float = 0.8249f32;
    i = 0 as libc::c_int as size_t;
    while i < length {
        let fresh0 = src;
        src = src.offset(1);
        let mut device_L: libc::c_float = *fresh0 * 100.0f32;
        let fresh1 = src;
        src = src.offset(1);
        let mut device_a: libc::c_float = *fresh1 * 255.0f32 - 128.0f32;
        let fresh2 = src;
        src = src.offset(1);
        let mut device_b: libc::c_float = *fresh2 * 255.0f32 - 128.0f32;
        let mut y: libc::c_float = (device_L + 16.0f32) / 116.0f32;
        let mut X: libc::c_float =
            if y + 0.002f32 * device_a <= 24.0f32 / 116.0f32 {
                (108.0f64 / 841.0f64) *
                    ((y + 0.002f32 * device_a) as libc::c_double -
                         16.0f64 / 116.0f64)
            } else {
                ((y + 0.002f32 * device_a) * (y + 0.002f32 * device_a) *
                     (y + 0.002f32 * device_a) * WhitePointX) as
                    libc::c_double
            } as libc::c_float;
        let mut Y: libc::c_float =
            if y <= 24.0f32 / 116.0f32 {
                (108.0f64 / 841.0f64) *
                    (y as libc::c_double - 16.0f64 / 116.0f64)
            } else { (y * y * y * WhitePointY) as libc::c_double } as
                libc::c_float;
        let mut Z: libc::c_float =
            if y - 0.005f32 * device_b <= 24.0f32 / 116.0f32 {
                (108.0f64 / 841.0f64) *
                    ((y - 0.005f32 * device_b) as libc::c_double -
                         16.0f64 / 116.0f64)
            } else {
                ((y - 0.005f32 * device_b) * (y - 0.005f32 * device_b) *
                     (y - 0.005f32 * device_b) * WhitePointZ) as
                    libc::c_double
            } as libc::c_float;
        let fresh3 = dest;
        dest = dest.offset(1);
        *fresh3 =
            (X as libc::c_double / (1.0f64 + 32767.0f64 / 32768.0f64)) as
                libc::c_float;
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 =
            (Y as libc::c_double / (1.0f64 + 32767.0f64 / 32768.0f64)) as
                libc::c_float;
        let fresh5 = dest;
        dest = dest.offset(1);
        *fresh5 =
            (Z as libc::c_double / (1.0f64 + 32767.0f64 / 32768.0f64)) as
                libc::c_float;
        i = i.wrapping_add(1)
    };
}
//Based on lcms cmsXYZ2Lab
unsafe extern "C" fn qcms_transform_module_XYZ_to_LAB(mut transform:
                                                          *mut qcms_modular_transform,
                                                      mut src:
                                                          *mut libc::c_float,
                                                      mut dest:
                                                          *mut libc::c_float,
                                                      mut length: size_t) {
    let mut i: size_t = 0;
    // lcms: D50 XYZ values
    let mut WhitePointX: libc::c_float = 0.9642f32;
    let mut WhitePointY: libc::c_float = 1.0f32;
    let mut WhitePointZ: libc::c_float = 0.8249f32;
    i = 0 as libc::c_int as size_t;
    while i < length {
        let fresh6 = src;
        src = src.offset(1);
        let mut device_x: libc::c_float =
            (*fresh6 as libc::c_double * (1.0f64 + 32767.0f64 / 32768.0f64) /
                 WhitePointX as libc::c_double) as libc::c_float;
        let fresh7 = src;
        src = src.offset(1);
        let mut device_y: libc::c_float =
            (*fresh7 as libc::c_double * (1.0f64 + 32767.0f64 / 32768.0f64) /
                 WhitePointY as libc::c_double) as libc::c_float;
        let fresh8 = src;
        src = src.offset(1);
        let mut device_z: libc::c_float =
            (*fresh8 as libc::c_double * (1.0f64 + 32767.0f64 / 32768.0f64) /
                 WhitePointZ as libc::c_double) as libc::c_float;
        let mut fx: libc::c_float =
            if device_x <=
                   24.0f32 / 116.0f32 * (24.0f32 / 116.0f32) *
                       (24.0f32 / 116.0f32) {
                (841.0f64 / 108.0f64 * device_x as libc::c_double) +
                    16.0f64 / 116.0f64
            } else { pow(device_x as libc::c_double, 1.0f64 / 3.0f64) } as
                libc::c_float;
        let mut fy: libc::c_float =
            if device_y <=
                   24.0f32 / 116.0f32 * (24.0f32 / 116.0f32) *
                       (24.0f32 / 116.0f32) {
                (841.0f64 / 108.0f64 * device_y as libc::c_double) +
                    16.0f64 / 116.0f64
            } else { pow(device_y as libc::c_double, 1.0f64 / 3.0f64) } as
                libc::c_float;
        let mut fz: libc::c_float =
            if device_z <=
                   24.0f32 / 116.0f32 * (24.0f32 / 116.0f32) *
                       (24.0f32 / 116.0f32) {
                (841.0f64 / 108.0f64 * device_z as libc::c_double) +
                    16.0f64 / 116.0f64
            } else { pow(device_z as libc::c_double, 1.0f64 / 3.0f64) } as
                libc::c_float;
        let mut L: libc::c_float = 116.0f32 * fy - 16.0f32;
        let mut a: libc::c_float = 500.0f32 * (fx - fy);
        let mut b: libc::c_float = 200.0f32 * (fy - fz);
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = L / 100.0f32;
        let fresh10 = dest;
        dest = dest.offset(1);
        *fresh10 = (a + 128.0f32) / 255.0f32;
        let fresh11 = dest;
        dest = dest.offset(1);
        *fresh11 = (b + 128.0f32) / 255.0f32;
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn qcms_transform_module_clut_only(mut transform:
                                                         *mut qcms_modular_transform,
                                                     mut src:
                                                         *mut libc::c_float,
                                                     mut dest:
                                                         *mut libc::c_float,
                                                     mut length: size_t) {
    let mut i: size_t = 0;
    let mut xy_len: libc::c_int = 1 as libc::c_int;
    let mut x_len: libc::c_int = (*transform).grid_size as libc::c_int;
    let mut len: libc::c_int = x_len * x_len;
    let mut r_table: *mut libc::c_float = (*transform).r_clut;
    let mut g_table: *mut libc::c_float = (*transform).g_clut;
    let mut b_table: *mut libc::c_float = (*transform).b_clut;
    i = 0 as libc::c_int as size_t;
    while i < length {
        if !((*transform).grid_size as libc::c_int >= 1 as libc::c_int) as
               libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 32],
                                                   &[libc::c_char; 32]>(b"qcms_transform_module_clut_only\x00")).as_ptr(),
                         b"chain.c\x00" as *const u8 as *const libc::c_char,
                         137 as libc::c_int,
                         b"transform->grid_size >= 1\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        let fresh12 = src;
        src = src.offset(1);
        let mut linear_r: libc::c_float = *fresh12;
        let fresh13 = src;
        src = src.offset(1);
        let mut linear_g: libc::c_float = *fresh13;
        let fresh14 = src;
        src = src.offset(1);
        let mut linear_b: libc::c_float = *fresh14;
        let mut x: libc::c_int =
            floorf(linear_r *
                       ((*transform).grid_size as libc::c_int -
                            1 as libc::c_int) as libc::c_float) as
                libc::c_int;
        let mut y: libc::c_int =
            floorf(linear_g *
                       ((*transform).grid_size as libc::c_int -
                            1 as libc::c_int) as libc::c_float) as
                libc::c_int;
        let mut z: libc::c_int =
            floorf(linear_b *
                       ((*transform).grid_size as libc::c_int -
                            1 as libc::c_int) as libc::c_float) as
                libc::c_int;
        let mut x_n: libc::c_int =
            ceilf(linear_r *
                      ((*transform).grid_size as libc::c_int -
                           1 as libc::c_int) as libc::c_float) as libc::c_int;
        let mut y_n: libc::c_int =
            ceilf(linear_g *
                      ((*transform).grid_size as libc::c_int -
                           1 as libc::c_int) as libc::c_float) as libc::c_int;
        let mut z_n: libc::c_int =
            ceilf(linear_b *
                      ((*transform).grid_size as libc::c_int -
                           1 as libc::c_int) as libc::c_float) as libc::c_int;
        let mut x_d: libc::c_float =
            linear_r *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) as
                    libc::c_float - x as libc::c_float;
        let mut y_d: libc::c_float =
            linear_g *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) as
                    libc::c_float - y as libc::c_float;
        let mut z_d: libc::c_float =
            linear_b *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) as
                    libc::c_float - z as libc::c_float;
        let mut r_x1: libc::c_float =
            lerp(*r_table.offset(((x * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *r_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut r_x2: libc::c_float =
            lerp(*r_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *r_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut r_y1: libc::c_float = lerp(r_x1, r_x2, y_d);
        let mut r_x3: libc::c_float =
            lerp(*r_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *r_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut r_x4: libc::c_float =
            lerp(*r_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut r_y2: libc::c_float = lerp(r_x3, r_x4, y_d);
        let mut clut_r: libc::c_float = lerp(r_y1, r_y2, z_d);
        let mut g_x1: libc::c_float =
            lerp(*g_table.offset(((x * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *g_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut g_x2: libc::c_float =
            lerp(*g_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *g_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut g_y1: libc::c_float = lerp(g_x1, g_x2, y_d);
        let mut g_x3: libc::c_float =
            lerp(*g_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *g_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut g_x4: libc::c_float =
            lerp(*g_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut g_y2: libc::c_float = lerp(g_x3, g_x4, y_d);
        let mut clut_g: libc::c_float = lerp(g_y1, g_y2, z_d);
        let mut b_x1: libc::c_float =
            lerp(*b_table.offset(((x * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *b_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut b_x2: libc::c_float =
            lerp(*b_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *b_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut b_y1: libc::c_float = lerp(b_x1, b_x2, y_d);
        let mut b_x3: libc::c_float =
            lerp(*b_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *b_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut b_x4: libc::c_float =
            lerp(*b_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut b_y2: libc::c_float = lerp(b_x3, b_x4, y_d);
        let mut clut_b: libc::c_float = lerp(b_y1, b_y2, z_d);
        let fresh15 = dest;
        dest = dest.offset(1);
        *fresh15 = clamp_float(clut_r);
        let fresh16 = dest;
        dest = dest.offset(1);
        *fresh16 = clamp_float(clut_g);
        let fresh17 = dest;
        dest = dest.offset(1);
        *fresh17 = clamp_float(clut_b);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn qcms_transform_module_clut(mut transform:
                                                    *mut qcms_modular_transform,
                                                mut src: *mut libc::c_float,
                                                mut dest: *mut libc::c_float,
                                                mut length: size_t) {
    let mut i: size_t = 0;
    let mut xy_len: libc::c_int = 1 as libc::c_int;
    let mut x_len: libc::c_int = (*transform).grid_size as libc::c_int;
    let mut len: libc::c_int = x_len * x_len;
    let mut r_table: *mut libc::c_float = (*transform).r_clut;
    let mut g_table: *mut libc::c_float = (*transform).g_clut;
    let mut b_table: *mut libc::c_float = (*transform).b_clut;
    i = 0 as libc::c_int as size_t;
    while i < length {
        if !((*transform).grid_size as libc::c_int >= 1 as libc::c_int) as
               libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 27],
                                                   &[libc::c_char; 27]>(b"qcms_transform_module_clut\x00")).as_ptr(),
                         b"chain.c\x00" as *const u8 as *const libc::c_char,
                         193 as libc::c_int,
                         b"transform->grid_size >= 1\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        let fresh18 = src;
        src = src.offset(1);
        let mut device_r: libc::c_float = *fresh18;
        let fresh19 = src;
        src = src.offset(1);
        let mut device_g: libc::c_float = *fresh19;
        let fresh20 = src;
        src = src.offset(1);
        let mut device_b: libc::c_float = *fresh20;
        let mut linear_r: libc::c_float =
            lut_interp_linear_float(device_r, (*transform).input_clut_table_r,
                                    (*transform).input_clut_table_length as
                                        libc::c_int);
        let mut linear_g: libc::c_float =
            lut_interp_linear_float(device_g, (*transform).input_clut_table_g,
                                    (*transform).input_clut_table_length as
                                        libc::c_int);
        let mut linear_b: libc::c_float =
            lut_interp_linear_float(device_b, (*transform).input_clut_table_b,
                                    (*transform).input_clut_table_length as
                                        libc::c_int);
        let mut x: libc::c_int =
            floorf(linear_r *
                       ((*transform).grid_size as libc::c_int -
                            1 as libc::c_int) as libc::c_float) as
                libc::c_int;
        let mut y: libc::c_int =
            floorf(linear_g *
                       ((*transform).grid_size as libc::c_int -
                            1 as libc::c_int) as libc::c_float) as
                libc::c_int;
        let mut z: libc::c_int =
            floorf(linear_b *
                       ((*transform).grid_size as libc::c_int -
                            1 as libc::c_int) as libc::c_float) as
                libc::c_int;
        let mut x_n: libc::c_int =
            ceilf(linear_r *
                      ((*transform).grid_size as libc::c_int -
                           1 as libc::c_int) as libc::c_float) as libc::c_int;
        let mut y_n: libc::c_int =
            ceilf(linear_g *
                      ((*transform).grid_size as libc::c_int -
                           1 as libc::c_int) as libc::c_float) as libc::c_int;
        let mut z_n: libc::c_int =
            ceilf(linear_b *
                      ((*transform).grid_size as libc::c_int -
                           1 as libc::c_int) as libc::c_float) as libc::c_int;
        let mut x_d: libc::c_float =
            linear_r *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) as
                    libc::c_float - x as libc::c_float;
        let mut y_d: libc::c_float =
            linear_g *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) as
                    libc::c_float - y as libc::c_float;
        let mut z_d: libc::c_float =
            linear_b *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) as
                    libc::c_float - z as libc::c_float;
        let mut r_x1: libc::c_float =
            lerp(*r_table.offset(((x * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *r_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut r_x2: libc::c_float =
            lerp(*r_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *r_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut r_y1: libc::c_float = lerp(r_x1, r_x2, y_d);
        let mut r_x3: libc::c_float =
            lerp(*r_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *r_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut r_x4: libc::c_float =
            lerp(*r_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut r_y2: libc::c_float = lerp(r_x3, r_x4, y_d);
        let mut clut_r: libc::c_float = lerp(r_y1, r_y2, z_d);
        let mut g_x1: libc::c_float =
            lerp(*g_table.offset(((x * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *g_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut g_x2: libc::c_float =
            lerp(*g_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *g_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut g_y1: libc::c_float = lerp(g_x1, g_x2, y_d);
        let mut g_x3: libc::c_float =
            lerp(*g_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *g_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut g_x4: libc::c_float =
            lerp(*g_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut g_y2: libc::c_float = lerp(g_x3, g_x4, y_d);
        let mut clut_g: libc::c_float = lerp(g_y1, g_y2, z_d);
        let mut b_x1: libc::c_float =
            lerp(*b_table.offset(((x * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *b_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut b_x2: libc::c_float =
            lerp(*b_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize),
                 *b_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut b_y1: libc::c_float = lerp(b_x1, b_x2, y_d);
        let mut b_x3: libc::c_float =
            lerp(*b_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *b_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut b_x4: libc::c_float =
            lerp(*b_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize),
                 *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                      3 as libc::c_int) as isize), x_d);
        let mut b_y2: libc::c_float = lerp(b_x3, b_x4, y_d);
        let mut clut_b: libc::c_float = lerp(b_y1, b_y2, z_d);
        let mut pcs_r: libc::c_float =
            lut_interp_linear_float(clut_r, (*transform).output_clut_table_r,
                                    (*transform).output_clut_table_length as
                                        libc::c_int);
        let mut pcs_g: libc::c_float =
            lut_interp_linear_float(clut_g, (*transform).output_clut_table_g,
                                    (*transform).output_clut_table_length as
                                        libc::c_int);
        let mut pcs_b: libc::c_float =
            lut_interp_linear_float(clut_b, (*transform).output_clut_table_b,
                                    (*transform).output_clut_table_length as
                                        libc::c_int);
        let fresh21 = dest;
        dest = dest.offset(1);
        *fresh21 = clamp_float(pcs_r);
        let fresh22 = dest;
        dest = dest.offset(1);
        *fresh22 = clamp_float(pcs_g);
        let fresh23 = dest;
        dest = dest.offset(1);
        *fresh23 = clamp_float(pcs_b);
        i = i.wrapping_add(1)
    };
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
unsafe extern "C" fn qcms_transform_module_gamma_table(mut transform:
                                                           *mut qcms_modular_transform,
                                                       mut src:
                                                           *mut libc::c_float,
                                                       mut dest:
                                                           *mut libc::c_float,
                                                       mut length: size_t) {
    let mut i: size_t = 0;
    let mut out_r: libc::c_float = 0.;
    let mut out_g: libc::c_float = 0.;
    let mut out_b: libc::c_float = 0.;
    i = 0 as libc::c_int as size_t;
    while i < length {
        let fresh24 = src;
        src = src.offset(1);
        let mut in_r: libc::c_float = *fresh24;
        let fresh25 = src;
        src = src.offset(1);
        let mut in_g: libc::c_float = *fresh25;
        let fresh26 = src;
        src = src.offset(1);
        let mut in_b: libc::c_float = *fresh26;
        out_r =
            lut_interp_linear_float(in_r, (*transform).input_clut_table_r,
                                    256 as libc::c_int);
        out_g =
            lut_interp_linear_float(in_g, (*transform).input_clut_table_g,
                                    256 as libc::c_int);
        out_b =
            lut_interp_linear_float(in_b, (*transform).input_clut_table_b,
                                    256 as libc::c_int);
        let fresh27 = dest;
        dest = dest.offset(1);
        *fresh27 = clamp_float(out_r);
        let fresh28 = dest;
        dest = dest.offset(1);
        *fresh28 = clamp_float(out_g);
        let fresh29 = dest;
        dest = dest.offset(1);
        *fresh29 = clamp_float(out_b);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn qcms_transform_module_gamma_lut(mut transform:
                                                         *mut qcms_modular_transform,
                                                     mut src:
                                                         *mut libc::c_float,
                                                     mut dest:
                                                         *mut libc::c_float,
                                                     mut length: size_t) {
    let mut i: size_t = 0;
    let mut out_r: libc::c_float = 0.;
    let mut out_g: libc::c_float = 0.;
    let mut out_b: libc::c_float = 0.;
    i = 0 as libc::c_int as size_t;
    while i < length {
        let fresh30 = src;
        src = src.offset(1);
        let mut in_r: libc::c_float = *fresh30;
        let fresh31 = src;
        src = src.offset(1);
        let mut in_g: libc::c_float = *fresh31;
        let fresh32 = src;
        src = src.offset(1);
        let mut in_b: libc::c_float = *fresh32;
        out_r =
            lut_interp_linear(in_r as libc::c_double,
                              (*transform).output_gamma_lut_r,
                              (*transform).output_gamma_lut_r_length as
                                  libc::c_int);
        out_g =
            lut_interp_linear(in_g as libc::c_double,
                              (*transform).output_gamma_lut_g,
                              (*transform).output_gamma_lut_g_length as
                                  libc::c_int);
        out_b =
            lut_interp_linear(in_b as libc::c_double,
                              (*transform).output_gamma_lut_b,
                              (*transform).output_gamma_lut_b_length as
                                  libc::c_int);
        let fresh33 = dest;
        dest = dest.offset(1);
        *fresh33 = clamp_float(out_r);
        let fresh34 = dest;
        dest = dest.offset(1);
        *fresh34 = clamp_float(out_g);
        let fresh35 = dest;
        dest = dest.offset(1);
        *fresh35 = clamp_float(out_b);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn qcms_transform_module_matrix_translate(mut transform:
                                                                *mut qcms_modular_transform,
                                                            mut src:
                                                                *mut libc::c_float,
                                                            mut dest:
                                                                *mut libc::c_float,
                                                            mut length:
                                                                size_t) {
    let mut i: size_t = 0;
    let mut mat: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    /* store the results in column major mode
	 * this makes doing the multiplication with sse easier */
    mat.m[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*transform).matrix.m[0 as libc::c_int as
                                  usize][0 as libc::c_int as usize];
    mat.m[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*transform).matrix.m[0 as libc::c_int as
                                  usize][1 as libc::c_int as usize];
    mat.m[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*transform).matrix.m[0 as libc::c_int as
                                  usize][2 as libc::c_int as usize];
    mat.m[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*transform).matrix.m[1 as libc::c_int as
                                  usize][0 as libc::c_int as usize];
    mat.m[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*transform).matrix.m[1 as libc::c_int as
                                  usize][1 as libc::c_int as usize];
    mat.m[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*transform).matrix.m[1 as libc::c_int as
                                  usize][2 as libc::c_int as usize];
    mat.m[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*transform).matrix.m[2 as libc::c_int as
                                  usize][0 as libc::c_int as usize];
    mat.m[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*transform).matrix.m[2 as libc::c_int as
                                  usize][1 as libc::c_int as usize];
    mat.m[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*transform).matrix.m[2 as libc::c_int as
                                  usize][2 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < length {
        let fresh36 = src;
        src = src.offset(1);
        let mut in_r: libc::c_float = *fresh36;
        let fresh37 = src;
        src = src.offset(1);
        let mut in_g: libc::c_float = *fresh37;
        let fresh38 = src;
        src = src.offset(1);
        let mut in_b: libc::c_float = *fresh38;
        let mut out_r: libc::c_float =
            mat.m[0 as libc::c_int as usize][0 as libc::c_int as usize] * in_r
                +
                mat.m[1 as libc::c_int as usize][0 as libc::c_int as usize] *
                    in_g +
                mat.m[2 as libc::c_int as usize][0 as libc::c_int as usize] *
                    in_b + (*transform).tx;
        let mut out_g: libc::c_float =
            mat.m[0 as libc::c_int as usize][1 as libc::c_int as usize] * in_r
                +
                mat.m[1 as libc::c_int as usize][1 as libc::c_int as usize] *
                    in_g +
                mat.m[2 as libc::c_int as usize][1 as libc::c_int as usize] *
                    in_b + (*transform).ty;
        let mut out_b: libc::c_float =
            mat.m[0 as libc::c_int as usize][2 as libc::c_int as usize] * in_r
                +
                mat.m[1 as libc::c_int as usize][2 as libc::c_int as usize] *
                    in_g +
                mat.m[2 as libc::c_int as usize][2 as libc::c_int as usize] *
                    in_b + (*transform).tz;
        let fresh39 = dest;
        dest = dest.offset(1);
        *fresh39 = clamp_float(out_r);
        let fresh40 = dest;
        dest = dest.offset(1);
        *fresh40 = clamp_float(out_g);
        let fresh41 = dest;
        dest = dest.offset(1);
        *fresh41 = clamp_float(out_b);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn qcms_transform_module_matrix(mut transform:
                                                      *mut qcms_modular_transform,
                                                  mut src: *mut libc::c_float,
                                                  mut dest:
                                                      *mut libc::c_float,
                                                  mut length: size_t) {
    let mut i: size_t = 0;
    let mut mat: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    /* store the results in column major mode
	 * this makes doing the multiplication with sse easier */
    mat.m[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*transform).matrix.m[0 as libc::c_int as
                                  usize][0 as libc::c_int as usize];
    mat.m[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*transform).matrix.m[0 as libc::c_int as
                                  usize][1 as libc::c_int as usize];
    mat.m[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*transform).matrix.m[0 as libc::c_int as
                                  usize][2 as libc::c_int as usize];
    mat.m[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*transform).matrix.m[1 as libc::c_int as
                                  usize][0 as libc::c_int as usize];
    mat.m[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*transform).matrix.m[1 as libc::c_int as
                                  usize][1 as libc::c_int as usize];
    mat.m[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*transform).matrix.m[1 as libc::c_int as
                                  usize][2 as libc::c_int as usize];
    mat.m[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*transform).matrix.m[2 as libc::c_int as
                                  usize][0 as libc::c_int as usize];
    mat.m[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*transform).matrix.m[2 as libc::c_int as
                                  usize][1 as libc::c_int as usize];
    mat.m[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*transform).matrix.m[2 as libc::c_int as
                                  usize][2 as libc::c_int as usize];
    i = 0 as libc::c_int as size_t;
    while i < length {
        let fresh42 = src;
        src = src.offset(1);
        let mut in_r: libc::c_float = *fresh42;
        let fresh43 = src;
        src = src.offset(1);
        let mut in_g: libc::c_float = *fresh43;
        let fresh44 = src;
        src = src.offset(1);
        let mut in_b: libc::c_float = *fresh44;
        let mut out_r: libc::c_float =
            mat.m[0 as libc::c_int as usize][0 as libc::c_int as usize] * in_r
                +
                mat.m[1 as libc::c_int as usize][0 as libc::c_int as usize] *
                    in_g +
                mat.m[2 as libc::c_int as usize][0 as libc::c_int as usize] *
                    in_b;
        let mut out_g: libc::c_float =
            mat.m[0 as libc::c_int as usize][1 as libc::c_int as usize] * in_r
                +
                mat.m[1 as libc::c_int as usize][1 as libc::c_int as usize] *
                    in_g +
                mat.m[2 as libc::c_int as usize][1 as libc::c_int as usize] *
                    in_b;
        let mut out_b: libc::c_float =
            mat.m[0 as libc::c_int as usize][2 as libc::c_int as usize] * in_r
                +
                mat.m[1 as libc::c_int as usize][2 as libc::c_int as usize] *
                    in_g +
                mat.m[2 as libc::c_int as usize][2 as libc::c_int as usize] *
                    in_b;
        let fresh45 = dest;
        dest = dest.offset(1);
        *fresh45 = clamp_float(out_r);
        let fresh46 = dest;
        dest = dest.offset(1);
        *fresh46 = clamp_float(out_g);
        let fresh47 = dest;
        dest = dest.offset(1);
        *fresh47 = clamp_float(out_b);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn qcms_modular_transform_alloc()
 -> *mut qcms_modular_transform {
    return calloc(1 as libc::c_int as libc::c_ulong,
                  ::std::mem::size_of::<qcms_modular_transform>() as
                      libc::c_ulong) as *mut qcms_modular_transform;
}
unsafe extern "C" fn qcms_modular_transform_release(mut transform:
                                                        *mut qcms_modular_transform) {
    let mut next_transform: *mut qcms_modular_transform =
        0 as *mut qcms_modular_transform;
    while !transform.is_null() {
        next_transform = (*transform).next_transform;
        // clut may use a single block of memory.
		// Perhaps we should remove this to simply the code.
        if (*transform).input_clut_table_r.offset((*transform).input_clut_table_length
                                                      as libc::c_int as isize)
               == (*transform).input_clut_table_g &&
               (*transform).input_clut_table_g.offset((*transform).input_clut_table_length
                                                          as libc::c_int as
                                                          isize) ==
                   (*transform).input_clut_table_b {
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
        if (*transform).r_clut.offset(1 as libc::c_int as isize) ==
               (*transform).g_clut &&
               (*transform).g_clut.offset(1 as libc::c_int as isize) ==
                   (*transform).b_clut {
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
        if (*transform).output_clut_table_r.offset((*transform).output_clut_table_length
                                                       as libc::c_int as
                                                       isize) ==
               (*transform).output_clut_table_g &&
               (*transform).output_clut_table_g.offset((*transform).output_clut_table_length
                                                           as libc::c_int as
                                                           isize) ==
                   (*transform).output_clut_table_b {
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
    };
}
/* Set transform to be the next element in the linked list. */
unsafe extern "C" fn append_transform(mut transform:
                                          *mut qcms_modular_transform,
                                      mut next_transform:
                                          *mut *mut *mut qcms_modular_transform) {
    **next_transform = transform;
    while !transform.is_null() {
        *next_transform = &mut (*transform).next_transform;
        transform = (*transform).next_transform
    };
}
/* reverse the transformation list (used by mBA) */
unsafe extern "C" fn reverse_transform(mut transform:
                                           *mut qcms_modular_transform)
 -> *mut qcms_modular_transform {
    let mut prev_transform: *mut qcms_modular_transform =
        0 as *mut qcms_modular_transform;
    while !transform.is_null() {
        let mut next_transform: *mut qcms_modular_transform =
            (*transform).next_transform;
        (*transform).next_transform = prev_transform;
        prev_transform = transform;
        transform = next_transform
    }
    return prev_transform;
}
unsafe extern "C" fn qcms_modular_transform_create_mAB(mut lut:
                                                           *mut lutmABType)
 -> *mut qcms_modular_transform {
    let mut current_block: u64;
    let mut first_transform: *mut qcms_modular_transform =
        0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform =
        &mut first_transform;
    let mut transform: *mut qcms_modular_transform =
        0 as *mut qcms_modular_transform;
    if !(*lut).a_curves[0 as libc::c_int as usize].is_null() {
        let mut clut_length: size_t = 0;
        let mut clut: *mut libc::c_float = 0 as *mut libc::c_float;
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
                (*transform).input_clut_table_r =
                    build_input_gamma_table((*lut).a_curves[0 as libc::c_int
                                                                as usize]);
                (*transform).input_clut_table_g =
                    build_input_gamma_table((*lut).a_curves[1 as libc::c_int
                                                                as usize]);
                (*transform).input_clut_table_b =
                    build_input_gamma_table((*lut).a_curves[2 as libc::c_int
                                                                as usize]);
                (*transform).transform_module_fn =
                    Some(qcms_transform_module_gamma_table as
                             unsafe extern "C" fn(_:
                                                      *mut qcms_modular_transform,
                                                  _: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: size_t) -> ());
                if (*lut).num_grid_points[0 as libc::c_int as usize] as
                       libc::c_int !=
                       (*lut).num_grid_points[1 as libc::c_int as usize] as
                           libc::c_int ||
                       (*lut).num_grid_points[1 as libc::c_int as usize] as
                           libc::c_int !=
                           (*lut).num_grid_points[2 as libc::c_int as usize]
                               as libc::c_int {
                    //XXX: We don't currently support clut that are not squared!
                    current_block = 7590209878260659629;
                } else {
                    // Prepare CLUT
                    transform = qcms_modular_transform_alloc();
                    if transform.is_null() {
                        current_block = 7590209878260659629;
                    } else {
                        append_transform(transform, &mut next_transform);
                        clut_length =
                            (::std::mem::size_of::<libc::c_float>() as
                                 libc::c_ulong as libc::c_double *
                                 pow((*lut).num_grid_points[0 as libc::c_int
                                                                as usize] as
                                         libc::c_double,
                                     3 as libc::c_int as libc::c_double) *
                                 3 as libc::c_int as libc::c_double) as
                                size_t;
                        clut = malloc(clut_length) as *mut libc::c_float;
                        if clut.is_null() {
                            current_block = 7590209878260659629;
                        } else {
                            memcpy(clut as *mut libc::c_void,
                                   (*lut).clut_table as *const libc::c_void,
                                   clut_length);
                            (*transform).r_clut =
                                clut.offset(0 as libc::c_int as isize);
                            (*transform).g_clut =
                                clut.offset(1 as libc::c_int as isize);
                            (*transform).b_clut =
                                clut.offset(2 as libc::c_int as isize);
                            (*transform).grid_size =
                                (*lut).num_grid_points[0 as libc::c_int as
                                                           usize] as uint16_t;
                            (*transform).transform_module_fn =
                                Some(qcms_transform_module_clut_only as
                                         unsafe extern "C" fn(_:
                                                                  *mut qcms_modular_transform,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _: size_t)
                                             -> ());
                            current_block = 10652014663920648156;
                        }
                    }
                }
            }
        }
    } else { current_block = 10652014663920648156; }
    match current_block {
        10652014663920648156 => {
            if !(*lut).m_curves[0 as libc::c_int as usize].is_null() {
                // M curve imples the presence of a Matrix
                // Prepare M curve
                transform = qcms_modular_transform_alloc();
                if transform.is_null() {
                    current_block = 7590209878260659629;
                } else {
                    append_transform(transform, &mut next_transform);
                    (*transform).input_clut_table_r =
                        build_input_gamma_table((*lut).m_curves[0 as
                                                                    libc::c_int
                                                                    as
                                                                    usize]);
                    (*transform).input_clut_table_g =
                        build_input_gamma_table((*lut).m_curves[1 as
                                                                    libc::c_int
                                                                    as
                                                                    usize]);
                    (*transform).input_clut_table_b =
                        build_input_gamma_table((*lut).m_curves[2 as
                                                                    libc::c_int
                                                                    as
                                                                    usize]);
                    (*transform).transform_module_fn =
                        Some(qcms_transform_module_gamma_table as
                                 unsafe extern "C" fn(_:
                                                          *mut qcms_modular_transform,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: size_t) -> ());
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
                            (*transform).tx =
                                s15Fixed16Number_to_float((*lut).e03);
                            (*transform).ty =
                                s15Fixed16Number_to_float((*lut).e13);
                            (*transform).tz =
                                s15Fixed16Number_to_float((*lut).e23);
                            (*transform).transform_module_fn =
                                Some(qcms_transform_module_matrix_translate as
                                         unsafe extern "C" fn(_:
                                                                  *mut qcms_modular_transform,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _: size_t)
                                             -> ());
                            current_block = 980989089337379490;
                        }
                    }
                }
            } else { current_block = 980989089337379490; }
            match current_block {
                7590209878260659629 => { }
                _ => {
                    if !(*lut).b_curves[0 as libc::c_int as usize].is_null() {
                        // Prepare B curve
                        transform = qcms_modular_transform_alloc();
                        if !transform.is_null() {
                            append_transform(transform, &mut next_transform);
                            (*transform).input_clut_table_r =
                                build_input_gamma_table((*lut).b_curves[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]);
                            (*transform).input_clut_table_g =
                                build_input_gamma_table((*lut).b_curves[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]);
                            (*transform).input_clut_table_b =
                                build_input_gamma_table((*lut).b_curves[2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]);
                            (*transform).transform_module_fn =
                                Some(qcms_transform_module_gamma_table as
                                         unsafe extern "C" fn(_:
                                                                  *mut qcms_modular_transform,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _: size_t)
                                             -> ());
                            if (*lut).reversed {
                                // mBA are identical to mAB except that the transformation order
		// is reversed
                                first_transform =
                                    reverse_transform(first_transform)
                            }
                            return first_transform
                        }
                    }
                }
            }
        }
        _ => { }
    }
    // B curve is mandatory
    qcms_modular_transform_release(first_transform);
    return 0 as *mut qcms_modular_transform;
}
unsafe extern "C" fn qcms_modular_transform_create_lut(mut lut: *mut lutType)
 -> *mut qcms_modular_transform {
    let mut first_transform: *mut qcms_modular_transform =
        0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform =
        &mut first_transform;
    let mut transform: *mut qcms_modular_transform =
        0 as *mut qcms_modular_transform;
    let mut in_curve_len: size_t = 0;
    let mut clut_length: size_t = 0;
    let mut out_curve_len: size_t = 0;
    let mut in_curves: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut clut: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut out_curves: *mut libc::c_float = 0 as *mut libc::c_float;
    // Prepare Matrix
    transform = qcms_modular_transform_alloc();
    if !transform.is_null() {
        append_transform(transform, &mut next_transform);
        (*transform).matrix = build_lut_matrix(lut);
        if !(*transform).matrix.invalid {
            (*transform).transform_module_fn =
                Some(qcms_transform_module_matrix as
                         unsafe extern "C" fn(_: *mut qcms_modular_transform,
                                              _: *mut libc::c_float,
                                              _: *mut libc::c_float,
                                              _: size_t) -> ());
            // Prepare input curves
            transform = qcms_modular_transform_alloc();
            if !transform.is_null() {
                append_transform(transform, &mut next_transform);
                in_curve_len =
                    (::std::mem::size_of::<libc::c_float>() as
                         libc::c_ulong).wrapping_mul((*lut).num_input_table_entries
                                                         as
                                                         libc::c_ulong).wrapping_mul(3
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong);
                in_curves = malloc(in_curve_len) as *mut libc::c_float;
                if !in_curves.is_null() {
                    memcpy(in_curves as *mut libc::c_void,
                           (*lut).input_table as *const libc::c_void,
                           in_curve_len);
                    (*transform).input_clut_table_r =
                        in_curves.offset(((*lut).num_input_table_entries as
                                              libc::c_int * 0 as libc::c_int)
                                             as isize);
                    (*transform).input_clut_table_g =
                        in_curves.offset(((*lut).num_input_table_entries as
                                              libc::c_int * 1 as libc::c_int)
                                             as isize);
                    (*transform).input_clut_table_b =
                        in_curves.offset(((*lut).num_input_table_entries as
                                              libc::c_int * 2 as libc::c_int)
                                             as isize);
                    (*transform).input_clut_table_length =
                        (*lut).num_input_table_entries;
                    // Prepare table
                    clut_length =
                        (::std::mem::size_of::<libc::c_float>() as
                             libc::c_ulong as libc::c_double *
                             pow((*lut).num_clut_grid_points as
                                     libc::c_double,
                                 3 as libc::c_int as libc::c_double) *
                             3 as libc::c_int as libc::c_double) as size_t;
                    clut = malloc(clut_length) as *mut libc::c_float;
                    if !clut.is_null() {
                        memcpy(clut as *mut libc::c_void,
                               (*lut).clut_table as *const libc::c_void,
                               clut_length);
                        (*transform).r_clut =
                            clut.offset(0 as libc::c_int as isize);
                        (*transform).g_clut =
                            clut.offset(1 as libc::c_int as isize);
                        (*transform).b_clut =
                            clut.offset(2 as libc::c_int as isize);
                        (*transform).grid_size =
                            (*lut).num_clut_grid_points as uint16_t;
                        // Prepare output curves
                        out_curve_len =
                            (::std::mem::size_of::<libc::c_float>() as
                                 libc::c_ulong).wrapping_mul((*lut).num_output_table_entries
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(3
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong);
                        out_curves =
                            malloc(out_curve_len) as *mut libc::c_float;
                        if !out_curves.is_null() {
                            memcpy(out_curves as *mut libc::c_void,
                                   (*lut).output_table as *const libc::c_void,
                                   out_curve_len);
                            (*transform).output_clut_table_r =
                                out_curves.offset(((*lut).num_output_table_entries
                                                       as libc::c_int *
                                                       0 as libc::c_int) as
                                                      isize);
                            (*transform).output_clut_table_g =
                                out_curves.offset(((*lut).num_output_table_entries
                                                       as libc::c_int *
                                                       1 as libc::c_int) as
                                                      isize);
                            (*transform).output_clut_table_b =
                                out_curves.offset(((*lut).num_output_table_entries
                                                       as libc::c_int *
                                                       2 as libc::c_int) as
                                                      isize);
                            (*transform).output_clut_table_length =
                                (*lut).num_output_table_entries;
                            (*transform).transform_module_fn =
                                Some(qcms_transform_module_clut as
                                         unsafe extern "C" fn(_:
                                                                  *mut qcms_modular_transform,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _: size_t)
                                             -> ());
                            return first_transform
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
pub unsafe extern "C" fn qcms_modular_transform_create_input(mut in_0:
                                                                 *mut qcms_profile)
 -> *mut qcms_modular_transform {
    let mut current_block: u64;
    let mut first_transform: *mut qcms_modular_transform =
        0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform =
        &mut first_transform;
    if !(*in_0).A2B0.is_null() {
        let mut lut_transform: *mut qcms_modular_transform =
            0 as *mut qcms_modular_transform;
        lut_transform = qcms_modular_transform_create_lut((*in_0).A2B0);
        if lut_transform.is_null() {
            current_block = 8903102000210989603;
        } else {
            append_transform(lut_transform, &mut next_transform);
            current_block = 10692455896603418738;
        }
    } else if !(*in_0).mAB.is_null() &&
                  (*(*in_0).mAB).num_in_channels as libc::c_int ==
                      3 as libc::c_int &&
                  (*(*in_0).mAB).num_out_channels as libc::c_int ==
                      3 as libc::c_int {
        let mut mAB_transform: *mut qcms_modular_transform =
            0 as *mut qcms_modular_transform;
        mAB_transform = qcms_modular_transform_create_mAB((*in_0).mAB);
        if mAB_transform.is_null() {
            current_block = 8903102000210989603;
        } else {
            append_transform(mAB_transform, &mut next_transform);
            current_block = 10692455896603418738;
        }
    } else {
        let mut transform: *mut qcms_modular_transform =
            0 as *mut qcms_modular_transform;
        transform = qcms_modular_transform_alloc();
        if transform.is_null() {
            current_block = 8903102000210989603;
        } else {
            append_transform(transform, &mut next_transform);
            (*transform).input_clut_table_r =
                build_input_gamma_table((*in_0).redTRC);
            (*transform).input_clut_table_g =
                build_input_gamma_table((*in_0).greenTRC);
            (*transform).input_clut_table_b =
                build_input_gamma_table((*in_0).blueTRC);
            (*transform).transform_module_fn =
                Some(qcms_transform_module_gamma_table as
                         unsafe extern "C" fn(_: *mut qcms_modular_transform,
                                              _: *mut libc::c_float,
                                              _: *mut libc::c_float,
                                              _: size_t) -> ());
            if (*transform).input_clut_table_r.is_null() ||
                   (*transform).input_clut_table_g.is_null() ||
                   (*transform).input_clut_table_b.is_null() {
                current_block = 8903102000210989603;
            } else {
                transform = qcms_modular_transform_alloc();
                if transform.is_null() {
                    current_block = 8903102000210989603;
                } else {
                    append_transform(transform, &mut next_transform);
                    (*transform).matrix.m[0 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] =
                        1 as libc::c_int as libc::c_float /
                            1.999969482421875f32;
                    (*transform).matrix.m[0 as libc::c_int as
                                              usize][1 as libc::c_int as
                                                         usize] = 0.0f32;
                    (*transform).matrix.m[0 as libc::c_int as
                                              usize][2 as libc::c_int as
                                                         usize] = 0.0f32;
                    (*transform).matrix.m[1 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] = 0.0f32;
                    (*transform).matrix.m[1 as libc::c_int as
                                              usize][1 as libc::c_int as
                                                         usize] =
                        1 as libc::c_int as libc::c_float /
                            1.999969482421875f32;
                    (*transform).matrix.m[1 as libc::c_int as
                                              usize][2 as libc::c_int as
                                                         usize] = 0.0f32;
                    (*transform).matrix.m[2 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] = 0.0f32;
                    (*transform).matrix.m[2 as libc::c_int as
                                              usize][1 as libc::c_int as
                                                         usize] = 0.0f32;
                    (*transform).matrix.m[2 as libc::c_int as
                                              usize][2 as libc::c_int as
                                                         usize] =
                        1 as libc::c_int as libc::c_float /
                            1.999969482421875f32;
                    (*transform).matrix.invalid = 0 as libc::c_int != 0;
                    (*transform).transform_module_fn =
                        Some(qcms_transform_module_matrix as
                                 unsafe extern "C" fn(_:
                                                          *mut qcms_modular_transform,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: size_t) -> ());
                    transform = qcms_modular_transform_alloc();
                    if transform.is_null() {
                        current_block = 8903102000210989603;
                    } else {
                        append_transform(transform, &mut next_transform);
                        (*transform).matrix = build_colorant_matrix(in_0);
                        (*transform).transform_module_fn =
                            Some(qcms_transform_module_matrix as
                                     unsafe extern "C" fn(_:
                                                              *mut qcms_modular_transform,
                                                          _:
                                                              *mut libc::c_float,
                                                          _:
                                                              *mut libc::c_float,
                                                          _: size_t) -> ());
                        current_block = 10692455896603418738;
                    }
                }
            }
        }
    }
    match current_block {
        10692455896603418738 => { return first_transform }
        _ => {
            qcms_modular_transform_release(first_transform);
            return 0 as *mut qcms_modular_transform
        }
    };
}
unsafe extern "C" fn qcms_modular_transform_create_output(mut out:
                                                              *mut qcms_profile)
 -> *mut qcms_modular_transform {
    let mut current_block: u64;
    let mut first_transform: *mut qcms_modular_transform =
        0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform =
        &mut first_transform;
    if !(*out).B2A0.is_null() {
        let mut lut_transform: *mut qcms_modular_transform =
            0 as *mut qcms_modular_transform;
        lut_transform = qcms_modular_transform_create_lut((*out).B2A0);
        if lut_transform.is_null() {
            current_block = 15713701561912628542;
        } else {
            append_transform(lut_transform, &mut next_transform);
            current_block = 13131896068329595644;
        }
    } else if !(*out).mBA.is_null() &&
                  (*(*out).mBA).num_in_channels as libc::c_int ==
                      3 as libc::c_int &&
                  (*(*out).mBA).num_out_channels as libc::c_int ==
                      3 as libc::c_int {
        let mut lut_transform_0: *mut qcms_modular_transform =
            0 as *mut qcms_modular_transform;
        lut_transform_0 = qcms_modular_transform_create_mAB((*out).mBA);
        if lut_transform_0.is_null() {
            current_block = 15713701561912628542;
        } else {
            append_transform(lut_transform_0, &mut next_transform);
            current_block = 13131896068329595644;
        }
    } else if !(*out).redTRC.is_null() && !(*out).greenTRC.is_null() &&
                  !(*out).blueTRC.is_null() {
        let mut transform: *mut qcms_modular_transform =
            0 as *mut qcms_modular_transform;
        transform = qcms_modular_transform_alloc();
        if transform.is_null() {
            current_block = 15713701561912628542;
        } else {
            append_transform(transform, &mut next_transform);
            (*transform).matrix = matrix_invert(build_colorant_matrix(out));
            (*transform).transform_module_fn =
                Some(qcms_transform_module_matrix as
                         unsafe extern "C" fn(_: *mut qcms_modular_transform,
                                              _: *mut libc::c_float,
                                              _: *mut libc::c_float,
                                              _: size_t) -> ());
            transform = qcms_modular_transform_alloc();
            if transform.is_null() {
                current_block = 15713701561912628542;
            } else {
                append_transform(transform, &mut next_transform);
                (*transform).matrix.m[0 as libc::c_int as
                                          usize][0 as libc::c_int as usize] =
                    1.999969482421875f32;
                (*transform).matrix.m[0 as libc::c_int as
                                          usize][1 as libc::c_int as usize] =
                    0.0f32;
                (*transform).matrix.m[0 as libc::c_int as
                                          usize][2 as libc::c_int as usize] =
                    0.0f32;
                (*transform).matrix.m[1 as libc::c_int as
                                          usize][0 as libc::c_int as usize] =
                    0.0f32;
                (*transform).matrix.m[1 as libc::c_int as
                                          usize][1 as libc::c_int as usize] =
                    1.999969482421875f32;
                (*transform).matrix.m[1 as libc::c_int as
                                          usize][2 as libc::c_int as usize] =
                    0.0f32;
                (*transform).matrix.m[2 as libc::c_int as
                                          usize][0 as libc::c_int as usize] =
                    0.0f32;
                (*transform).matrix.m[2 as libc::c_int as
                                          usize][1 as libc::c_int as usize] =
                    0.0f32;
                (*transform).matrix.m[2 as libc::c_int as
                                          usize][2 as libc::c_int as usize] =
                    1.999969482421875f32;
                (*transform).matrix.invalid = 0 as libc::c_int != 0;
                (*transform).transform_module_fn =
                    Some(qcms_transform_module_matrix as
                             unsafe extern "C" fn(_:
                                                      *mut qcms_modular_transform,
                                                  _: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: size_t) -> ());
                transform = qcms_modular_transform_alloc();
                if transform.is_null() {
                    current_block = 15713701561912628542;
                } else {
                    append_transform(transform, &mut next_transform);
                    build_output_lut((*out).redTRC,
                                     &mut (*transform).output_gamma_lut_r,
                                     &mut (*transform).output_gamma_lut_r_length);
                    build_output_lut((*out).greenTRC,
                                     &mut (*transform).output_gamma_lut_g,
                                     &mut (*transform).output_gamma_lut_g_length);
                    build_output_lut((*out).blueTRC,
                                     &mut (*transform).output_gamma_lut_b,
                                     &mut (*transform).output_gamma_lut_b_length);
                    (*transform).transform_module_fn =
                        Some(qcms_transform_module_gamma_lut as
                                 unsafe extern "C" fn(_:
                                                          *mut qcms_modular_transform,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: size_t) -> ());
                    if (*transform).output_gamma_lut_r.is_null() ||
                           (*transform).output_gamma_lut_g.is_null() ||
                           (*transform).output_gamma_lut_b.is_null() {
                        current_block = 15713701561912628542;
                    } else { current_block = 13131896068329595644; }
                }
            }
        }
    } else {
        if !(0 as libc::c_int != 0 &&
                 !(b"Unsupported output profile workflow.\x00" as *const u8 as
                       *const libc::c_char).is_null()) as libc::c_int as
               libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 37],
                                                   &[libc::c_char; 37]>(b"qcms_modular_transform_create_output\x00")).as_ptr(),
                         b"chain.c\x00" as *const u8 as *const libc::c_char,
                         833 as libc::c_int,
                         b"0 && \"Unsupported output profile workflow.\"\x00"
                             as *const u8 as *const libc::c_char);
        } else { };
        return 0 as *mut qcms_modular_transform
    }
    match current_block {
        13131896068329595644 => { return first_transform }
        _ => {
            qcms_modular_transform_release(first_transform);
            return 0 as *mut qcms_modular_transform
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
unsafe extern "C" fn qcms_modular_transform_create(mut in_0:
                                                       *mut qcms_profile,
                                                   mut out: *mut qcms_profile)
 -> *mut qcms_modular_transform {
    let mut current_block: u64;
    let mut first_transform: *mut qcms_modular_transform =
        0 as *mut qcms_modular_transform;
    let mut next_transform: *mut *mut qcms_modular_transform =
        &mut first_transform;
    if (*in_0).color_space == 0x52474220 as libc::c_int as libc::c_uint {
        let mut rgb_to_pcs: *mut qcms_modular_transform =
            0 as *mut qcms_modular_transform;
        rgb_to_pcs = qcms_modular_transform_create_input(in_0);
        if !rgb_to_pcs.is_null() {
            append_transform(rgb_to_pcs, &mut next_transform);
            if (*in_0).pcs == 0x4c616220 as libc::c_int as libc::c_uint &&
                   (*out).pcs == 0x58595a20 as libc::c_int as libc::c_uint {
                let mut lab_to_pcs: *mut qcms_modular_transform =
                    0 as *mut qcms_modular_transform;
                lab_to_pcs = qcms_modular_transform_alloc();
                if lab_to_pcs.is_null() {
                    current_block = 8418824557173580938;
                } else {
                    append_transform(lab_to_pcs, &mut next_transform);
                    (*lab_to_pcs).transform_module_fn =
                        Some(qcms_transform_module_LAB_to_XYZ as
                                 unsafe extern "C" fn(_:
                                                          *mut qcms_modular_transform,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: size_t) -> ());
                    current_block = 10599921512955367680;
                }
            } else { current_block = 10599921512955367680; }
            match current_block {
                8418824557173580938 => { }
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
                    if (*in_0).pcs ==
                           0x58595a20 as libc::c_int as libc::c_uint &&
                           (*out).pcs ==
                               0x4c616220 as libc::c_int as libc::c_uint {
                        let mut pcs_to_lab: *mut qcms_modular_transform =
                            0 as *mut qcms_modular_transform;
                        pcs_to_lab = qcms_modular_transform_alloc();
                        if pcs_to_lab.is_null() {
                            current_block = 8418824557173580938;
                        } else {
                            append_transform(pcs_to_lab, &mut next_transform);
                            (*pcs_to_lab).transform_module_fn =
                                Some(qcms_transform_module_XYZ_to_LAB as
                                         unsafe extern "C" fn(_:
                                                                  *mut qcms_modular_transform,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _: size_t)
                                             -> ());
                            current_block = 7175849428784450219;
                        }
                    } else { current_block = 7175849428784450219; }
                    match current_block {
                        8418824557173580938 => { }
                        _ => {
                            if (*out).color_space ==
                                   0x52474220 as libc::c_int as libc::c_uint {
                                let mut pcs_to_rgb:
                                        *mut qcms_modular_transform =
                                    0 as *mut qcms_modular_transform;
                                pcs_to_rgb =
                                    qcms_modular_transform_create_output(out);
                                if !pcs_to_rgb.is_null() {
                                    append_transform(pcs_to_rgb,
                                                     &mut next_transform);
                                    // Not Completed
	//return qcms_modular_transform_reduce(first_transform);
                                    return first_transform
                                }
                            } else {
                                if !(0 as libc::c_int != 0 &&
                                         !(b"output color space not supported\x00"
                                               as *const u8 as
                                               *const libc::c_char).is_null())
                                       as libc::c_int as libc::c_long != 0 {
                                    __assert_rtn((*::std::mem::transmute::<&[u8; 30],
                                                                           &[libc::c_char; 30]>(b"qcms_modular_transform_create\x00")).as_ptr(),
                                                 b"chain.c\x00" as *const u8
                                                     as *const libc::c_char,
                                                 947 as libc::c_int,
                                                 b"0 && \"output color space not supported\"\x00"
                                                     as *const u8 as
                                                     *const libc::c_char);
                                } else { };
                            }
                        }
                    }
                }
            }
        }
    } else {
        if !(0 as libc::c_int != 0 &&
                 !(b"input color space not supported\x00" as *const u8 as
                       *const libc::c_char).is_null()) as libc::c_int as
               libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 30],
                                                   &[libc::c_char; 30]>(b"qcms_modular_transform_create\x00")).as_ptr(),
                         b"chain.c\x00" as *const u8 as *const libc::c_char,
                         907 as libc::c_int,
                         b"0 && \"input color space not supported\"\x00" as
                             *const u8 as *const libc::c_char);
        } else { };
    }
    qcms_modular_transform_release(first_transform);
    return 0 as *mut qcms_modular_transform;
}
unsafe extern "C" fn qcms_modular_transform_data(mut transform:
                                                     *mut qcms_modular_transform,
                                                 mut src: *mut libc::c_float,
                                                 mut dest: *mut libc::c_float,
                                                 mut len: size_t)
 -> *mut libc::c_float {
    while !transform.is_null() {
        // Keep swaping src/dest when performing a transform to use less memory.
        let mut new_src: *mut libc::c_float = dest;
        let transform_fn: transform_module_fn_t =
            (*transform).transform_module_fn;
        if transform_fn !=
               Some(qcms_transform_module_gamma_table as
                        unsafe extern "C" fn(_: *mut qcms_modular_transform,
                                             _: *mut libc::c_float,
                                             _: *mut libc::c_float, _: size_t)
                            -> ()) &&
               transform_fn !=
                   Some(qcms_transform_module_gamma_lut as
                            unsafe extern "C" fn(_:
                                                     *mut qcms_modular_transform,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: size_t) -> ()) &&
               transform_fn !=
                   Some(qcms_transform_module_clut as
                            unsafe extern "C" fn(_:
                                                     *mut qcms_modular_transform,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: size_t) -> ()) &&
               transform_fn !=
                   Some(qcms_transform_module_clut_only as
                            unsafe extern "C" fn(_:
                                                     *mut qcms_modular_transform,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: size_t) -> ()) &&
               transform_fn !=
                   Some(qcms_transform_module_matrix as
                            unsafe extern "C" fn(_:
                                                     *mut qcms_modular_transform,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: size_t) -> ()) &&
               transform_fn !=
                   Some(qcms_transform_module_matrix_translate as
                            unsafe extern "C" fn(_:
                                                     *mut qcms_modular_transform,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: size_t) -> ()) &&
               transform_fn !=
                   Some(qcms_transform_module_LAB_to_XYZ as
                            unsafe extern "C" fn(_:
                                                     *mut qcms_modular_transform,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: size_t) -> ()) &&
               transform_fn !=
                   Some(qcms_transform_module_XYZ_to_LAB as
                            unsafe extern "C" fn(_:
                                                     *mut qcms_modular_transform,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: size_t) -> ()) {
            if !(0 as libc::c_int != 0 &&
                     !(b"Unsupported transform module\x00" as *const u8 as
                           *const libc::c_char).is_null()) as libc::c_int as
                   libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 28],
                                                       &[libc::c_char; 28]>(b"qcms_modular_transform_data\x00")).as_ptr(),
                             b"chain.c\x00" as *const u8 as
                                 *const libc::c_char, 972 as libc::c_int,
                             b"0 && \"Unsupported transform module\"\x00" as
                                 *const u8 as *const libc::c_char);
            } else { };
            return 0 as *mut libc::c_float
        }
        if (*transform).grid_size as libc::c_int <= 0 as libc::c_int &&
               (transform_fn ==
                    Some(qcms_transform_module_clut as
                             unsafe extern "C" fn(_:
                                                      *mut qcms_modular_transform,
                                                  _: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: size_t) -> ()) ||
                    transform_fn ==
                        Some(qcms_transform_module_clut_only as
                                 unsafe extern "C" fn(_:
                                                          *mut qcms_modular_transform,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: size_t) -> ())) {
            if !(0 as libc::c_int != 0 &&
                     !(b"Invalid transform\x00" as *const u8 as
                           *const libc::c_char).is_null()) as libc::c_int as
                   libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 28],
                                                       &[libc::c_char; 28]>(b"qcms_modular_transform_data\x00")).as_ptr(),
                             b"chain.c\x00" as *const u8 as
                                 *const libc::c_char, 978 as libc::c_int,
                             b"0 && \"Invalid transform\"\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            return 0 as *mut libc::c_float
        }
        (*transform).transform_module_fn.expect("non-null function pointer")(transform,
                                                                             src,
                                                                             dest,
                                                                             len);
        dest = src;
        src = new_src;
        transform = (*transform).next_transform
    }
    // The results end up in the src buffer because of the switching
    return src;
}
#[no_mangle]
pub unsafe extern "C" fn qcms_chain_transform(mut in_0: *mut qcms_profile,
                                              mut out: *mut qcms_profile,
                                              mut src: *mut libc::c_float,
                                              mut dest: *mut libc::c_float,
                                              mut lutSize: size_t)
 -> *mut libc::c_float {
    let mut transform_list: *mut qcms_modular_transform =
        qcms_modular_transform_create(in_0, out);
    if !transform_list.is_null() {
        let mut lut: *mut libc::c_float =
            qcms_modular_transform_data(transform_list, src, dest,
                                        lutSize.wrapping_div(3 as libc::c_int
                                                                 as
                                                                 libc::c_ulong));
        qcms_modular_transform_release(transform_list);
        return lut
    }
    return 0 as *mut libc::c_float;
}
