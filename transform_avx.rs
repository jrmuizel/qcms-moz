use ::libc;
#[cfg(target_arch = "x86")]
pub use std::arch::x86::{__m128, __m128i, __m256, __m256i, _mm_add_ps,
                         _mm_mul_ps, _mm_min_ps, _mm_max_ps, _mm_loadu_ps,
                         _mm_cvtps_epi32, _mm_store_si128,
                         _mm256_castps128_ps256, _mm256_castps256_ps128,
                         _mm256_setzero_ps, _mm256_set1_ps, _mm256_set_ps,
                         _mm256_store_si256, _mm256_broadcast_ps,
                         _mm_broadcast_ss, _mm256_cvtps_epi32, _mm256_mul_ps,
                         _mm256_min_ps, _mm256_max_ps, _mm256_add_ps,
                         _mm_setzero_ps};
#[cfg(target_arch = "x86_64")]
pub use std::arch::x86_64::{__m128, __m128i, __m256, __m256i, _mm_add_ps,
                            _mm_mul_ps, _mm_min_ps, _mm_max_ps, _mm_loadu_ps,
                            _mm_cvtps_epi32, _mm_store_si128,
                            _mm256_castps128_ps256, _mm256_castps256_ps128,
                            _mm256_setzero_ps, _mm256_set1_ps, _mm256_set_ps,
                            _mm256_store_si256, _mm256_broadcast_ps,
                            _mm_broadcast_ss, _mm256_cvtps_epi32,
                            _mm256_mul_ps, _mm256_min_ps, _mm256_max_ps,
                            _mm256_add_ps, _mm_setzero_ps, _mm256_insertf128_ps};
