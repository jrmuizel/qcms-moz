use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn floorf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn qcms_transform_data_rgb_out_lut_avx(transform: *const qcms_transform,
                                           src: *const libc::c_uchar,
                                           dest: *mut libc::c_uchar,
                                           length: size_t);
    #[no_mangle]
    fn qcms_transform_data_rgba_out_lut_avx(transform: *const qcms_transform,
                                            src: *const libc::c_uchar,
                                            dest: *mut libc::c_uchar,
                                            length: size_t);
    #[no_mangle]
    fn qcms_transform_data_bgra_out_lut_avx(transform: *const qcms_transform,
                                            src: *const libc::c_uchar,
                                            dest: *mut libc::c_uchar,
                                            length: size_t);
    #[no_mangle]
    fn qcms_transform_data_rgb_out_lut_sse2(transform: *const qcms_transform,
                                            src: *const libc::c_uchar,
                                            dest: *mut libc::c_uchar,
                                            length: size_t);
    #[no_mangle]
    fn qcms_transform_data_rgba_out_lut_sse2(transform: *const qcms_transform,
                                             src: *const libc::c_uchar,
                                             dest: *mut libc::c_uchar,
                                             length: size_t);
    #[no_mangle]
    fn qcms_transform_data_bgra_out_lut_sse2(transform: *const qcms_transform,
                                             src: *const libc::c_uchar,
                                             dest: *mut libc::c_uchar,
                                             length: size_t);
    #[no_mangle]
    fn qcms_transform_data_rgb_out_lut_sse1(transform: *const qcms_transform,
                                            src: *const libc::c_uchar,
                                            dest: *mut libc::c_uchar,
                                            length: size_t);
    #[no_mangle]
    fn qcms_transform_data_rgba_out_lut_sse1(transform: *const qcms_transform,
                                             src: *const libc::c_uchar,
                                             dest: *mut libc::c_uchar,
                                             length: size_t);
    #[no_mangle]
    fn qcms_transform_data_bgra_out_lut_sse1(transform: *const qcms_transform,
                                             src: *const libc::c_uchar,
                                             dest: *mut libc::c_uchar,
                                             length: size_t);
    /* vim: set ts=8 sw=8 noexpandtab: */
//  qcms
//  Copyright (C) 2009 Mozilla Foundation
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
    // Generates and returns a 3D LUT with lutSize^3 samples using the provided src/dest.
    #[no_mangle]
    fn qcms_chain_transform(in_0: *mut qcms_profile, out: *mut qcms_profile,
                            src: *mut libc::c_float, dest: *mut libc::c_float,
                            lutSize: size_t) -> *mut libc::c_float;
    #[no_mangle]
    fn matrix_eval(mat: matrix, v: vector) -> vector;
    #[no_mangle]
    fn matrix_multiply(a: matrix, b: matrix) -> matrix;
    #[no_mangle]
    fn matrix_invert(mat: matrix) -> matrix;
    #[no_mangle]
    fn matrix_invalid() -> matrix;
    #[no_mangle]
    fn lut_interp_linear(value: libc::c_double, table: *mut uint16_t,
                         length: libc::c_int) -> libc::c_float;
    #[no_mangle]
    fn build_input_gamma_table(TRC: *mut curveType) -> *mut libc::c_float;
    #[no_mangle]
    fn build_colorant_matrix(p: *mut qcms_profile) -> matrix;
    #[no_mangle]
    fn build_output_lut(trc: *mut curveType,
                        output_gamma_lut: *mut *mut uint16_t,
                        output_gamma_lut_length: *mut size_t);
    #[no_mangle]
    fn compute_precache(trc: *mut curveType, output: *mut uint8_t) -> bool;
}
pub type __darwin_size_t = libc::c_ulong;
pub type int32_t = libc::c_int;
pub type uintptr_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
/* vim: set ts=8 sw=8 noexpandtab: */
/* used as a lookup table for the output transformation.
 * we refcount them so we only need to have one around per output
 * profile, instead of duplicating them per transform */
