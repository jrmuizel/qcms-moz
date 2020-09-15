use ::libc;
use std::mem::zeroed;
use crate::transform::{BGRA, Format, RGBA, RGB, qcms_transform};
//use std::arch::aarch64::{float32x4_t};
use core_arch::arch::aarch64::{uint32x4_t, int32x4_t, float32x4_t, vld1q_f32, vcvtq_u32_f32, vld1q_u32, vgetq_lane_s32, vmulq_f32, vmaxq_f32, vminq_f32, vaddq_f32, vcvtq_s32_f32, vld1q_dup_f32};
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;

/* pre-shuffled: just load these into XMM reg instead of load-scalar/shufps sequence */
static mut floatScaleX4: [f32; 4] =
    [8192f32,
     8192f32,
     8192f32,
     8192f32];
static mut clampMaxValueX4: [f32; 4] =
    [(8192i32 - 1i32) as f32 /
         8192f32,
     (8192i32 - 1i32) as f32 /
         8192f32,
     (8192i32 - 1i32) as f32 /
         8192f32,
     (8192i32 - 1i32) as f32 /
         8192f32];
//template <size_t kRIndex, size_t kGIndex, size_t kBIndex, size_t kAIndex = NO_A_INDEX>
unsafe extern "C" fn qcms_transform_data_template_lut_neon<F: Format>(mut transform:
                                                               *const qcms_transform,
                                                           mut src:
                                                               *const libc::c_uchar,
                                                           mut dest:
                                                               *mut libc::c_uchar,
                                                           mut length:
                                                               size_t) {
    
    let mut mat: *const [f32; 4] = (*transform).matrix.as_ptr();
    let mut input_back: [libc::c_char; 32] = [0; 32];
    /* Ensure we have a buffer that's 16 byte aligned regardless of the original
     * stack alignment. We can't use __attribute__((aligned(16))) or __declspec(align(32))
     * because they don't work on stack variables. gcc 4.4 does do the right thing
     * on x86 but that's too new for us right now. For more info: gcc bug #16660 */
    let mut input: *const f32 =
        (&mut *input_back.as_mut_ptr().offset(16isize) as
             *mut libc::c_char as uintptr_t &
             !(0xfi32) as libc::c_ulong) as *mut f32;
    /* share input and output locations to save having to keep the
     * locations in separate registers */
    let mut output: *const u32 = input as *mut u32;
    /* deref *transform now to avoid it in loop */
    let mut igtbl_r: *const f32 = (*transform).input_gamma_table_r;
    let mut igtbl_g: *const f32 = (*transform).input_gamma_table_g;
    let mut igtbl_b: *const f32 = (*transform).input_gamma_table_b;
    /* deref *transform now to avoid it in loop */
    let mut otdata_r: *const u8 =
        &mut *(*(*transform).output_table_r).data.as_mut_ptr().offset(0isize)
            as *mut u8;
    let mut otdata_g: *const u8 =
        &mut *(*(*transform).output_table_g).data.as_mut_ptr().offset(0isize)
            as *mut u8;
    let mut otdata_b: *const u8 =
        &mut *(*(*transform).output_table_b).data.as_mut_ptr().offset(0isize)
            as *mut u8;
    /* input matrix values never change */
    let mat0: float32x4_t =
        vld1q_f32((*mat.offset(0isize)).as_ptr());
    let mat1: float32x4_t =
        vld1q_f32((*mat.offset(1isize)).as_ptr());
    let mat2: float32x4_t =
        vld1q_f32((*mat.offset(2isize)).as_ptr());
    /* these values don't change, either */
    let max: float32x4_t = vld1q_dup_f32(clampMaxValueX4.as_ptr());
    let min: float32x4_t = zeroed();
    let scale: float32x4_t = vld1q_dup_f32(floatScaleX4.as_ptr());
    let components: libc::c_uint =
        if F::kAIndex == 0xffu64 {
            3i32
        } else { 4i32 } as libc::c_uint;
    /* working variables */
    let mut vec_r: float32x4_t = zeroed();
    let mut vec_g: float32x4_t = zeroed();
    let mut vec_b: float32x4_t = zeroed();
    let mut result: int32x4_t = zeroed();
    let mut alpha: libc::c_uchar = 0;
    /* CYA */
    if length == 0 { return }
    /* one pixel is handled outside of the loop */
    length = length.wrapping_sub(1);
    /* setup for transforming 1st pixel */
    vec_r =
        vld1q_dup_f32(&*igtbl_r.offset(*src.offset(F::kRIndex as isize) as isize));
    vec_g =
        vld1q_dup_f32(&*igtbl_g.offset(*src.offset(F::kGIndex as isize) as isize));
    vec_b =
        vld1q_dup_f32(&*igtbl_b.offset(*src.offset(F::kBIndex as isize) as isize));
    if F::kAIndex != 0xffu64 {
        alpha = *src.offset(F::kAIndex as isize)
    }
    src = src.offset(components as isize);
     let mut i:  libc::c_uint =  0u32;
    while (i as libc::c_ulong) < length {
        /* gamma * matrix */
        vec_r = vmulq_f32(vec_r, mat0);
        vec_g = vmulq_f32(vec_g, mat1);
        vec_b = vmulq_f32(vec_b, mat2);
        /* store alpha for this pixel; load alpha for next */
        if F::kAIndex != 0xffu64 {
            *dest.offset(F::kAIndex as isize) = alpha;
            alpha = *src.offset(F::kAIndex as isize)
        }
        /* crunch, crunch, crunch */
        vec_r = vaddq_f32(vec_r, vaddq_f32(vec_g, vec_b));
        vec_r = vmaxq_f32(min, vec_r);
        vec_r = vminq_f32(max, vec_r);
        result = vcvtq_s32_f32(vmulq_f32(vec_r, scale));
        
        /* use calc'd indices to output RGB values */
        *dest.offset(F::kRIndex as isize) =
            *otdata_r.offset(vgetq_lane_s32(result, 0) as isize);
        *dest.offset(F::kGIndex as isize) =
            *otdata_g.offset(vgetq_lane_s32(result, 1) as isize);
        *dest.offset(F::kBIndex as isize) =
            *otdata_b.offset(vgetq_lane_s32(result, 2) as isize);

        /* load gamma values for next loop while store completes */
        vec_r =
            vld1q_dup_f32(&*igtbl_r.offset(*src.offset(F::kRIndex as isize) as
                                             isize));
        vec_g =
            vld1q_dup_f32(&*igtbl_g.offset(*src.offset(F::kGIndex as isize) as
                                             isize));
        vec_b =
            vld1q_dup_f32(&*igtbl_b.offset(*src.offset(F::kBIndex as isize) as
                                             isize));
        
        dest = dest.offset(components as isize);
        src = src.offset(components as isize);
        i = i.wrapping_add(1)
    }
    /* handle final (maybe only) pixel */
    vec_r = vmulq_f32(vec_r, mat0);
    vec_g = vmulq_f32(vec_g, mat1);
    vec_b = vmulq_f32(vec_b, mat2);
    if F::kAIndex != 0xffu64 {
        *dest.offset(F::kAIndex as isize) = alpha
    }
    vec_r  = vaddq_f32(vec_r, vaddq_f32(vec_g, vec_b));
    vec_r = vmaxq_f32(min, vec_r);
    vec_r = vminq_f32(max, vec_r);
    result = vcvtq_s32_f32(vmulq_f32(vec_r, scale));

    *dest.offset(F::kRIndex as isize) =
        *otdata_r.offset(vgetq_lane_s32(result, 0) as isize);
    *dest.offset(F::kGIndex as isize) =
        *otdata_g.offset(vgetq_lane_s32(result, 1) as isize);
    *dest.offset(F::kBIndex as isize) =
        *otdata_b.offset(vgetq_lane_s32(result, 2)as isize);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgb_out_lut_neon(mut transform:
                                                                  *const qcms_transform,
                                                              mut src:
                                                                  *const libc::c_uchar,
                                                              mut dest:
                                                                  *mut libc::c_uchar,
                                                              mut length:
                                                                  size_t) {
    qcms_transform_data_template_lut_neon::<RGB>(transform, src, dest, length);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgba_out_lut_neon(mut transform:
                                                                   *const qcms_transform,
                                                               mut src:
                                                                   *const libc::c_uchar,
                                                               mut dest:
                                                                   *mut libc::c_uchar,
                                                               mut length:
                                                                   size_t) {
    qcms_transform_data_template_lut_neon::<RGBA>(transform, src, dest, length);
}

#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_bgra_out_lut_neon(mut transform:
                                                                   *const qcms_transform,
                                                               mut src:
                                                                   *const libc::c_uchar,
                                                               mut dest:
                                                                   *mut libc::c_uchar,
                                                               mut length:
                                                                   size_t) {
    qcms_transform_data_template_lut_neon::<BGRA>(transform, src, dest, length);
}