pub type __darwin_size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
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
static mut kRIndex: size_t = 2 as libc::c_int as size_t;
static mut kGIndex: size_t = 1 as libc::c_int as size_t;
static mut kBIndex: size_t = 0 as libc::c_int as size_t;
static mut kAIndex: size_t = 3 as libc::c_int as size_t;
//template <size_t kRIndex, size_t kGIndex, size_t kBIndex, size_t kAIndex = NO_A_INDEX>
unsafe extern "C" fn qcms_transform_data_template_lut_avx(mut transform:
                                                              *const qcms_transform,
                                                          mut src:
                                                              *const libc::c_uchar,
                                                          mut dest:
                                                              *mut libc::c_uchar,
                                                          mut length:
                                                              size_t) {
    let mut mat: *const [libc::c_float; 4] = (*transform).matrix.as_ptr();
    let mut input_back: [libc::c_char; 64] = [0; 64];
    /* Ensure we have a buffer that's 32 byte aligned regardless of the original
     * stack alignment. We can't use __attribute__((aligned(32))) or __declspec(align(32))
     * because they don't work on stack variables. gcc 4.4 does do the right thing
     * on x86 but that's too new for us right now. For more info: gcc bug #16660 */
    let mut input: *const libc::c_float =
        (&mut *input_back.as_mut_ptr().offset(32 as libc::c_int as isize) as
             *mut libc::c_char as uintptr_t &
             !(0x1f as libc::c_int) as libc::c_ulong) as *mut libc::c_float;
    /* share input and output locations to save having to keep the
     * locations in separate registers */
    let mut output: *const uint32_t = input as *mut uint32_t;
    /* deref *transform now to avoid it in loop */
    let mut igtbl_r: *const libc::c_float = (*transform).input_gamma_table_r;
    let mut igtbl_g: *const libc::c_float = (*transform).input_gamma_table_g;
    let mut igtbl_b: *const libc::c_float = (*transform).input_gamma_table_b;
    /* deref *transform now to avoid it in loop */
    let mut otdata_r: *const uint8_t =
        &mut *(*(*transform).output_table_r).data.as_mut_ptr().offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
            as *mut uint8_t;
    let mut otdata_g: *const uint8_t =
        &mut *(*(*transform).output_table_g).data.as_mut_ptr().offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
            as *mut uint8_t;
    let mut otdata_b: *const uint8_t =
        &mut *(*(*transform).output_table_b).data.as_mut_ptr().offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
            as *mut uint8_t;
    /* input matrix values never change */
    let mat0: __m256 =
        _mm256_broadcast_ps(&*((*mat.offset(0 as libc::c_int as isize)).as_ptr()
                                as *const __m128));
    let mat1: __m256 =
        _mm256_broadcast_ps(&*((*mat.offset(1 as libc::c_int as isize)).as_ptr()
                                as *const __m128));
    let mat2: __m256 =
        _mm256_broadcast_ps(&*((*mat.offset(2 as libc::c_int as isize)).as_ptr()
                                as *const __m128));
    /* these values don't change, either */
    let max: __m256 =
        _mm256_set1_ps((8192 as libc::c_int - 1 as libc::c_int) as
                           libc::c_float /
                           8192 as libc::c_int as libc::c_float);
    let min: __m256 = _mm256_setzero_ps();
    let scale: __m256 = _mm256_set1_ps(8192 as libc::c_int as libc::c_float);
    let components: libc::c_uint =
        if kAIndex == 0xff as libc::c_int as libc::c_ulong {
            3 as libc::c_int
        } else { 4 as libc::c_int } as libc::c_uint;
    /* working variables */
    let mut vec_r: __m256 = _mm256_setzero_ps();
    let mut vec_g: __m256 = _mm256_setzero_ps();
    let mut vec_b: __m256 = _mm256_setzero_ps();
    let mut result: __m256 = _mm256_setzero_ps();
    let mut vec_r0: __m128 = _mm_setzero_ps();
    let mut vec_g0: __m128 = _mm_setzero_ps();
    let mut vec_b0: __m128 = _mm_setzero_ps();
    let mut vec_r1: __m128 = _mm_setzero_ps();
    let mut vec_g1: __m128 = _mm_setzero_ps();
    let mut vec_b1: __m128 = _mm_setzero_ps();
    let mut alpha1: libc::c_uchar = 0;
    let mut alpha2: libc::c_uchar = 0;
    /* CYA */
    if length == 0 { return }
    /* If there are at least 2 pixels, then we can load their components into
       a single 256-bit register for processing. */
    if length > 1 as libc::c_int as libc::c_ulong {
        vec_r0 =
            _mm_broadcast_ss(&*igtbl_r.offset(*src.offset(kRIndex as isize) as
                                                  isize));
        vec_g0 =
            _mm_broadcast_ss(&*igtbl_g.offset(*src.offset(kGIndex as isize) as
                                                  isize));
        vec_b0 =
            _mm_broadcast_ss(&*igtbl_b.offset(*src.offset(kBIndex as isize) as
                                                  isize));
        vec_r1 =
            _mm_broadcast_ss(&*igtbl_r.offset(*src.offset(kRIndex.wrapping_add(components
                                                                                   as
                                                                                   libc::c_ulong)
                                                              as isize) as
                                                  isize));
        vec_g1 =
            _mm_broadcast_ss(&*igtbl_g.offset(*src.offset(kGIndex.wrapping_add(components
                                                                                   as
                                                                                   libc::c_ulong)
                                                              as isize) as
                                                  isize));
        vec_b1 =
            _mm_broadcast_ss(&*igtbl_b.offset(*src.offset(kBIndex.wrapping_add(components
                                                                                   as
                                                                                   libc::c_ulong)
                                                              as isize) as
                                                  isize));
        vec_r =
            _mm256_insertf128_ps(_mm256_castps128_ps256(vec_r0), vec_r1,
                                 1 as libc::c_int);
        vec_g =
            _mm256_insertf128_ps(_mm256_castps128_ps256(vec_g0), vec_g1,
                                 1 as libc::c_int);
        vec_b =
            _mm256_insertf128_ps(_mm256_castps128_ps256(vec_b0), vec_b1,
                                 1 as libc::c_int);
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            alpha1 = *src.offset(kAIndex as isize);
            alpha2 =
                *src.offset(kAIndex.wrapping_add(components as libc::c_ulong)
                                as isize)
        }
    }
    /* If there are at least 4 pixels, then we can iterate and preload the
       next 2 while we store the result of the current 2. */
    while length > 3 as libc::c_int as libc::c_ulong {
        /* Ensure we are pointing at the next 2 pixels for the next load. */
        src =
            src.offset((2 as libc::c_int as
                            libc::c_uint).wrapping_mul(components) as isize);
        /* gamma * matrix */
        vec_r = _mm256_mul_ps(vec_r, mat0);
        vec_g = _mm256_mul_ps(vec_g, mat1);
        vec_b = _mm256_mul_ps(vec_b, mat2);
        /* store alpha for these pixels; load alpha for next two */
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            *dest.offset(kAIndex as isize) = alpha1;
            *dest.offset(kAIndex.wrapping_add(components as libc::c_ulong) as
                             isize) = alpha2;
            alpha1 = *src.offset(kAIndex as isize);
            alpha2 =
                *src.offset(kAIndex.wrapping_add(components as libc::c_ulong)
                                as isize)
        }
        /* crunch, crunch, crunch */
        vec_r = _mm256_add_ps(vec_r, _mm256_add_ps(vec_g, vec_b));
        vec_r = _mm256_max_ps(min, vec_r);
        vec_r = _mm256_min_ps(max, vec_r);
        result = _mm256_mul_ps(vec_r, scale);
        /* store calc'd output tables indices */
        _mm256_store_si256(output as *mut __m256i,
                           _mm256_cvtps_epi32(result));
        /* load gamma values for next loop while store completes */
        vec_r0 =
            _mm_broadcast_ss(&*igtbl_r.offset(*src.offset(kRIndex as isize) as
                                                  isize));
        vec_g0 =
            _mm_broadcast_ss(&*igtbl_g.offset(*src.offset(kGIndex as isize) as
                                                  isize));
        vec_b0 =
            _mm_broadcast_ss(&*igtbl_b.offset(*src.offset(kBIndex as isize) as
                                                  isize));
        vec_r1 =
            _mm_broadcast_ss(&*igtbl_r.offset(*src.offset(kRIndex.wrapping_add(components
                                                                                   as
                                                                                   libc::c_ulong)
                                                              as isize) as
                                                  isize));
        vec_g1 =
            _mm_broadcast_ss(&*igtbl_g.offset(*src.offset(kGIndex.wrapping_add(components
                                                                                   as
                                                                                   libc::c_ulong)
                                                              as isize) as
                                                  isize));
        vec_b1 =
            _mm_broadcast_ss(&*igtbl_b.offset(*src.offset(kBIndex.wrapping_add(components
                                                                                   as
                                                                                   libc::c_ulong)
                                                              as isize) as
                                                  isize));
        vec_r =
            _mm256_insertf128_ps(_mm256_castps128_ps256(vec_r0), vec_r1,
                                 1 as libc::c_int);
        vec_g =
            _mm256_insertf128_ps(_mm256_castps128_ps256(vec_g0), vec_g1,
                                 1 as libc::c_int);
        vec_b =
            _mm256_insertf128_ps(_mm256_castps128_ps256(vec_b0), vec_b1,
                                 1 as libc::c_int);
        /* use calc'd indices to output RGB values */
        *dest.offset(kRIndex as isize) =
            *otdata_r.offset(*output.offset(0 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kGIndex as isize) =
            *otdata_g.offset(*output.offset(1 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kBIndex as isize) =
            *otdata_b.offset(*output.offset(2 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kRIndex.wrapping_add(components as libc::c_ulong) as
                         isize) =
            *otdata_r.offset(*output.offset(4 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kGIndex.wrapping_add(components as libc::c_ulong) as
                         isize) =
            *otdata_g.offset(*output.offset(5 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kBIndex.wrapping_add(components as libc::c_ulong) as
                         isize) =
            *otdata_b.offset(*output.offset(6 as libc::c_int as isize) as
                                 isize);
        dest =
            dest.offset((2 as libc::c_int as
                             libc::c_uint).wrapping_mul(components) as isize);
        length =
            (length as
                 libc::c_ulong).wrapping_sub(2 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    /* There are 0-3 pixels remaining. If there are 2-3 remaining, then we know
       we have already populated the necessary registers to start the transform. */
    if length > 1 as libc::c_int as libc::c_ulong {
        vec_r = _mm256_mul_ps(vec_r, mat0);
        vec_g = _mm256_mul_ps(vec_g, mat1);
        vec_b = _mm256_mul_ps(vec_b, mat2);
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            *dest.offset(kAIndex as isize) = alpha1;
            *dest.offset(kAIndex.wrapping_add(components as libc::c_ulong) as
                             isize) = alpha2
        }
        vec_r = _mm256_add_ps(vec_r, _mm256_add_ps(vec_g, vec_b));
        vec_r = _mm256_max_ps(min, vec_r);
        vec_r = _mm256_min_ps(max, vec_r);
        result = _mm256_mul_ps(vec_r, scale);
        _mm256_store_si256(output as *mut __m256i,
                           _mm256_cvtps_epi32(result));
        *dest.offset(kRIndex as isize) =
            *otdata_r.offset(*output.offset(0 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kGIndex as isize) =
            *otdata_g.offset(*output.offset(1 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kBIndex as isize) =
            *otdata_b.offset(*output.offset(2 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kRIndex.wrapping_add(components as libc::c_ulong) as
                         isize) =
            *otdata_r.offset(*output.offset(4 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kGIndex.wrapping_add(components as libc::c_ulong) as
                         isize) =
            *otdata_g.offset(*output.offset(5 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kBIndex.wrapping_add(components as libc::c_ulong) as
                         isize) =
            *otdata_b.offset(*output.offset(6 as libc::c_int as isize) as
                                 isize);
        src =
            src.offset((2 as libc::c_int as
                            libc::c_uint).wrapping_mul(components) as isize);
        dest =
            dest.offset((2 as libc::c_int as
                             libc::c_uint).wrapping_mul(components) as isize);
        length =
            (length as
                 libc::c_ulong).wrapping_sub(2 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    /* There may be 0-1 pixels remaining. */
    if length == 1 as libc::c_int as libc::c_ulong {
        vec_r0 =
            _mm_broadcast_ss(&*igtbl_r.offset(*src.offset(kRIndex as isize) as
                                                  isize));
        vec_g0 =
            _mm_broadcast_ss(&*igtbl_g.offset(*src.offset(kGIndex as isize) as
                                                  isize));
        vec_b0 =
            _mm_broadcast_ss(&*igtbl_b.offset(*src.offset(kBIndex as isize) as
                                                  isize));
        vec_r0 = _mm_mul_ps(vec_r0, _mm256_castps256_ps128(mat0));
        vec_g0 = _mm_mul_ps(vec_g0, _mm256_castps256_ps128(mat1));
        vec_b0 = _mm_mul_ps(vec_b0, _mm256_castps256_ps128(mat2));
        if kAIndex != 0xff as libc::c_int as libc::c_ulong {
            *dest.offset(kAIndex as isize) = *src.offset(kAIndex as isize)
        }
        vec_r0 = _mm_add_ps(vec_r0, _mm_add_ps(vec_g0, vec_b0));
        vec_r0 = _mm_max_ps(_mm256_castps256_ps128(min), vec_r0);
        vec_r0 = _mm_min_ps(_mm256_castps256_ps128(max), vec_r0);
        vec_r0 = _mm_mul_ps(vec_r0, _mm256_castps256_ps128(scale));
        _mm_store_si128(output as *mut __m128i, _mm_cvtps_epi32(vec_r0));
        *dest.offset(kRIndex as isize) =
            *otdata_r.offset(*output.offset(0 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kGIndex as isize) =
            *otdata_g.offset(*output.offset(1 as libc::c_int as isize) as
                                 isize);
        *dest.offset(kBIndex as isize) =
            *otdata_b.offset(*output.offset(2 as libc::c_int as isize) as
                                 isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgb_out_lut_avx(mut transform:
                                                                 *const qcms_transform,
                                                             mut src:
                                                                 *const libc::c_uchar,
                                                             mut dest:
                                                                 *mut libc::c_uchar,
                                                             mut length:
                                                                 size_t) {
    //qcms_transform_data_template_lut_avx<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX>(transform, src, dest, length);
    qcms_transform_data_template_lut_avx(transform, src, dest, length);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_rgba_out_lut_avx(mut transform:
                                                                  *const qcms_transform,
                                                              mut src:
                                                                  *const libc::c_uchar,
                                                              mut dest:
                                                                  *mut libc::c_uchar,
                                                              mut length:
                                                                  size_t) {
    //qcms_transform_data_template_lut_avx<RGBA_R_INDEX, RGBA_G_INDEX, RGBA_B_INDEX, RGBA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_template_lut_avx(transform, src, dest, length);
}
#[no_mangle]
pub unsafe extern "C" fn qcms_transform_data_bgra_out_lut_avx(mut transform:
                                                                  *const qcms_transform,
                                                              mut src:
                                                                  *const libc::c_uchar,
                                                              mut dest:
                                                                  *mut libc::c_uchar,
                                                              mut length:
                                                                  size_t) {
    //qcms_transform_data_template_lut_avx<BGRA_R_INDEX, BGRA_G_INDEX, BGRA_B_INDEX, BGRA_A_INDEX>(transform, src, dest, length);
    qcms_transform_data_template_lut_avx(transform, src, dest, length);
}
