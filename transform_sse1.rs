use ::libc;
use crate::transform::{RGB, Format, RGBA, BGRA};
#[cfg(target_arch = "x86")]
pub use std::arch::x86::{__m64, __m128, _mm_add_ps, _mm_mul_ps, _mm_min_ps,
                         _mm_max_ps, _mm_cvtps_pi32, _mm_load_ss, _mm_load_ps,
                         _mm_setzero_ps, _mm_movehl_ps, _mm_shuffle_ps};
#[cfg(target_arch = "x86_64")]
pub use std::arch::x86_64::{__m64, __m128, _mm_add_ps, _mm_mul_ps, _mm_min_ps,
                            _mm_max_ps, _mm_cvtps_pi32, _mm_load_ss,
                            _mm_load_ps, _mm_setzero_ps, _mm_movehl_ps,
                            _mm_shuffle_ps, _mm_empty};
pub type __darwin_size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;

#[repr(C, packed)]#[derive(Copy, Clone)]
pub struct __mm_load_ss_struct {
    pub __u: libc::c_float,
}

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct precache_output {
    pub ref_count: libc::c_int,
    pub data: [uint8_t; 8192],
}
/* if we've already got an ICC_H header we can ignore the following */
/* icc34 defines */
/* **************************************************************** 
 Copyright (c) 1994-1996 SunSoft, Inc.

                    Rights Reserved

Permission is hereby granted, free of charge, to any person 
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without restrict- 
ion, including without limitation the rights to use, copy, modify, 
merge, publish distribute, sublicense, and/or sell copies of the 
Software, and to permit persons to whom the Software is furnished 
to do so, subject to the following conditions: 
 
The above copyright notice and this permission notice shall be 
included in all copies or substantial portions of the Software. 
 
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, 
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES 
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-
INFRINGEMENT.  IN NO EVENT SHALL SUNSOFT, INC. OR ITS PARENT 
COMPANY BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, 
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING 
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR 
OTHER DEALINGS IN THE SOFTWARE. 
 
Except as contained in this notice, the name of SunSoft, Inc. 
shall not be used in advertising or otherwise to promote the 
sale, use or other dealings in this Software without written 
authorization from SunSoft Inc. 
******************************************************************/
/*
 * QCMS, in general, is not threadsafe. However, it should be safe to create
 * profile and transformation objects on different threads, so long as you
 * don't use the same objects on different threads at the same time.
 */
/* 
 * Color Space Signatures
 * Note that only icSigXYZData and icSigLabData are valid
 * Profile Connection Spaces (PCSs)
 */
/* 'XYZ ' */
/* 'Lab ' */
/* 'Luv ' */
/* 'YCbr' */
/* 'Yxy ' */
/* 'RGB ' */
/* 'GRAY' */
/* 'HSV ' */
/* 'HLS ' */
/* 'CMYK' */
/* 'CMY ' */
/* '2CLR' */
/* '3CLR' */
/* '4CLR' */
/* '5CLR' */
/* '6CLR' */
/* '7CLR' */
/* '8CLR' */
/* '9CLR' */
/* 'ACLR' */
/* 'BCLR' */
/* 'CCLR' */
/* 'DCLR' */
/* 'ECLR' */
/* 'FCLR' */
pub type qcms_transform = _qcms_transform;

/* pre-shuffled: just load these into XMM reg instead of load-scalar/shufps sequence */
static mut floatScaleX4: [libc::c_float; 4] =
    [8192f32,
     8192f32,
     8192f32,
     8192f32];
static mut clampMaxValueX4: [libc::c_float; 4] =
    [(8192i32 - 1i32) as libc::c_float /
         8192f32,
     (8192i32 - 1i32) as libc::c_float /
         8192f32,
     (8192i32 - 1i32) as libc::c_float /
         8192f32,
     (8192i32 - 1i32) as libc::c_float /
         8192f32];