/* We previously used a count of 65536 here but that seems like more
	 * precision than we actually need.  By reducing the size we can
	 * improve startup performance and reduce memory usage. ColorSync on
	 * 10.5 uses 4097 which is perhaps because they use a fixed point
	 * representation where 1. is represented by 0x1000. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _qcms_transform {
    pub matrix: [[libc::c_float; 4]; 3],
    pub input_gamma_table_r: *mut libc::c_float,
    pub input_gamma_table_g: *mut libc::c_float,
    pub input_gamma_table_b: *mut libc::c_float,
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
    pub input_gamma_table_gray: *mut libc::c_float,
    pub out_gamma_r: libc::c_float,
    pub out_gamma_g: libc::c_float,
    pub out_gamma_b: libc::c_float,
    pub out_gamma_gray: libc::c_float,
    pub output_gamma_lut_r: *mut uint16_t,
    pub output_gamma_lut_g: *mut uint16_t,
    pub output_gamma_lut_b: *mut uint16_t,
    pub output_gamma_lut_gray: *mut uint16_t,
    pub output_gamma_lut_r_length: size_t,
    pub output_gamma_lut_g_length: size_t,
    pub output_gamma_lut_b_length: size_t,
    pub output_gamma_lut_gray_length: size_t,
    pub output_table_r: *mut precache_output,
    pub output_table_g: *mut precache_output,
    pub output_table_b: *mut precache_output,
    pub transform_fn: transform_fn_t,
}
pub type transform_fn_t
    =
    Option<unsafe extern "C" fn(_: *const _qcms_transform,
                                _: *const libc::c_uchar,
                                _: *mut libc::c_uchar, _: size_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct precache_output {
    pub ref_count: libc::c_int,
    pub data: [uint8_t; 8192],
}
pub type qcms_transform = _qcms_transform;
// 16 is the upperbound, actual is 0..num_in_channels.
// reversed elements (for mBA)
/* should lut8Type and lut16Type be different types? */
// used by lut8Type/lut16Type (mft2) only
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
pub type qcms_data_type = libc::c_uint;
pub const QCMS_DATA_GRAYA_8: qcms_data_type = 4;
pub const QCMS_DATA_GRAY_8: qcms_data_type = 3;
pub const QCMS_DATA_BGRA_8: qcms_data_type = 2;
pub const QCMS_DATA_RGBA_8: qcms_data_type = 1;
pub const QCMS_DATA_RGB_8: qcms_data_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qcms_CIE_xyY {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub Y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qcms_CIE_xyYTRIPLE {
    pub red: qcms_CIE_xyY,
    pub green: qcms_CIE_xyY,
    pub blue: qcms_CIE_xyY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIE_XYZ {
    pub X: libc::c_double,
    pub Y: libc::c_double,
    pub Z: libc::c_double,
}
/* vim: set ts=8 sw=8 noexpandtab: */
//  qcms
//  Copyright (C) 2009 Mozilla Foundation
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub v: [libc::c_float; 3],
}
#[inline]
unsafe extern "C" fn double_to_s15Fixed16Number(mut v: libc::c_double)
 -> s15Fixed16Number {
    return (v * 65536 as libc::c_int as libc::c_double) as int32_t;
}
#[inline]
unsafe extern "C" fn clamp_u8(mut v: libc::c_float) -> libc::c_uchar {
    if v as libc::c_double > 255.0f64 {
        return 255 as libc::c_int as libc::c_uchar
    } else if v < 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int as libc::c_uchar
    } else {
        return floorf((v as libc::c_double + 0.5f64) as libc::c_float) as
                   libc::c_uchar
    };
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
/* for MSVC, GCC, Intel, and Sun compilers */
/* _M_IX86 || __i386__ || __i386 || _M_AMD64 || __x86_64__ || __x86_64 */
/* *
 * AltiVec detection for PowerPC CPUs
 * In case we have a method of detecting do the runtime detection.
 * Otherwise statically choose the AltiVec path in case the compiler
 * was told to build with AltiVec support.
 */
// (defined(__POWERPC__) || defined(__powerpc__))
static mut kRIndex: size_t = 2 as libc::c_int as size_t;
static mut kGIndex: size_t = 1 as libc::c_int as size_t;
static mut kBIndex: size_t = 0 as libc::c_int as size_t;
static mut kAIndex: size_t = 3 as libc::c_int as size_t;
// Build a White point, primary chromas transfer matrix from RGB to CIE XYZ
// This is just an approximation, I am not handling all the non-linear
// aspects of the RGB to XYZ process, and assumming that the gamma correction
// has transitive property in the tranformation chain.
//
// the alghoritm:
//
//            - First I build the absolute conversion matrix using
//              primaries in XYZ. This matrix is next inverted
//            - Then I eval the source white point across this matrix
//              obtaining the coeficients of the transformation
//            - Then, I apply these coeficients to the original matrix
unsafe extern "C" fn build_RGB_to_XYZ_transfer_matrix(mut white: qcms_CIE_xyY,
                                                      mut primrs:
                                                          qcms_CIE_xyYTRIPLE)
 -> matrix {
    let mut primaries: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    let mut primaries_invert: matrix =
        matrix{m: [[0.; 3]; 3], invalid: false,};
    let mut result: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    let mut white_point: vector = vector{v: [0.; 3],};
    let mut coefs: vector = vector{v: [0.; 3],};
    let mut xn: libc::c_double = 0.;
    let mut yn: libc::c_double = 0.;
    let mut xr: libc::c_double = 0.;
    let mut yr: libc::c_double = 0.;
    let mut xg: libc::c_double = 0.;
    let mut yg: libc::c_double = 0.;
    let mut xb: libc::c_double = 0.;
    let mut yb: libc::c_double = 0.;
    xn = white.x;
    yn = white.y;
    if yn == 0.0f64 { return matrix_invalid() }
    xr = primrs.red.x;
    yr = primrs.red.y;
    xg = primrs.green.x;
    yg = primrs.green.y;
    xb = primrs.blue.x;
    yb = primrs.blue.y;
    primaries.m[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        xr as libc::c_float;
    primaries.m[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        xg as libc::c_float;
    primaries.m[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        xb as libc::c_float;
    primaries.m[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        yr as libc::c_float;
    primaries.m[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        yg as libc::c_float;
    primaries.m[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        yb as libc::c_float;
    primaries.m[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        (1 as libc::c_int as libc::c_double - xr - yr) as libc::c_float;
    primaries.m[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        (1 as libc::c_int as libc::c_double - xg - yg) as libc::c_float;
    primaries.m[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        (1 as libc::c_int as libc::c_double - xb - yb) as libc::c_float;
    primaries.invalid = 0 as libc::c_int != 0;
    white_point.v[0 as libc::c_int as usize] = (xn / yn) as libc::c_float;
    white_point.v[1 as libc::c_int as usize] = 1.0f64 as libc::c_float;
    white_point.v[2 as libc::c_int as usize] =
        ((1.0f64 - xn - yn) / yn) as libc::c_float;
    primaries_invert = matrix_invert(primaries);
    if primaries_invert.invalid { return matrix_invalid() }
    coefs = matrix_eval(primaries_invert, white_point);
    result.m[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        (coefs.v[0 as libc::c_int as usize] as libc::c_double * xr) as
            libc::c_float;
    result.m[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (coefs.v[1 as libc::c_int as usize] as libc::c_double * xg) as
            libc::c_float;
    result.m[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        (coefs.v[2 as libc::c_int as usize] as libc::c_double * xb) as
            libc::c_float;
    result.m[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (coefs.v[0 as libc::c_int as usize] as libc::c_double * yr) as
            libc::c_float;
    result.m[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (coefs.v[1 as libc::c_int as usize] as libc::c_double * yg) as
            libc::c_float;
    result.m[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        (coefs.v[2 as libc::c_int as usize] as libc::c_double * yb) as
            libc::c_float;
    result.m[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        (coefs.v[0 as libc::c_int as usize] as libc::c_double *
             (1.0f64 - xr - yr)) as libc::c_float;
    result.m[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        (coefs.v[1 as libc::c_int as usize] as libc::c_double *
             (1.0f64 - xg - yg)) as libc::c_float;
    result.m[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        (coefs.v[2 as libc::c_int as usize] as libc::c_double *
             (1.0f64 - xb - yb)) as libc::c_float;
    result.invalid = primaries_invert.invalid;
    return result;
}
/* CIE Illuminant D50 */
static mut D50_XYZ: CIE_XYZ =
    {
        let mut init = CIE_XYZ{X: 0.9642f64, Y: 1.0000f64, Z: 0.8249f64,};
        init
    };
/* from lcms: xyY2XYZ()
 * corresponds to argyll: icmYxy2XYZ() */
unsafe extern "C" fn xyY2XYZ(mut source: qcms_CIE_xyY) -> CIE_XYZ {
    let mut dest: CIE_XYZ = CIE_XYZ{X: 0., Y: 0., Z: 0.,};
    dest.X = source.x / source.y * source.Y;
    dest.Y = source.Y;
    dest.Z =
        (1 as libc::c_int as libc::c_double - source.x - source.y) / source.y
            * source.Y;
    return dest;
}
/* from lcms: ComputeChromaticAdaption */
// Compute chromatic adaption matrix using chad as cone matrix
unsafe extern "C" fn compute_chromatic_adaption(mut source_white_point:
                                                    CIE_XYZ,
                                                mut dest_white_point: CIE_XYZ,
                                                mut chad: matrix) -> matrix {
    let mut chad_inv: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    let mut cone_source_XYZ: vector = vector{v: [0.; 3],};
    let mut cone_source_rgb: vector = vector{v: [0.; 3],};
    let mut cone_dest_XYZ: vector = vector{v: [0.; 3],};
    let mut cone_dest_rgb: vector = vector{v: [0.; 3],};
    let mut cone: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    let mut tmp: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    tmp = chad;
    chad_inv = matrix_invert(tmp);
    if chad_inv.invalid { return matrix_invalid() }
    cone_source_XYZ.v[0 as libc::c_int as usize] =
        source_white_point.X as libc::c_float;
    cone_source_XYZ.v[1 as libc::c_int as usize] =
        source_white_point.Y as libc::c_float;
    cone_source_XYZ.v[2 as libc::c_int as usize] =
        source_white_point.Z as libc::c_float;
    cone_dest_XYZ.v[0 as libc::c_int as usize] =
        dest_white_point.X as libc::c_float;
    cone_dest_XYZ.v[1 as libc::c_int as usize] =
        dest_white_point.Y as libc::c_float;
    cone_dest_XYZ.v[2 as libc::c_int as usize] =
        dest_white_point.Z as libc::c_float;
    cone_source_rgb = matrix_eval(chad, cone_source_XYZ);
    cone_dest_rgb = matrix_eval(chad, cone_dest_XYZ);
    cone.m[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        cone_dest_rgb.v[0 as libc::c_int as usize] /
            cone_source_rgb.v[0 as libc::c_int as usize];
    cone.m[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    cone.m[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    cone.m[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    cone.m[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        cone_dest_rgb.v[1 as libc::c_int as usize] /
            cone_source_rgb.v[1 as libc::c_int as usize];
    cone.m[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    cone.m[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    cone.m[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    cone.m[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        cone_dest_rgb.v[2 as libc::c_int as usize] /
            cone_source_rgb.v[2 as libc::c_int as usize];
    cone.invalid = 0 as libc::c_int != 0;
    // Normalize
    return matrix_multiply(chad_inv, matrix_multiply(cone, chad));
}
/* from lcms: cmsAdaptionMatrix */
// Returns the final chrmatic adaptation from illuminant FromIll to Illuminant ToIll
// Bradford is assumed
unsafe extern "C" fn adaption_matrix(mut source_illumination: CIE_XYZ,
                                     mut target_illumination: CIE_XYZ)
 -> matrix {
    let mut lam_rigg: matrix =
        {
            let mut init =
                matrix{m:
                           [[0.8951f32, 0.2664f32, -0.1614f32],
                            [-0.7502f32, 1.7135f32, 0.0367f32],
                            [0.0389f32, -0.0685f32, 1.0296f32]],
                       invalid: false,};
            init
        };
    return compute_chromatic_adaption(source_illumination,
                                      target_illumination, lam_rigg);
}
/* from lcms: cmsAdaptMatrixToD50 */
unsafe extern "C" fn adapt_matrix_to_D50(mut r: matrix,
                                         mut source_white_pt: qcms_CIE_xyY)
 -> matrix {
    let mut Dn: CIE_XYZ = CIE_XYZ{X: 0., Y: 0., Z: 0.,};
    let mut Bradford: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    if source_white_pt.y == 0.0f64 { return matrix_invalid() }
    Dn = xyY2XYZ(source_white_pt);
    Bradford = adaption_matrix(Dn, D50_XYZ);
    if Bradford.invalid { return matrix_invalid() }
    return matrix_multiply(Bradford, r);
}
#[no_mangle]
pub unsafe extern "C" fn set_rgb_colorants(mut profile: *mut qcms_profile,
                                           mut white_point: qcms_CIE_xyY,
                                           mut primaries: qcms_CIE_xyYTRIPLE)
 -> bool {
    let mut colorants: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    colorants = build_RGB_to_XYZ_transfer_matrix(white_point, primaries);
    colorants = adapt_matrix_to_D50(colorants, white_point);
    if colorants.invalid { return 0 as libc::c_int != 0 }
    /* note: there's a transpose type of operation going on here */
    (*profile).redColorant.X =
        double_to_s15Fixed16Number(colorants.m[0 as libc::c_int as
                                                   usize][0 as libc::c_int as
                                                              usize] as
                                       libc::c_double);
    (*profile).redColorant.Y =
        double_to_s15Fixed16Number(colorants.m[1 as libc::c_int as
                                                   usize][0 as libc::c_int as
                                                              usize] as
                                       libc::c_double);
    (*profile).redColorant.Z =
        double_to_s15Fixed16Number(colorants.m[2 as libc::c_int as
                                                   usize][0 as libc::c_int as
                                                              usize] as
                                       libc::c_double);
    (*profile).greenColorant.X =
        double_to_s15Fixed16Number(colorants.m[0 as libc::c_int as
                                                   usize][1 as libc::c_int as
                                                              usize] as
                                       libc::c_double);
    (*profile).greenColorant.Y =
        double_to_s15Fixed16Number(colorants.m[1 as libc::c_int as
                                                   usize][1 as libc::c_int as
                                                              usize] as
                                       libc::c_double);
    (*profile).greenColorant.Z =
        double_to_s15Fixed16Number(colorants.m[2 as libc::c_int as
                                                   usize][1 as libc::c_int as
                                                              usize] as
                                       libc::c_double);
    (*profile).blueColorant.X =
        double_to_s15Fixed16Number(colorants.m[0 as libc::c_int as
                                                   usize][2 as libc::c_int as
                                                              usize] as
                                       libc::c_double);
    (*profile).blueColorant.Y =
        double_to_s15Fixed16Number(colorants.m[1 as libc::c_int as
                                                   usize][2 as libc::c_int as
                                                              usize] as
                                       libc::c_double);
    (*profile).blueColorant.Z =
        double_to_s15Fixed16Number(colorants.m[2 as libc::c_int as
                                                   usize][2 as libc::c_int as
                                                              usize] as
                                       libc::c_double);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_rgb_colorants(mut colorants: *mut matrix,
                                           mut white_point: qcms_CIE_xyY,
                                           mut primaries: qcms_CIE_xyYTRIPLE)
 -> bool {
    *colorants = build_RGB_to_XYZ_transfer_matrix(white_point, primaries);
    *colorants = adapt_matrix_to_D50(*colorants, white_point);
    return if (*colorants).invalid as libc::c_int != 0 {
               1 as libc::c_int
           } else { 0 as libc::c_int } != 0;
}
/* Alpha is not corrected.
   A rationale for this is found in Alvy Ray's "Should Alpha Be Nonlinear If
   RGB Is?" Tech Memo 17 (December 14, 1998).
	See: ftp://ftp.alvyray.com/Acrobat/17_Nonln.pdf
*/
#[no_mangle]
pub static mut kInAIndex: size_t = 0xff as libc::c_int as size_t;
#[no_mangle]
pub static mut kOutAIndex: size_t = unsafe { kInAIndex };
unsafe extern "C" fn qcms_transform_data_gray_template_lut(mut transform:
                                                               *const qcms_transform,
                                                           mut src:
                                                               *const libc::c_uchar,
                                                           mut dest:
                                                               *mut libc::c_uchar,
                                                           mut length:
                                                               size_t) {
    let components: libc::c_uint =
        if kOutAIndex == 0xff as libc::c_int as libc::c_ulong {
            3 as libc::c_int
        } else { 4 as libc::c_int } as libc::c_uint;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < length {
        let mut out_device_r: libc::c_float = 0.;
        let mut out_device_g: libc::c_float = 0.;
        let mut out_device_b: libc::c_float = 0.;
        let fresh0 = src;
        src = src.offset(1);
        let mut device: libc::c_uchar = *fresh0;
        let mut alpha: libc::c_uchar = 0xff as libc::c_int as libc::c_uchar;
        if kInAIndex != 0xff as libc::c_int as libc::c_ulong {
            let fresh1 = src;
            src = src.offset(1);
            alpha = *fresh1
        }
        let mut linear: libc::c_float =
            *(*transform).input_gamma_table_gray.offset(device as isize);
        out_device_r =
            lut_interp_linear(linear as libc::c_double,
                              (*transform).output_gamma_lut_r,
                              (*transform).output_gamma_lut_r_length as
                                  libc::c_int);
        out_device_g =
            lut_interp_linear(linear as libc::c_double,
                              (*transform).output_gamma_lut_g,
                              (*transform).output_gamma_lut_g_length as
                                  libc::c_int);
        out_device_b =
            lut_interp_linear(linear as libc::c_double,
                              (*transform).output_gamma_lut_b,
                              (*transform).output_gamma_lut_b_length as
                                  libc::c_int);
        *dest.offset(kRIndex as isize) =
            clamp_u8(out_device_r * 255 as libc::c_int as libc::c_float);
        *dest.offset(kGIndex as isize) =
            clamp_u8(out_device_g * 255 as libc::c_int as libc::c_float);
        *dest.offset(kBIndex as isize) =
            clamp_u8(out_device_b * 255 as libc::c_int as libc::c_float);
        if kOutAIndex != 0xff as libc::c_int as libc::c_ulong {
            *dest.offset(kOutAIndex as isize) = alpha
        }
        dest = dest.offset(components as isize);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn qcms_transform_data_gray_out_lut(mut transform:
                                                          *const qcms_transform,
                                                      mut src:
                                                          *const libc::c_uchar,
                                                      mut dest:
                                                          *mut libc::c_uchar,
                                                      mut length: size_t) {
    //qcms_transform_data_gray_template_lut<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_lut(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_gray_rgba_out_lut(mut transform:
                                                               *const qcms_transform,
                                                           mut src:
                                                               *const libc::c_uchar,
                                                           mut dest:
                                                               *mut libc::c_uchar,
                                                           mut length:
                                                               size_t) {
    //qcms_transform_data_gray_template_lut<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX, NO_A_INDEX, RGBA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_lut(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_gray_bgra_out_lut(mut transform:
                                                               *const qcms_transform,
                                                           mut src:
                                                               *const libc::c_uchar,
                                                           mut dest:
                                                               *mut libc::c_uchar,
                                                           mut length:
                                                               size_t) {
    //qcms_transform_data_gray_template_lut<BGRA_R_INDEX, BGRA_G_INDEX, BGRA_B_INDEX, NO_A_INDEX, BGRA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_lut(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_graya_rgba_out_lut(mut transform:
                                                                *const qcms_transform,
                                                            mut src:
                                                                *const libc::c_uchar,
                                                            mut dest:
                                                                *mut libc::c_uchar,
                                                            mut length:
                                                                size_t) {
    //qcms_transform_data_gray_template_lut<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX, RGBA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_lut(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_graya_bgra_out_lut(mut transform:
                                                                *const qcms_transform,
                                                            mut src:
                                                                *const libc::c_uchar,
                                                            mut dest:
                                                                *mut libc::c_uchar,
                                                            mut length:
                                                                size_t) {
    //qcms_transform_data_gray_template_lut<BGRA_R_INDEX, BGRA_G_INDEX, BGRA_B_INDEX, BGRA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_lut(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_gray_template_precache(mut transform:
                                                                    *const qcms_transform,
                                                                mut src:
                                                                    *const libc::c_uchar,
                                                                mut dest:
                                                                    *mut libc::c_uchar,
                                                                mut length:
                                                                    size_t) {
    let components: libc::c_uint =
        if kOutAIndex == 0xff as libc::c_int as libc::c_ulong {
            3 as libc::c_int
        } else { 4 as libc::c_int } as libc::c_uint;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < length {
        let fresh2 = src;
        src = src.offset(1);
        let mut device: libc::c_uchar = *fresh2;
        let mut alpha: libc::c_uchar = 0xff as libc::c_int as libc::c_uchar;
        if kInAIndex != 0xff as libc::c_int as libc::c_ulong {
            let fresh3 = src;
            src = src.offset(1);
            alpha = *fresh3
        }
        let mut gray: uint16_t = 0;
        let mut linear: libc::c_float =
            *(*transform).input_gamma_table_gray.offset(device as isize);
        /* we could round here... */
        gray =
            (linear *
                 (8192 as libc::c_int - 1 as libc::c_int) as libc::c_float) as
                uint16_t;
        *dest.offset(kRIndex as isize) =
            (*(*transform).output_table_r).data[gray as usize];
        *dest.offset(kGIndex as isize) =
            (*(*transform).output_table_g).data[gray as usize];
        *dest.offset(kBIndex as isize) =
            (*(*transform).output_table_b).data[gray as usize];
        if kOutAIndex != 0xff as libc::c_int as libc::c_ulong {
            *dest.offset(kOutAIndex as isize) = alpha
        }
        dest = dest.offset(components as isize);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn qcms_transform_data_gray_out_precache(mut transform:
                                                               *const qcms_transform,
                                                           mut src:
                                                               *const libc::c_uchar,
                                                           mut dest:
                                                               *mut libc::c_uchar,
                                                           mut length:
                                                               size_t) {
    //qcms_transform_data_gray_template_precache<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_precache(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_gray_rgba_out_precache(mut transform:
                                                                    *const qcms_transform,
                                                                mut src:
                                                                    *const libc::c_uchar,
                                                                mut dest:
                                                                    *mut libc::c_uchar,
                                                                mut length:
                                                                    size_t) {
    //qcms_transform_data_gray_template_precache<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX, NO_A_INDEX, RGBA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_precache(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_gray_bgra_out_precache(mut transform:
                                                                    *const qcms_transform,
                                                                mut src:
                                                                    *const libc::c_uchar,
                                                                mut dest:
                                                                    *mut libc::c_uchar,
                                                                mut length:
                                                                    size_t) {
    //qcms_transform_data_gray_template_precache<BGRA_R_INDEX, BGRA_G_INDEX, BGRA_B_INDEX, NO_A_INDEX, BGRA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_precache(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_graya_rgba_out_precache(mut transform:
                                                                     *const qcms_transform,
                                                                 mut src:
                                                                     *const libc::c_uchar,
                                                                 mut dest:
                                                                     *mut libc::c_uchar,
                                                                 mut length:
                                                                     size_t) {
    //qcms_transform_data_gray_template_precache<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX, RGBA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_precache(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_graya_bgra_out_precache(mut transform:
                                                                     *const qcms_transform,
                                                                 mut src:
                                                                     *const libc::c_uchar,
                                                                 mut dest:
                                                                     *mut libc::c_uchar,
                                                                 mut length:
                                                                     size_t) {
    //qcms_transform_data_gray_template_precache<BGRA_R_INDEX, BGRA_G_INDEX, BGRA_B_INDEX, BGRA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_gray_template_precache(transform, src, dest, length);
}
//template <size_t kRIndex, size_t kGIndex, size_t kBIndex, size_t kAIndex = NO_A_INDEX>
unsafe extern "C" fn qcms_transform_data_template_lut_precache(mut transform:
                                                                   *const qcms_transform,
                                                               mut src:
                                                                   *const libc::c_uchar,
                                                               mut dest:
                                                                   *mut libc::c_uchar,
                                                               mut length:
                                                                   size_t) {
    let components: libc::c_uint =
        if kAIndex == 0xff as libc::c_int as libc::c_ulong {
            3 as libc::c_int
        } else { 4 as libc::c_int } as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut mat: *const [libc::c_float; 4] = (*transform).matrix.as_ptr();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < length {
        let mut device_r: libc::c_uchar = *src.offset(kRIndex as isize);
        let mut device_g: libc::c_uchar = *src.offset(kGIndex as isize);
        let mut device_b: libc::c_uchar = *src.offset(kBIndex as isize);
        let mut alpha: libc::c_uchar = 0;
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            alpha = *src.offset(kAIndex as isize)
        }
        src = src.offset(components as isize);
        let mut r: uint16_t = 0;
        let mut g: uint16_t = 0;
        let mut b: uint16_t = 0;
        let mut linear_r: libc::c_float =
            *(*transform).input_gamma_table_r.offset(device_r as isize);
        let mut linear_g: libc::c_float =
            *(*transform).input_gamma_table_g.offset(device_g as isize);
        let mut linear_b: libc::c_float =
            *(*transform).input_gamma_table_b.offset(device_b as isize);
        let mut out_linear_r: libc::c_float =
            (*mat.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] * linear_r +
                (*mat.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] * linear_g
                +
                (*mat.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] *
                    linear_b;
        let mut out_linear_g: libc::c_float =
            (*mat.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] * linear_r +
                (*mat.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] * linear_g
                +
                (*mat.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] *
                    linear_b;
        let mut out_linear_b: libc::c_float =
            (*mat.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] * linear_r +
                (*mat.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] * linear_g
                +
                (*mat.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] *
                    linear_b;
        out_linear_r = clamp_float(out_linear_r);
        out_linear_g = clamp_float(out_linear_g);
        out_linear_b = clamp_float(out_linear_b);
        /* we could round here... */
        r =
            (out_linear_r *
                 (8192 as libc::c_int - 1 as libc::c_int) as libc::c_float) as
                uint16_t;
        g =
            (out_linear_g *
                 (8192 as libc::c_int - 1 as libc::c_int) as libc::c_float) as
                uint16_t;
        b =
            (out_linear_b *
                 (8192 as libc::c_int - 1 as libc::c_int) as libc::c_float) as
                uint16_t;
        *dest.offset(kRIndex as isize) =
            (*(*transform).output_table_r).data[r as usize];
        *dest.offset(kGIndex as isize) =
            (*(*transform).output_table_g).data[g as usize];
        *dest.offset(kBIndex as isize) =
            (*(*transform).output_table_b).data[b as usize];
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            *dest.offset(kAIndex as isize) = alpha
        }
        dest = dest.offset(components as isize);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgb_out_lut_precache(mut transform:
                                                                      *const qcms_transform,
                                                                  mut src:
                                                                      *const libc::c_uchar,
                                                                  mut dest:
                                                                      *mut libc::c_uchar,
                                                                  mut length:
                                                                      size_t) {
    qcms_transform_data_template_lut_precache(transform, src, dest, length);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgba_out_lut_precache(mut transform:
                                                                       *const qcms_transform,
                                                                   mut src:
                                                                       *const libc::c_uchar,
                                                                   mut dest:
                                                                       *mut libc::c_uchar,
                                                                   mut length:
                                                                       size_t) {
    qcms_transform_data_template_lut_precache(transform, src, dest, length);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_bgra_out_lut_precache(mut transform:
                                                                       *const qcms_transform,
                                                                   mut src:
                                                                       *const libc::c_uchar,
                                                                   mut dest:
                                                                       *mut libc::c_uchar,
                                                                   mut length:
                                                                       size_t) {
    qcms_transform_data_template_lut_precache(transform, src, dest, length);
}
// Not used
/* 
static void qcms_transform_data_clut(const qcms_transform *transform, const unsigned char *src, unsigned char *dest, size_t length) {
	unsigned int i;
	int xy_len = 1;
	int x_len = transform->grid_size;
	int len = x_len * x_len;
	const float* r_table = transform->r_clut;
	const float* g_table = transform->g_clut;
	const float* b_table = transform->b_clut;
  
	for (i = 0; i < length; i++) {
		unsigned char in_r = *src++;
		unsigned char in_g = *src++;
		unsigned char in_b = *src++;
		float linear_r = in_r/255.0f, linear_g=in_g/255.0f, linear_b = in_b/255.0f;

		int x = floorf(linear_r * (transform->grid_size-1));
		int y = floorf(linear_g * (transform->grid_size-1));
		int z = floorf(linear_b * (transform->grid_size-1));
		int x_n = ceilf(linear_r * (transform->grid_size-1));
		int y_n = ceilf(linear_g * (transform->grid_size-1));
		int z_n = ceilf(linear_b * (transform->grid_size-1));
		float x_d = linear_r * (transform->grid_size-1) - x; 
		float y_d = linear_g * (transform->grid_size-1) - y;
		float z_d = linear_b * (transform->grid_size-1) - z; 

		float r_x1 = lerp(CLU(r_table,x,y,z), CLU(r_table,x_n,y,z), x_d);
		float r_x2 = lerp(CLU(r_table,x,y_n,z), CLU(r_table,x_n,y_n,z), x_d);
		float r_y1 = lerp(r_x1, r_x2, y_d);
		float r_x3 = lerp(CLU(r_table,x,y,z_n), CLU(r_table,x_n,y,z_n), x_d);
		float r_x4 = lerp(CLU(r_table,x,y_n,z_n), CLU(r_table,x_n,y_n,z_n), x_d);
		float r_y2 = lerp(r_x3, r_x4, y_d);
		float clut_r = lerp(r_y1, r_y2, z_d);

		float g_x1 = lerp(CLU(g_table,x,y,z), CLU(g_table,x_n,y,z), x_d);
		float g_x2 = lerp(CLU(g_table,x,y_n,z), CLU(g_table,x_n,y_n,z), x_d);
		float g_y1 = lerp(g_x1, g_x2, y_d);
		float g_x3 = lerp(CLU(g_table,x,y,z_n), CLU(g_table,x_n,y,z_n), x_d);
		float g_x4 = lerp(CLU(g_table,x,y_n,z_n), CLU(g_table,x_n,y_n,z_n), x_d);
		float g_y2 = lerp(g_x3, g_x4, y_d);
		float clut_g = lerp(g_y1, g_y2, z_d);

		float b_x1 = lerp(CLU(b_table,x,y,z), CLU(b_table,x_n,y,z), x_d);
		float b_x2 = lerp(CLU(b_table,x,y_n,z), CLU(b_table,x_n,y_n,z), x_d);
		float b_y1 = lerp(b_x1, b_x2, y_d);
		float b_x3 = lerp(CLU(b_table,x,y,z_n), CLU(b_table,x_n,y,z_n), x_d);
		float b_x4 = lerp(CLU(b_table,x,y_n,z_n), CLU(b_table,x_n,y_n,z_n), x_d);
		float b_y2 = lerp(b_x3, b_x4, y_d);
		float clut_b = lerp(b_y1, b_y2, z_d);

		*dest++ = clamp_u8(clut_r*255.0f);
		*dest++ = clamp_u8(clut_g*255.0f);
		*dest++ = clamp_u8(clut_b*255.0f);
	}	
}
*/
unsafe extern "C" fn int_div_ceil(mut value: libc::c_int,
                                  mut div: libc::c_int) -> libc::c_int {
    return (value + div - 1 as libc::c_int) / div;
}
// Using lcms' tetra interpolation algorithm.
unsafe extern "C" fn qcms_transform_data_tetra_clut_template(mut transform:
                                                                 *const qcms_transform,
                                                             mut src:
                                                                 *const libc::c_uchar,
                                                             mut dest:
                                                                 *mut libc::c_uchar,
                                                             mut length:
                                                                 size_t) {
    let components: libc::c_uint =
        if kAIndex == 0xff as libc::c_int as libc::c_ulong {
            3 as libc::c_int
        } else { 4 as libc::c_int } as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut xy_len: libc::c_int = 1 as libc::c_int;
    let mut x_len: libc::c_int = (*transform).grid_size as libc::c_int;
    let mut len: libc::c_int = x_len * x_len;
    let mut r_table: *mut libc::c_float = (*transform).r_clut;
    let mut g_table: *mut libc::c_float = (*transform).g_clut;
    let mut b_table: *mut libc::c_float = (*transform).b_clut;
    let mut c0_r: libc::c_float = 0.;
    let mut c1_r: libc::c_float = 0.;
    let mut c2_r: libc::c_float = 0.;
    let mut c3_r: libc::c_float = 0.;
    let mut c0_g: libc::c_float = 0.;
    let mut c1_g: libc::c_float = 0.;
    let mut c2_g: libc::c_float = 0.;
    let mut c3_g: libc::c_float = 0.;
    let mut c0_b: libc::c_float = 0.;
    let mut c1_b: libc::c_float = 0.;
    let mut c2_b: libc::c_float = 0.;
    let mut c3_b: libc::c_float = 0.;
    let mut clut_r: libc::c_float = 0.;
    let mut clut_g: libc::c_float = 0.;
    let mut clut_b: libc::c_float = 0.;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < length {
        let mut in_r: libc::c_uchar = *src.offset(kRIndex as isize);
        let mut in_g: libc::c_uchar = *src.offset(kGIndex as isize);
        let mut in_b: libc::c_uchar = *src.offset(kBIndex as isize);
        let mut in_a: libc::c_uchar = 0;
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            in_a = *src.offset(kAIndex as isize)
        }
        src = src.offset(components as isize);
        let mut linear_r: libc::c_float =
            in_r as libc::c_int as libc::c_float / 255.0f32;
        let mut linear_g: libc::c_float =
            in_g as libc::c_int as libc::c_float / 255.0f32;
        let mut linear_b: libc::c_float =
            in_b as libc::c_int as libc::c_float / 255.0f32;
        let mut x: libc::c_int =
            in_r as libc::c_int *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) /
                255 as libc::c_int;
        let mut y: libc::c_int =
            in_g as libc::c_int *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) /
                255 as libc::c_int;
        let mut z: libc::c_int =
            in_b as libc::c_int *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) /
                255 as libc::c_int;
        let mut x_n: libc::c_int =
            int_div_ceil(in_r as libc::c_int *
                             ((*transform).grid_size as libc::c_int -
                                  1 as libc::c_int), 255 as libc::c_int);
        let mut y_n: libc::c_int =
            int_div_ceil(in_g as libc::c_int *
                             ((*transform).grid_size as libc::c_int -
                                  1 as libc::c_int), 255 as libc::c_int);
        let mut z_n: libc::c_int =
            int_div_ceil(in_b as libc::c_int *
                             ((*transform).grid_size as libc::c_int -
                                  1 as libc::c_int), 255 as libc::c_int);
        let mut rx: libc::c_float =
            linear_r *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) as
                    libc::c_float - x as libc::c_float;
        let mut ry: libc::c_float =
            linear_g *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) as
                    libc::c_float - y as libc::c_float;
        let mut rz: libc::c_float =
            linear_b *
                ((*transform).grid_size as libc::c_int - 1 as libc::c_int) as
                    libc::c_float - z as libc::c_float;
        c0_r =
            *r_table.offset(((x * len + y * x_len + z * xy_len) *
                                 3 as libc::c_int) as isize);
        c0_g =
            *g_table.offset(((x * len + y * x_len + z * xy_len) *
                                 3 as libc::c_int) as isize);
        c0_b =
            *b_table.offset(((x * len + y * x_len + z * xy_len) *
                                 3 as libc::c_int) as isize);
        if rx >= ry {
            if ry >= rz {
                //rx >= ry && ry >= rz
                c1_r =
                    *r_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize) -
                        c0_r; //rz > rx && rx >= ry
                c2_r =
                    *r_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize) -
                        *r_table.offset(((x_n * len + y * x_len + z * xy_len)
                                             * 3 as libc::c_int) as isize);
                c3_r =
                    *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len)
                                         * 3 as libc::c_int) as isize) -
                        *r_table.offset(((x_n * len + y_n * x_len +
                                              z * xy_len) * 3 as libc::c_int)
                                            as isize);
                c1_g =
                    *g_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize) - c0_g;
                c2_g =
                    *g_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize) -
                        *g_table.offset(((x_n * len + y * x_len + z * xy_len)
                                             * 3 as libc::c_int) as isize);
                c3_g =
                    *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len)
                                         * 3 as libc::c_int) as isize) -
                        *g_table.offset(((x_n * len + y_n * x_len +
                                              z * xy_len) * 3 as libc::c_int)
                                            as isize);
                c1_b =
                    *b_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize) - c0_b;
                c2_b =
                    *b_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize) -
                        *b_table.offset(((x_n * len + y * x_len + z * xy_len)
                                             * 3 as libc::c_int) as isize);
                c3_b =
                    *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len)
                                         * 3 as libc::c_int) as isize) -
                        *b_table.offset(((x_n * len + y_n * x_len +
                                              z * xy_len) * 3 as libc::c_int)
                                            as isize)
            } else if rx >= rz {
                //rx >= rz && rz >= ry
                c1_r =
                    *r_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize) - c0_r;
                c2_r =
                    *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len)
                                         * 3 as libc::c_int) as isize) -
                        *r_table.offset(((x_n * len + y * x_len +
                                              z_n * xy_len) *
                                             3 as libc::c_int) as isize);
                c3_r =
                    *r_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize) -
                        *r_table.offset(((x_n * len + y * x_len + z * xy_len)
                                             * 3 as libc::c_int) as isize);
                c1_g =
                    *g_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize) - c0_g;
                c2_g =
                    *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len)
                                         * 3 as libc::c_int) as isize) -
                        *g_table.offset(((x_n * len + y * x_len +
                                              z_n * xy_len) *
                                             3 as libc::c_int) as isize);
                c3_g =
                    *g_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize) -
                        *g_table.offset(((x_n * len + y * x_len + z * xy_len)
                                             * 3 as libc::c_int) as isize);
                c1_b =
                    *b_table.offset(((x_n * len + y * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize) - c0_b;
                c2_b =
                    *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len)
                                         * 3 as libc::c_int) as isize) -
                        *b_table.offset(((x_n * len + y * x_len +
                                              z_n * xy_len) *
                                             3 as libc::c_int) as isize);
                c3_b =
                    *b_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize) -
                        *b_table.offset(((x_n * len + y * x_len + z * xy_len)
                                             * 3 as libc::c_int) as isize)
            } else {
                c1_r =
                    *r_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize) -
                        *r_table.offset(((x * len + y * x_len + z_n * xy_len)
                                             * 3 as libc::c_int) as isize);
                c2_r =
                    *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len)
                                         * 3 as libc::c_int) as isize) -
                        *r_table.offset(((x_n * len + y * x_len +
                                              z_n * xy_len) *
                                             3 as libc::c_int) as isize);
                c3_r =
                    *r_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize) - c0_r;
                c1_g =
                    *g_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize) -
                        *g_table.offset(((x * len + y * x_len + z_n * xy_len)
                                             * 3 as libc::c_int) as isize);
                c2_g =
                    *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len)
                                         * 3 as libc::c_int) as isize) -
                        *g_table.offset(((x_n * len + y * x_len +
                                              z_n * xy_len) *
                                             3 as libc::c_int) as isize);
                c3_g =
                    *g_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize) - c0_g;
                c1_b =
                    *b_table.offset(((x_n * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize) -
                        *b_table.offset(((x * len + y * x_len + z_n * xy_len)
                                             * 3 as libc::c_int) as isize);
                c2_b =
                    *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len)
                                         * 3 as libc::c_int) as isize) -
                        *b_table.offset(((x_n * len + y * x_len +
                                              z_n * xy_len) *
                                             3 as libc::c_int) as isize);
                c3_b =
                    *b_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize) - c0_b
            }
        } else if rx >= rz {
            //ry > rx && rx >= rz
            c1_r =
                *r_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *r_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as
                                        isize); //rz > ry && ry > rx
            c2_r =
                *r_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                     3 as libc::c_int) as isize) - c0_r;
            c3_r =
                *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *r_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize);
            c1_g =
                *g_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *g_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize);
            c2_g =
                *g_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                     3 as libc::c_int) as isize) - c0_g;
            c3_g =
                *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *g_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize);
            c1_b =
                *b_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *b_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize);
            c2_b =
                *b_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                     3 as libc::c_int) as isize) - c0_b;
            c3_b =
                *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *b_table.offset(((x_n * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize)
        } else if ry >= rz {
            //ry >= rz && rz > rx 
            c1_r =
                *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *r_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize);
            c2_r =
                *r_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                     3 as libc::c_int) as isize) - c0_r;
            c3_r =
                *r_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *r_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize);
            c1_g =
                *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *g_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize);
            c2_g =
                *g_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                     3 as libc::c_int) as isize) - c0_g;
            c3_g =
                *g_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *g_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize);
            c1_b =
                *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *b_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize);
            c2_b =
                *b_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                     3 as libc::c_int) as isize) - c0_b;
            c3_b =
                *b_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *b_table.offset(((x * len + y_n * x_len + z * xy_len) *
                                         3 as libc::c_int) as isize)
        } else {
            c1_r =
                *r_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *r_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize);
            c2_r =
                *r_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *r_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize);
            c3_r =
                *r_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) - c0_r;
            c1_g =
                *g_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *g_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize);
            c2_g =
                *g_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *g_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize);
            c3_g =
                *g_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) - c0_g;
            c1_b =
                *b_table.offset(((x_n * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *b_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize);
            c2_b =
                *b_table.offset(((x * len + y_n * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) -
                    *b_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                         3 as libc::c_int) as isize);
            c3_b =
                *b_table.offset(((x * len + y * x_len + z_n * xy_len) *
                                     3 as libc::c_int) as isize) - c0_b
        }
        clut_r = c0_r + c1_r * rx + c2_r * ry + c3_r * rz;
        clut_g = c0_g + c1_g * rx + c2_g * ry + c3_g * rz;
        clut_b = c0_b + c1_b * rx + c2_b * ry + c3_b * rz;
        *dest.offset(kRIndex as isize) = clamp_u8(clut_r * 255.0f32);
        *dest.offset(kGIndex as isize) = clamp_u8(clut_g * 255.0f32);
        *dest.offset(kBIndex as isize) = clamp_u8(clut_b * 255.0f32);
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            *dest.offset(kAIndex as isize) = in_a
        }
        dest = dest.offset(components as isize);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn qcms_transform_data_tetra_clut_rgb(mut transform:
                                                            *const qcms_transform,
                                                        mut src:
                                                            *const libc::c_uchar,
                                                        mut dest:
                                                            *mut libc::c_uchar,
                                                        mut length: size_t) {
    //qcms_transform_data_tetra_clut_template<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX>(transform, src, dest, length);
    qcms_transform_data_tetra_clut_template(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_tetra_clut_rgba(mut transform:
                                                             *const qcms_transform,
                                                         mut src:
                                                             *const libc::c_uchar,
                                                         mut dest:
                                                             *mut libc::c_uchar,
                                                         mut length: size_t) {
    //qcms_transform_data_tetra_clut_template<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX, RGBA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_tetra_clut_template(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_tetra_clut_bgra(mut transform:
                                                             *const qcms_transform,
                                                         mut src:
                                                             *const libc::c_uchar,
                                                         mut dest:
                                                             *mut libc::c_uchar,
                                                         mut length: size_t) {
    //qcms_transform_data_tetra_clut_template<BGRA_R_INDEX, BGRA_G_INDEX, BGRA_B_INDEX, BGRA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_tetra_clut_template(transform, src, dest, length);
}
unsafe extern "C" fn qcms_transform_data_template_lut(mut transform:
                                                          *const qcms_transform,
                                                      mut src:
                                                          *const libc::c_uchar,
                                                      mut dest:
                                                          *mut libc::c_uchar,
                                                      mut length: size_t) {
    let components: libc::c_uint =
        if kAIndex == 0xff as libc::c_int as libc::c_ulong {
            3 as libc::c_int
        } else { 4 as libc::c_int } as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut mat: *const [libc::c_float; 4] = (*transform).matrix.as_ptr();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < length {
        let mut device_r: libc::c_uchar = *src.offset(kRIndex as isize);
        let mut device_g: libc::c_uchar = *src.offset(kGIndex as isize);
        let mut device_b: libc::c_uchar = *src.offset(kBIndex as isize);
        let mut alpha: libc::c_uchar = 0;
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            alpha = *src.offset(kAIndex as isize)
        }
        src = src.offset(components as isize);
        let mut out_device_r: libc::c_float = 0.;
        let mut out_device_g: libc::c_float = 0.;
        let mut out_device_b: libc::c_float = 0.;
        let mut linear_r: libc::c_float =
            *(*transform).input_gamma_table_r.offset(device_r as isize);
        let mut linear_g: libc::c_float =
            *(*transform).input_gamma_table_g.offset(device_g as isize);
        let mut linear_b: libc::c_float =
            *(*transform).input_gamma_table_b.offset(device_b as isize);
        let mut out_linear_r: libc::c_float =
            (*mat.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] * linear_r +
                (*mat.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] * linear_g
                +
                (*mat.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] *
                    linear_b;
        let mut out_linear_g: libc::c_float =
            (*mat.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] * linear_r +
                (*mat.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] * linear_g
                +
                (*mat.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] *
                    linear_b;
        let mut out_linear_b: libc::c_float =
            (*mat.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] * linear_r +
                (*mat.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] * linear_g
                +
                (*mat.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] *
                    linear_b;
        out_linear_r = clamp_float(out_linear_r);
        out_linear_g = clamp_float(out_linear_g);
        out_linear_b = clamp_float(out_linear_b);
        out_device_r =
            lut_interp_linear(out_linear_r as libc::c_double,
                              (*transform).output_gamma_lut_r,
                              (*transform).output_gamma_lut_r_length as
                                  libc::c_int);
        out_device_g =
            lut_interp_linear(out_linear_g as libc::c_double,
                              (*transform).output_gamma_lut_g,
                              (*transform).output_gamma_lut_g_length as
                                  libc::c_int);
        out_device_b =
            lut_interp_linear(out_linear_b as libc::c_double,
                              (*transform).output_gamma_lut_b,
                              (*transform).output_gamma_lut_b_length as
                                  libc::c_int);
        *dest.offset(kRIndex as isize) =
            clamp_u8(out_device_r * 255 as libc::c_int as libc::c_float);
        *dest.offset(kGIndex as isize) =
            clamp_u8(out_device_g * 255 as libc::c_int as libc::c_float);
        *dest.offset(kBIndex as isize) =
            clamp_u8(out_device_b * 255 as libc::c_int as libc::c_float);
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            *dest.offset(kAIndex as isize) = alpha
        }
        dest = dest.offset(components as isize);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgb_out_lut(mut transform:
                                                             *const qcms_transform,
                                                         mut src:
                                                             *const libc::c_uchar,
                                                         mut dest:
                                                             *mut libc::c_uchar,
                                                         mut length: size_t) {
    //qcms_transform_data_template_lut<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX>(transform, src, dest, length);
    qcms_transform_data_template_lut(transform, src, dest, length);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgba_out_lut(mut transform:
                                                              *const qcms_transform,
                                                          mut src:
                                                              *const libc::c_uchar,
                                                          mut dest:
                                                              *mut libc::c_uchar,
                                                          mut length:
                                                              size_t) {
    //qcms_transform_data_template_lut<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX, RGBA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_template_lut(transform, src, dest, length);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_bgra_out_lut(mut transform:
                                                              *const qcms_transform,
                                                          mut src:
                                                              *const libc::c_uchar,
                                                          mut dest:
                                                              *mut libc::c_uchar,
                                                          mut length:
                                                              size_t) {
    //qcms_transform_data_template_lut<BGRA_R_INDEX, BGRA_G_INDEX, BGRA_B_INDEX, BGRA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_template_lut(transform, src, dest, length);
}
/*
 * If users create and destroy objects on different threads, even if the same
 * objects aren't used on different threads at the same time, we can still run
 * in to trouble with refcounts if they aren't atomic.
 *
 * This can lead to us prematurely deleting the precache if threads get unlucky
 * and write the wrong value to the ref count.
 */
unsafe extern "C" fn precache_reference(mut p: *mut precache_output)
 -> *mut precache_output {
    let fresh4 = &mut (*p).ref_count;
    let fresh5 = 1 as libc::c_int;
    (::std::intrinsics::atomic_xadd(fresh4, fresh5)) + fresh5;
    return p;
}
unsafe extern "C" fn precache_create() -> *mut precache_output {
    let mut p: *mut precache_output =
        malloc(::std::mem::size_of::<precache_output>() as libc::c_ulong) as
            *mut precache_output;
    if !p.is_null() { (*p).ref_count = 1 as libc::c_int }
    return p;
}
/* produces the nearest float to 'a' with a maximum error
 * of 1/1024 which happens for large values like 0x40000040 */
#[no_mangle]
pub unsafe extern "C" fn precache_release(mut p: *mut precache_output) {
    let fresh6 = &mut (*p).ref_count as *mut libc::c_int;
    let fresh7 = 1 as libc::c_int;
    if ::std::intrinsics::atomic_xsub(fresh6, fresh7) - fresh7 ==
           0 as libc::c_int {
        free(p as *mut libc::c_void);
    };
}
unsafe extern "C" fn transform_alloc() -> *mut qcms_transform {
    /* transform needs to be aligned on a 16byte boundrary */
    let mut original_block: *mut libc::c_char =
        calloc((::std::mem::size_of::<qcms_transform>() as
                    libc::c_ulong).wrapping_add(::std::mem::size_of::<*mut libc::c_void>()
                                                    as
                                                    libc::c_ulong).wrapping_add(16
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong),
               1 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    /* make room for a pointer to the block returned by calloc */
    let mut transform_start: *mut libc::c_void =
        original_block.offset(::std::mem::size_of::<*mut libc::c_void>() as
                                  libc::c_ulong as isize) as
            *mut libc::c_void;
    /* align transform_start */
    let mut transform_aligned: *mut qcms_transform =
        ((transform_start as
              uintptr_t).wrapping_add(15 as libc::c_int as libc::c_ulong) &
             !(0xf as libc::c_int) as libc::c_ulong) as *mut qcms_transform;
    /* store a pointer to the block returned by calloc so that we can free it later */
    let mut original_block_ptr: *mut *mut libc::c_void =
        transform_aligned as *mut *mut libc::c_void;
    if original_block.is_null() { return 0 as *mut qcms_transform }
    original_block_ptr = original_block_ptr.offset(-1);
    *original_block_ptr = original_block as *mut libc::c_void;
    return transform_aligned;
}
unsafe extern "C" fn transform_free(mut t: *mut qcms_transform) {
    /* get at the pointer to the unaligned block returned by calloc */
    let mut p: *mut *mut libc::c_void = t as *mut *mut libc::c_void;
    p = p.offset(-1);
    free(*p);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_release(mut t: *mut qcms_transform) {
    /* ensure we only free the gamma tables once even if there are
	 * multiple references to the same data */
    if !(*t).output_table_r.is_null() {
        precache_release((*t).output_table_r);
    }
    if !(*t).output_table_g.is_null() {
        precache_release((*t).output_table_g);
    }
    if !(*t).output_table_b.is_null() {
        precache_release((*t).output_table_b);
    }
    free((*t).input_gamma_table_r as *mut libc::c_void);
    if (*t).input_gamma_table_g != (*t).input_gamma_table_r {
        free((*t).input_gamma_table_g as *mut libc::c_void);
    }
    if (*t).input_gamma_table_g != (*t).input_gamma_table_r &&
           (*t).input_gamma_table_g != (*t).input_gamma_table_b {
        free((*t).input_gamma_table_b as *mut libc::c_void);
    }
    free((*t).input_gamma_table_gray as *mut libc::c_void);
    free((*t).output_gamma_lut_r as *mut libc::c_void);
    free((*t).output_gamma_lut_g as *mut libc::c_void);
    free((*t).output_gamma_lut_b as *mut libc::c_void);
    /* r_clut points to beginning of buffer allocated in qcms_transform_precacheLUT_float */
    if !(*t).r_clut.is_null() { free((*t).r_clut as *mut libc::c_void); }
    transform_free(t);
}
// Determine if we can build with SSE2 (this was partly copied from jmorecfg.h in
// mozilla/jpeg)
 // -------------------------------------------------------------------------
// -------------------------Runtime SSEx Detection-----------------------------
/* MMX is always supported per
 *  Gecko v1.9.1 minimum CPU requirements */
unsafe extern "C" fn sse_version_available() -> libc::c_int {
    /* we know at build time that 64-bit CPUs always have SSE2
	 * this tells the compiler that non-SSE2 branches will never be
	 * taken (i.e. OK to optimze away the SSE1 and non-SIMD code */
    return 2 as libc::c_int;
}
static mut bradford_matrix: matrix =
    {
        let mut init =
            matrix{m:
                       [[0.8951f32, 0.2664f32, -0.1614f32],
                        [-0.7502f32, 1.7135f32, 0.0367f32],
                        [0.0389f32, -0.0685f32, 1.0296f32]],
                   invalid: 0 as libc::c_int != 0,};
        init
    };
static mut bradford_matrix_inv: matrix =
    {
        let mut init =
            matrix{m:
                       [[0.9869929f32, -0.1470543f32, 0.1599627f32],
                        [0.4323053f32, 0.5183603f32, 0.0492912f32],
                        [-0.0085287f32, 0.0400428f32, 0.9684867f32]],
                   invalid: 0 as libc::c_int != 0,};
        init
    };
// See ICCv4 E.3
#[no_mangle]
pub unsafe extern "C" fn compute_whitepoint_adaption(mut X: libc::c_float,
                                                     mut Y: libc::c_float,
                                                     mut Z: libc::c_float)
 -> matrix {
    let mut p: libc::c_float =
        (0.96422f32 *
             bradford_matrix.m[0 as libc::c_int as
                                   usize][0 as libc::c_int as usize] +
             1.000f32 *
                 bradford_matrix.m[1 as libc::c_int as
                                       usize][0 as libc::c_int as usize] +
             0.82521f32 *
                 bradford_matrix.m[2 as libc::c_int as
                                       usize][0 as libc::c_int as usize]) /
            (X *
                 bradford_matrix.m[0 as libc::c_int as
                                       usize][0 as libc::c_int as usize] +
                 Y *
                     bradford_matrix.m[1 as libc::c_int as
                                           usize][0 as libc::c_int as usize] +
                 Z *
                     bradford_matrix.m[2 as libc::c_int as
                                           usize][0 as libc::c_int as usize]);
    let mut y: libc::c_float =
        (0.96422f32 *
             bradford_matrix.m[0 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
             1.000f32 *
                 bradford_matrix.m[1 as libc::c_int as
                                       usize][1 as libc::c_int as usize] +
             0.82521f32 *
                 bradford_matrix.m[2 as libc::c_int as
                                       usize][1 as libc::c_int as usize]) /
            (X *
                 bradford_matrix.m[0 as libc::c_int as
                                       usize][1 as libc::c_int as usize] +
                 Y *
                     bradford_matrix.m[1 as libc::c_int as
                                           usize][1 as libc::c_int as usize] +
                 Z *
                     bradford_matrix.m[2 as libc::c_int as
                                           usize][1 as libc::c_int as usize]);
    let mut b: libc::c_float =
        (0.96422f32 *
             bradford_matrix.m[0 as libc::c_int as
                                   usize][2 as libc::c_int as usize] +
             1.000f32 *
                 bradford_matrix.m[1 as libc::c_int as
                                       usize][2 as libc::c_int as usize] +
             0.82521f32 *
                 bradford_matrix.m[2 as libc::c_int as
                                       usize][2 as libc::c_int as usize]) /
            (X *
                 bradford_matrix.m[0 as libc::c_int as
                                       usize][2 as libc::c_int as usize] +
                 Y *
                     bradford_matrix.m[1 as libc::c_int as
                                           usize][2 as libc::c_int as usize] +
                 Z *
                     bradford_matrix.m[2 as libc::c_int as
                                           usize][2 as libc::c_int as usize]);
    let mut white_adaption: matrix =
        {
            let mut init =
                matrix{m:
                           [[p, 0 as libc::c_int as libc::c_float,
                             0 as libc::c_int as libc::c_float],
                            [0 as libc::c_int as libc::c_float, y,
                             0 as libc::c_int as libc::c_float],
                            [0 as libc::c_int as libc::c_float,
                             0 as libc::c_int as libc::c_float, b]],
                       invalid: 0 as libc::c_int != 0,};
            init
        };
    return matrix_multiply(bradford_matrix_inv,
                           matrix_multiply(white_adaption, bradford_matrix));
}
#[no_mangle]
pub unsafe extern "C" fn qcms_profile_precache_output_transform(mut profile:
                                                                    *mut qcms_profile) {
    /* we only support precaching on rgb profiles */
    if (*profile).color_space != 0x52474220 as libc::c_int as libc::c_uint {
        return
    }
    if qcms_supports_iccv4 {
        /* don't precache since we will use the B2A LUT */
        if !(*profile).B2A0.is_null() { return }
        /* don't precache since we will use the mBA LUT */
        if !(*profile).mBA.is_null() { return }
    }
    /* don't precache if we do not have the TRC curves */
    if (*profile).redTRC.is_null() || (*profile).greenTRC.is_null() ||
           (*profile).blueTRC.is_null() {
        return
    }
    if (*profile).output_table_r.is_null() {
        (*profile).output_table_r = precache_create();
        if !(*profile).output_table_r.is_null() &&
               !compute_precache((*profile).redTRC,
                                 (*(*profile).output_table_r).data.as_mut_ptr())
           {
            precache_release((*profile).output_table_r);
            (*profile).output_table_r = 0 as *mut precache_output
        }
    }
    if (*profile).output_table_g.is_null() {
        (*profile).output_table_g = precache_create();
        if !(*profile).output_table_g.is_null() &&
               !compute_precache((*profile).greenTRC,
                                 (*(*profile).output_table_g).data.as_mut_ptr())
           {
            precache_release((*profile).output_table_g);
            (*profile).output_table_g = 0 as *mut precache_output
        }
    }
    if (*profile).output_table_b.is_null() {
        (*profile).output_table_b = precache_create();
        if !(*profile).output_table_b.is_null() &&
               !compute_precache((*profile).blueTRC,
                                 (*(*profile).output_table_b).data.as_mut_ptr())
           {
            precache_release((*profile).output_table_b);
            (*profile).output_table_b = 0 as *mut precache_output
        }
    };
}
/* Replace the current transformation with a LUT transformation using a given number of sample points */
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_precacheLUT_float(mut transform:
                                                              *mut qcms_transform,
                                                          mut in_0:
                                                              *mut qcms_profile,
                                                          mut out:
                                                              *mut qcms_profile,
                                                          mut samples:
                                                              libc::c_int,
                                                          mut in_type:
                                                              qcms_data_type)
 -> *mut qcms_transform {
    /* The range between which 2 consecutive sample points can be used to interpolate */
    let mut x: uint16_t = 0;
    let mut y: uint16_t = 0;
    let mut z: uint16_t = 0;
    let mut l: uint32_t = 0;
    let mut lutSize: uint32_t =
        (3 as libc::c_int * samples * samples * samples) as uint32_t;
    let mut src: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut dest: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut lut: *mut libc::c_float = 0 as *mut libc::c_float;
    src =
        malloc((lutSize as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                    as libc::c_ulong)) as
            *mut libc::c_float;
    dest =
        malloc((lutSize as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                    as libc::c_ulong)) as
            *mut libc::c_float;
    if !src.is_null() && !dest.is_null() {
        /* Prepare a list of points we want to sample */
        l = 0 as libc::c_int as uint32_t;
        x = 0 as libc::c_int as uint16_t;
        while (x as libc::c_int) < samples {
            y = 0 as libc::c_int as uint16_t;
            while (y as libc::c_int) < samples {
                z = 0 as libc::c_int as uint16_t;
                while (z as libc::c_int) < samples {
                    let fresh8 = l;
                    l = l.wrapping_add(1);
                    *src.offset(fresh8 as isize) =
                        x as libc::c_int as libc::c_float /
                            (samples - 1 as libc::c_int) as libc::c_float;
                    let fresh9 = l;
                    l = l.wrapping_add(1);
                    *src.offset(fresh9 as isize) =
                        y as libc::c_int as libc::c_float /
                            (samples - 1 as libc::c_int) as libc::c_float;
                    let fresh10 = l;
                    l = l.wrapping_add(1);
                    *src.offset(fresh10 as isize) =
                        z as libc::c_int as libc::c_float /
                            (samples - 1 as libc::c_int) as libc::c_float;
                    z = z.wrapping_add(1)
                }
                y = y.wrapping_add(1)
            }
            x = x.wrapping_add(1)
        }
        lut = qcms_chain_transform(in_0, out, src, dest, lutSize as size_t);
        if !lut.is_null() {
            (*transform).r_clut =
                &mut *lut.offset(0 as libc::c_int as isize) as
                    *mut libc::c_float;
            (*transform).g_clut =
                &mut *lut.offset(1 as libc::c_int as isize) as
                    *mut libc::c_float;
            (*transform).b_clut =
                &mut *lut.offset(2 as libc::c_int as isize) as
                    *mut libc::c_float;
            (*transform).grid_size = samples as uint16_t;
            if in_type as libc::c_uint ==
                   QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint {
                (*transform).transform_fn =
                    Some(qcms_transform_data_tetra_clut_rgba as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            } else if in_type as libc::c_uint ==
                          QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint {
                (*transform).transform_fn =
                    Some(qcms_transform_data_tetra_clut_bgra as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            } else if in_type as libc::c_uint ==
                          QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint {
                (*transform).transform_fn =
                    Some(qcms_transform_data_tetra_clut_rgb as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            }
            if (*transform).transform_fn.is_none() as libc::c_int as
                   libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 33],
                                                       &[libc::c_char; 33]>(b"qcms_transform_precacheLUT_float\x00")).as_ptr(),
                             b"transform.c\x00" as *const u8 as
                                 *const libc::c_char, 1130 as libc::c_int,
                             b"transform->transform_fn\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
        }
    }
    //XXX: qcms_modular_transform_data may return either the src or dest buffer. If so it must not be free-ed
	// It will be stored in r_clut, which will be cleaned up in qcms_transform_release.
    if !src.is_null() && lut != src { free(src as *mut libc::c_void); }
    if !dest.is_null() && lut != dest { free(dest as *mut libc::c_void); }
    if lut.is_null() { return 0 as *mut qcms_transform }
    return transform;
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_create(mut in_0: *mut qcms_profile,
                                               mut in_type: qcms_data_type,
                                               mut out: *mut qcms_profile,
                                               mut out_type: qcms_data_type,
                                               mut intent: qcms_intent)
 -> *mut qcms_transform {
    // Ensure the requested input and output types make sense.
    let mut match_0: bool = 0 as libc::c_int != 0;
    if in_type as libc::c_uint ==
           QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint {
        match_0 =
            out_type as libc::c_uint ==
                QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint
    } else if in_type as libc::c_uint ==
                  QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint {
        match_0 =
            out_type as libc::c_uint ==
                QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint
    } else if in_type as libc::c_uint ==
                  QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint {
        match_0 =
            out_type as libc::c_uint ==
                QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint
    } else if in_type as libc::c_uint ==
                  QCMS_DATA_GRAY_8 as libc::c_int as libc::c_uint {
        match_0 =
            out_type as libc::c_uint ==
                QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint ||
                out_type as libc::c_uint ==
                    QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint ||
                out_type as libc::c_uint ==
                    QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint
    } else if in_type as libc::c_uint ==
                  QCMS_DATA_GRAYA_8 as libc::c_int as libc::c_uint {
        match_0 =
            out_type as libc::c_uint ==
                QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint ||
                out_type as libc::c_uint ==
                    QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint
    }
    if !match_0 {
        if !(0 as libc::c_int != 0 &&
                 !(b"input/output type\x00" as *const u8 as
                       *const libc::c_char).is_null()) as libc::c_int as
               libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 22],
                                                   &[libc::c_char; 22]>(b"qcms_transform_create\x00")).as_ptr(),
                         b"transform.c\x00" as *const u8 as
                             *const libc::c_char, 1171 as libc::c_int,
                         b"0 && \"input/output type\"\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        return 0 as *mut qcms_transform
    }
    let mut transform: *mut qcms_transform = transform_alloc();
    if transform.is_null() { return 0 as *mut qcms_transform }
    let mut precache: bool = 0 as libc::c_int != 0;
    if !(*out).output_table_r.is_null() && !(*out).output_table_g.is_null() &&
           !(*out).output_table_b.is_null() {
        precache = 1 as libc::c_int != 0
    }
    // This precache assumes RGB_SIGNATURE (fails on GRAY_SIGNATURE, for instance)
    if qcms_supports_iccv4 as libc::c_int != 0 &&
           (in_type as libc::c_uint ==
                QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint ||
                in_type as libc::c_uint ==
                    QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint ||
                in_type as libc::c_uint ==
                    QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint) &&
           (!(*in_0).A2B0.is_null() || !(*out).B2A0.is_null() ||
                !(*in_0).mAB.is_null() || !(*out).mAB.is_null()) {
        // Precache the transformation to a CLUT 33x33x33 in size.
		// 33 is used by many profiles and works well in pratice. 
		// This evenly divides 256 into blocks of 8x8x8.
		// TODO For transforming small data sets of about 200x200 or less
		// precaching should be avoided.
        let mut result: *mut qcms_transform =
            qcms_transform_precacheLUT_float(transform, in_0, out,
                                             33 as libc::c_int, in_type);
        if result.is_null() {
            if !(0 as libc::c_int != 0 &&
                     !(b"precacheLUT failed\x00" as *const u8 as
                           *const libc::c_char).is_null()) as libc::c_int as
                   libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 22],
                                                       &[libc::c_char; 22]>(b"qcms_transform_create\x00")).as_ptr(),
                             b"transform.c\x00" as *const u8 as
                                 *const libc::c_char, 1199 as libc::c_int,
                             b"0 && \"precacheLUT failed\"\x00" as *const u8
                                 as *const libc::c_char);
            } else { };
            qcms_transform_release(transform);
            return 0 as *mut qcms_transform
        }
        return result
    }
    if precache {
        (*transform).output_table_r =
            precache_reference((*out).output_table_r);
        (*transform).output_table_g =
            precache_reference((*out).output_table_g);
        (*transform).output_table_b =
            precache_reference((*out).output_table_b)
    } else {
        if (*out).redTRC.is_null() || (*out).greenTRC.is_null() ||
               (*out).blueTRC.is_null() {
            qcms_transform_release(transform);
            return 0 as *mut qcms_transform
        }
        build_output_lut((*out).redTRC, &mut (*transform).output_gamma_lut_r,
                         &mut (*transform).output_gamma_lut_r_length);
        build_output_lut((*out).greenTRC,
                         &mut (*transform).output_gamma_lut_g,
                         &mut (*transform).output_gamma_lut_g_length);
        build_output_lut((*out).blueTRC, &mut (*transform).output_gamma_lut_b,
                         &mut (*transform).output_gamma_lut_b_length);
        if (*transform).output_gamma_lut_r.is_null() ||
               (*transform).output_gamma_lut_g.is_null() ||
               (*transform).output_gamma_lut_b.is_null() {
            qcms_transform_release(transform);
            return 0 as *mut qcms_transform
        }
    }
    if (*in_0).color_space == 0x52474220 as libc::c_int as libc::c_uint {
        let mut in_matrix: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
        let mut out_matrix: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
        let mut result_0: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
        if precache {
            if qcms_supports_avx {
                if in_type as libc::c_uint ==
                       QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_rgb_out_lut_avx as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                } else if in_type as libc::c_uint ==
                              QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint
                 {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_rgba_out_lut_avx as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                } else if in_type as libc::c_uint ==
                              QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint
                 {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_bgra_out_lut_avx as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                }
            } else if sse_version_available() >= 2 as libc::c_int {
                if in_type as libc::c_uint ==
                       QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_rgb_out_lut_sse2 as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                } else if in_type as libc::c_uint ==
                              QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint
                 {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_rgba_out_lut_sse2 as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                } else if in_type as libc::c_uint ==
                              QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint
                 {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_bgra_out_lut_sse2 as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                }
                /* Microsoft Compiler for x64 doesn't support MMX.
                     * SSE code uses MMX so that we disable on x64 */
            } else if sse_version_available() >= 1 as libc::c_int {
                if in_type as libc::c_uint ==
                       QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_rgb_out_lut_sse1 as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                } else if in_type as libc::c_uint ==
                              QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint
                 {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_rgba_out_lut_sse1 as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                } else if in_type as libc::c_uint ==
                              QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint
                 {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_bgra_out_lut_sse1 as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                }
            } else if in_type as libc::c_uint ==
                          QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint {
                (*transform).transform_fn =
                    Some(qcms_transform_data_rgb_out_lut_precache as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            } else if in_type as libc::c_uint ==
                          QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint {
                (*transform).transform_fn =
                    Some(qcms_transform_data_rgba_out_lut_precache as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            } else if in_type as libc::c_uint ==
                          QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint {
                (*transform).transform_fn =
                    Some(qcms_transform_data_bgra_out_lut_precache as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            }
        } else if in_type as libc::c_uint ==
                      QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint {
            (*transform).transform_fn =
                Some(qcms_transform_data_rgb_out_lut as
                         unsafe extern "C" fn(_: *const qcms_transform,
                                              _: *const libc::c_uchar,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> ())
        } else if in_type as libc::c_uint ==
                      QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint {
            (*transform).transform_fn =
                Some(qcms_transform_data_rgba_out_lut as
                         unsafe extern "C" fn(_: *const qcms_transform,
                                              _: *const libc::c_uchar,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> ())
        } else if in_type as libc::c_uint ==
                      QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint {
            (*transform).transform_fn =
                Some(qcms_transform_data_bgra_out_lut as
                         unsafe extern "C" fn(_: *const qcms_transform,
                                              _: *const libc::c_uchar,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> ())
        }
        //XXX: avoid duplicating tables if we can
        (*transform).input_gamma_table_r =
            build_input_gamma_table((*in_0).redTRC);
        (*transform).input_gamma_table_g =
            build_input_gamma_table((*in_0).greenTRC);
        (*transform).input_gamma_table_b =
            build_input_gamma_table((*in_0).blueTRC);
        if (*transform).input_gamma_table_r.is_null() ||
               (*transform).input_gamma_table_g.is_null() ||
               (*transform).input_gamma_table_b.is_null() {
            qcms_transform_release(transform);
            return 0 as *mut qcms_transform
        }
        /* build combined colorant matrix */
        in_matrix = build_colorant_matrix(in_0);
        out_matrix = build_colorant_matrix(out);
        out_matrix = matrix_invert(out_matrix);
        if out_matrix.invalid {
            qcms_transform_release(transform);
            return 0 as *mut qcms_transform
        }
        result_0 = matrix_multiply(out_matrix, in_matrix);
        /* check for NaN values in the matrix and bail if we find any */
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < 3 as libc::c_int as libc::c_uint {
            let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while j < 3 as libc::c_int as libc::c_uint {
                if result_0.m[i as usize][j as usize] !=
                       result_0.m[i as usize][j as usize] {
                    qcms_transform_release(transform);
                    return 0 as *mut qcms_transform
                }
                j = j.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
        /* store the results in column major mode
		 * this makes doing the multiplication with sse easier */
        (*transform).matrix[0 as libc::c_int as
                                usize][0 as libc::c_int as usize] =
            result_0.m[0 as libc::c_int as usize][0 as libc::c_int as usize];
        (*transform).matrix[1 as libc::c_int as
                                usize][0 as libc::c_int as usize] =
            result_0.m[0 as libc::c_int as usize][1 as libc::c_int as usize];
        (*transform).matrix[2 as libc::c_int as
                                usize][0 as libc::c_int as usize] =
            result_0.m[0 as libc::c_int as usize][2 as libc::c_int as usize];
        (*transform).matrix[0 as libc::c_int as
                                usize][1 as libc::c_int as usize] =
            result_0.m[1 as libc::c_int as usize][0 as libc::c_int as usize];
        (*transform).matrix[1 as libc::c_int as
                                usize][1 as libc::c_int as usize] =
            result_0.m[1 as libc::c_int as usize][1 as libc::c_int as usize];
        (*transform).matrix[2 as libc::c_int as
                                usize][1 as libc::c_int as usize] =
            result_0.m[1 as libc::c_int as usize][2 as libc::c_int as usize];
        (*transform).matrix[0 as libc::c_int as
                                usize][2 as libc::c_int as usize] =
            result_0.m[2 as libc::c_int as usize][0 as libc::c_int as usize];
        (*transform).matrix[1 as libc::c_int as
                                usize][2 as libc::c_int as usize] =
            result_0.m[2 as libc::c_int as usize][1 as libc::c_int as usize];
        (*transform).matrix[2 as libc::c_int as
                                usize][2 as libc::c_int as usize] =
            result_0.m[2 as libc::c_int as usize][2 as libc::c_int as usize]
    } else if (*in_0).color_space == 0x47524159 as libc::c_int as libc::c_uint
     {
        (*transform).input_gamma_table_gray =
            build_input_gamma_table((*in_0).grayTRC);
        if (*transform).input_gamma_table_gray.is_null() {
            qcms_transform_release(transform);
            return 0 as *mut qcms_transform
        }
        if precache {
            if out_type as libc::c_uint ==
                   QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint {
                (*transform).transform_fn =
                    Some(qcms_transform_data_gray_out_precache as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            } else if out_type as libc::c_uint ==
                          QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint {
                if in_type as libc::c_uint ==
                       QCMS_DATA_GRAY_8 as libc::c_int as libc::c_uint {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_gray_rgba_out_precache as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                } else {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_graya_rgba_out_precache as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                }
            } else if out_type as libc::c_uint ==
                          QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint {
                if in_type as libc::c_uint ==
                       QCMS_DATA_GRAY_8 as libc::c_int as libc::c_uint {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_gray_bgra_out_precache as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                } else {
                    (*transform).transform_fn =
                        Some(qcms_transform_data_graya_bgra_out_precache as
                                 unsafe extern "C" fn(_:
                                                          *const qcms_transform,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: size_t) -> ())
                }
            }
        } else if out_type as libc::c_uint ==
                      QCMS_DATA_RGB_8 as libc::c_int as libc::c_uint {
            (*transform).transform_fn =
                Some(qcms_transform_data_gray_out_lut as
                         unsafe extern "C" fn(_: *const qcms_transform,
                                              _: *const libc::c_uchar,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> ())
        } else if out_type as libc::c_uint ==
                      QCMS_DATA_RGBA_8 as libc::c_int as libc::c_uint {
            if in_type as libc::c_uint ==
                   QCMS_DATA_GRAY_8 as libc::c_int as libc::c_uint {
                (*transform).transform_fn =
                    Some(qcms_transform_data_gray_rgba_out_lut as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            } else {
                (*transform).transform_fn =
                    Some(qcms_transform_data_graya_rgba_out_lut as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            }
        } else if out_type as libc::c_uint ==
                      QCMS_DATA_BGRA_8 as libc::c_int as libc::c_uint {
            if in_type as libc::c_uint ==
                   QCMS_DATA_GRAY_8 as libc::c_int as libc::c_uint {
                (*transform).transform_fn =
                    Some(qcms_transform_data_gray_bgra_out_lut as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            } else {
                (*transform).transform_fn =
                    Some(qcms_transform_data_graya_bgra_out_lut as
                             unsafe extern "C" fn(_: *const qcms_transform,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: size_t) -> ())
            }
        }
    } else {
        if !(0 as libc::c_int != 0 &&
                 !(b"unexpected colorspace\x00" as *const u8 as
                       *const libc::c_char).is_null()) as libc::c_int as
               libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 22],
                                                   &[libc::c_char; 22]>(b"qcms_transform_create\x00")).as_ptr(),
                         b"transform.c\x00" as *const u8 as
                             *const libc::c_char, 1384 as libc::c_int,
                         b"0 && \"unexpected colorspace\"\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        qcms_transform_release(transform);
        return 0 as *mut qcms_transform
    }
    if (*transform).transform_fn.is_none() as libc::c_int as libc::c_long != 0
       {
        __assert_rtn((*::std::mem::transmute::<&[u8; 22],
                                               &[libc::c_char; 22]>(b"qcms_transform_create\x00")).as_ptr(),
                     b"transform.c\x00" as *const u8 as *const libc::c_char,
                     1388 as libc::c_int,
                     b"transform->transform_fn\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    return transform;
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data(mut transform:
                                                 *mut qcms_transform,
                                             mut src: *const libc::c_void,
                                             mut dest: *mut libc::c_void,
                                             mut length: size_t) {
    (*transform).transform_fn.expect("non-null function pointer")(transform,
                                                                  src as
                                                                      *const libc::c_uchar,
                                                                  dest as
                                                                      *mut libc::c_uchar,
                                                                  length);
}
#[no_mangle]
pub static mut qcms_supports_iccv4: bool = false;
#[no_mangle]
pub unsafe extern "C" fn qcms_enable_iccv4() {
    qcms_supports_iccv4 = 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn qcms_enable_neon() { }
#[no_mangle]
pub static mut qcms_supports_avx: bool = false;
#[no_mangle]
pub unsafe extern "C" fn qcms_enable_avx() {
    qcms_supports_avx = 1 as libc::c_int != 0;
}