unsafe extern "C" fn qcms_transform_data_template_lut_sse1<F: Format>(mut transform:
                                                               *const qcms_transform,
                                                           mut src:
                                                               *const libc::c_uchar,
                                                           mut dest:
                                                               *mut libc::c_uchar,
                                                           mut length:
                                                               size_t) {
    let mut i: libc::c_uint = 0;
    let mut mat: *const [libc::c_float; 4] = (*transform).matrix.as_ptr();
    let mut input_back: [libc::c_char; 32] = [0; 32];
    /* Ensure we have a buffer that's 16 byte aligned regardless of the original
     * stack alignment. We can't use __attribute__((aligned(16))) or __declspec(align(32))
     * because they don't work on stack variables. gcc 4.4 does do the right thing
     * on x86 but that's too new for us right now. For more info: gcc bug #16660 */
    let mut input: *const libc::c_float =
        (&mut *input_back.as_mut_ptr().offset(16isize) as
             *mut libc::c_char as uintptr_t &
             !(0xfi32) as libc::c_ulong) as *mut libc::c_float;
    /* share input and output locations to save having to keep the
     * locations in separate registers */
    let mut output: *const uint32_t = input as *mut uint32_t;
    /* deref *transform now to avoid it in loop */
    let mut igtbl_r: *const libc::c_float = (*transform).input_gamma_table_r;
    let mut igtbl_g: *const libc::c_float = (*transform).input_gamma_table_g;
    let mut igtbl_b: *const libc::c_float = (*transform).input_gamma_table_b;
    /* deref *transform now to avoid it in loop */
    let mut otdata_r: *const uint8_t =
        &mut *(*(*transform).output_table_r).data.as_mut_ptr().offset(0isize)
            as *mut uint8_t;
    let mut otdata_g: *const uint8_t =
        &mut *(*(*transform).output_table_g).data.as_mut_ptr().offset(0isize)
            as *mut uint8_t;
    let mut otdata_b: *const uint8_t =
        &mut *(*(*transform).output_table_b).data.as_mut_ptr().offset(0isize)
            as *mut uint8_t;
    /* input matrix values never change */
    let mat0: __m128 =
        _mm_load_ps((*mat.offset(0isize)).as_ptr());
    let mat1: __m128 =
        _mm_load_ps((*mat.offset(1isize)).as_ptr());
    let mat2: __m128 =
        _mm_load_ps((*mat.offset(2isize)).as_ptr());
    /* these values don't change, either */
    let max: __m128 = _mm_load_ps(clampMaxValueX4.as_ptr());
    let min: __m128 = _mm_setzero_ps();
    let scale: __m128 = _mm_load_ps(floatScaleX4.as_ptr());
    let components: libc::c_uint =
        if F::kAIndex == 0xffu64 {
            3i32
        } else { 4i32 } as libc::c_uint;
    /* working variables */
    let mut vec_r: __m128 = _mm_setzero_ps();
    let mut vec_g: __m128 = _mm_setzero_ps();
    let mut vec_b: __m128 = _mm_setzero_ps();
    let mut result: __m128 = _mm_setzero_ps();
    let mut alpha: libc::c_uchar = 0;
    /* CYA */
    if length == 0 { return }
    /* one pixel is handled outside of the loop */
    length = length.wrapping_sub(1);
    /* setup for transforming 1st pixel */
    vec_r =
        _mm_load_ss(&*igtbl_r.offset(*src.offset(F::kRIndex as isize) as isize));
    vec_g =
        _mm_load_ss(&*igtbl_g.offset(*src.offset(F::kGIndex as isize) as isize));
    vec_b =
        _mm_load_ss(&*igtbl_b.offset(*src.offset(F::kBIndex as isize) as isize));
    if F::kAIndex != 0xffu64 {
        alpha = *src.offset(F::kAIndex as isize)
    }
    src = src.offset(components as isize);
    /* transform all but final pixel */
    i = 0u32;
    while (i as libc::c_ulong) < length {
        /* position values from gamma tables */
        vec_r = _mm_shuffle_ps(vec_r, vec_r, 0i32);
        vec_g = _mm_shuffle_ps(vec_g, vec_g, 0i32);
        vec_b = _mm_shuffle_ps(vec_b, vec_b, 0i32);
        /* gamma * matrix */
        vec_r = _mm_mul_ps(vec_r, mat0);
        vec_g = _mm_mul_ps(vec_g, mat1);
        vec_b = _mm_mul_ps(vec_b, mat2);
        /* store alpha for this pixel; load alpha for next */
        if F::kAIndex != 0xffu64 {
            *dest.offset(F::kAIndex as isize) = alpha;
            alpha = *src.offset(F::kAIndex as isize)
        }
        /* crunch, crunch, crunch */
        vec_r = _mm_add_ps(vec_r, _mm_add_ps(vec_g, vec_b));
        vec_r = _mm_max_ps(min, vec_r);
        vec_r = _mm_min_ps(max, vec_r);
        result = _mm_mul_ps(vec_r, scale);
        /* store calc'd output tables indices */
        *(&*output.offset(0isize) as *const uint32_t as
              *mut __m64) = _mm_cvtps_pi32(result);
        result = _mm_movehl_ps(result, result);
        *(&*output.offset(2isize) as *const uint32_t as
              *mut __m64) = _mm_cvtps_pi32(result);
        /* load gamma values for next loop while store completes */
        vec_r =
            _mm_load_ss(&*igtbl_r.offset(*src.offset(F::kRIndex as isize) as
                                             isize));
        vec_g =
            _mm_load_ss(&*igtbl_g.offset(*src.offset(F::kGIndex as isize) as
                                             isize));
        vec_b =
            _mm_load_ss(&*igtbl_b.offset(*src.offset(F::kBIndex as isize) as
                                             isize));
        src = src.offset(components as isize);
        /* use calc'd indices to output RGB values */
        *dest.offset(F::kRIndex as isize) =
            *otdata_r.offset(*output.offset(0isize) as
                                 isize);
        *dest.offset(F::kGIndex as isize) =
            *otdata_g.offset(*output.offset(1isize) as
                                 isize);
        *dest.offset(F::kBIndex as isize) =
            *otdata_b.offset(*output.offset(2isize) as
                                 isize);
        dest = dest.offset(components as isize);
        i = i.wrapping_add(1)
    }
    /* handle final (maybe only) pixel */
    vec_r = _mm_shuffle_ps(vec_r, vec_r, 0i32);
    vec_g = _mm_shuffle_ps(vec_g, vec_g, 0i32);
    vec_b = _mm_shuffle_ps(vec_b, vec_b, 0i32);
    vec_r = _mm_mul_ps(vec_r, mat0);
    vec_g = _mm_mul_ps(vec_g, mat1);
    vec_b = _mm_mul_ps(vec_b, mat2);
    if F::kAIndex != 0xffu64 {
        *dest.offset(F::kAIndex as isize) = alpha
    }
    vec_r = _mm_add_ps(vec_r, _mm_add_ps(vec_g, vec_b));
    vec_r = _mm_max_ps(min, vec_r);
    vec_r = _mm_min_ps(max, vec_r);
    result = _mm_mul_ps(vec_r, scale);
    *(&*output.offset(0isize) as *const uint32_t as
          *mut __m64) = _mm_cvtps_pi32(result);
    result = _mm_movehl_ps(result, result);
    *(&*output.offset(2isize) as *const uint32_t as
          *mut __m64) = _mm_cvtps_pi32(result);
    *dest.offset(F::kRIndex as isize) =
        *otdata_r.offset(*output.offset(0isize) as isize);
    *dest.offset(F::kGIndex as isize) =
        *otdata_g.offset(*output.offset(1isize) as isize);
    *dest.offset(F::kBIndex as isize) =
        *otdata_b.offset(*output.offset(2isize) as isize);
    _mm_empty();
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgb_out_lut_sse1(mut transform:
                                                                  *const qcms_transform,
                                                              mut src:
                                                                  *const libc::c_uchar,
                                                              mut dest:
                                                                  *mut libc::c_uchar,
                                                              mut length:
                                                                  size_t) {
    qcms_transform_data_template_lut_sse1::<RGB>(transform, src, dest, length);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgba_out_lut_sse1(mut transform:
                                                                   *const qcms_transform,
                                                               mut src:
                                                                   *const libc::c_uchar,
                                                               mut dest:
                                                                   *mut libc::c_uchar,
                                                               mut length:
                                                                   size_t) {
    qcms_transform_data_template_lut_sse1::<RGBA>(transform, src, dest, length);
}
/* vim: set ts=8 sw=8 noexpandtab: */
/* used as a lookup table for the output transformation.
 * we refcount them so we only need to have one around per output
 * profile, instead of duplicating them per transform */
/* We previously used a count of 65536 here but that seems like more
	 * precision than we actually need.  By reducing the size we can
	 * improve startup performance and reduce memory usage. ColorSync on
	 * 10.5 uses 4097 which is perhaps because they use a fixed point
	 * representation where 1. is represented by 0x1000. */
// 16 is the upperbound, actual is 0..num_in_channels.
// reversed elements (for mBA)
/* should lut8Type and lut16Type be different types? */
// used by lut8Type/lut16Type (mft2) only
/* produces the nearest float to 'a' with a maximum error
 * of 1/1024 which happens for large values like 0x40000040 */
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_bgra_out_lut_sse1(mut transform:
                                                                   *const qcms_transform,
                                                               mut src:
                                                                   *const libc::c_uchar,
                                                               mut dest:
                                                                   *mut libc::c_uchar,
                                                               mut length:
                                                                   size_t) {
    //qcms_transform_data_template_lut_sse1<BGRA_R_INDEX, BGRA_G_INDEX, BGRA_B_INDEX, BGRA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_template_lut_sse1::<BGRA>(transform, src, dest, length);
}
