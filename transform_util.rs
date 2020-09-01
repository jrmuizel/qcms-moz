use ::libc;
extern "C" {
    #[no_mangle]
    fn fabs(_: f64) -> f64;
    #[no_mangle]
    fn pow(_: f64, _: f64) -> f64;
    #[no_mangle]
    fn ceilf(_: f32) -> f32;
    #[no_mangle]
    fn ceil(_: f64) -> f64;
    #[no_mangle]
    fn floorf(_: f32) -> f32;
    #[no_mangle]
    fn floor(_: f64) -> f64;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: i32, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
}
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;

#[repr(C)]#[derive(Copy, Clone)]
pub struct precache_output {
    pub ref_count: i32,
    pub data: [uint8_t; 8192],
}
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;

#[repr(C)]#[derive(Copy, Clone)]
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

#[repr(C)]#[derive(Copy, Clone)]
pub struct matrix {
    pub m: [[f32; 3]; 3],
    pub invalid: bool,
}

#[repr(C)]#[derive(Copy, Clone)]
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
    pub clut_table: *mut f32,
    pub a_curves: [*mut curveType; 10],
    pub b_curves: [*mut curveType; 10],
    pub m_curves: [*mut curveType; 10],
    pub clut_table_data: [f32; 0],
}

#[repr(C)]#[derive(Copy, Clone)]
pub struct curveType {
    pub type_0: uint32_t,
    pub count: uint32_t,
    pub parameter: [f32; 7],
    pub data: [uInt16Number; 0],
}
pub type uInt16Number = uint16_t;
pub type uint32_t = libc::c_uint;
pub type s15Fixed16Number = int32_t;

#[repr(C)]#[derive(Copy, Clone)]
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
    pub input_table: *mut f32,
    pub clut_table: *mut f32,
    pub output_table: *mut f32,
    pub table_data: [f32; 0],
}

#[repr(C)]#[derive(Copy, Clone)]
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
pub type uint16_fract_t = uint16_t;
#[inline]
unsafe extern "C" fn s15Fixed16Number_to_float(mut a: s15Fixed16Number)
 -> f32 {
    return a as f32 / 65536.0f32;
}
#[inline]
unsafe extern "C" fn u8Fixed8Number_to_float(mut x: uint16_t)
 -> f32 {
    return (x as i32 as f64 / 256.0f64) as f32;
}
#[inline]
unsafe extern "C" fn clamp_float(mut a: f32) -> f32 {
    if a as f64 > 1.0f64 {
        return 1f32
    } else if a >= 0f32 {
        return a
    } else { return 0f32 };
}
//'para'
/* value must be a value between 0 and 1 */
//XXX: is the above a good restriction to have?
// the output range of this functions is 0..1
#[no_mangle]
pub unsafe extern "C" fn lut_interp_linear(mut input_value: f64,
                                           mut table: *mut uint16_t,
                                           mut length: i32)
 -> f32 {
    
    
    
    input_value = input_value * (length - 1i32) as f64;
    
    
     let mut upper:  i32 =  ceil(input_value) as i32; let mut lower:  i32 =  floor(input_value) as i32; let mut value:  f32 =
    
        (*table.offset(upper as isize) as i32 as f64 *
             (1.0f64 - (upper as f64 - input_value)) +
             *table.offset(lower as isize) as i32 as f64 *
                 (upper as f64 - input_value)) as f32;
    /* scale the value */
    return value * (1.0f32 / 65535.0f32);
}
/* same as above but takes and returns a uint16_t value representing a range from 0..1 */
#[no_mangle]
pub unsafe extern "C" fn lut_interp_linear16(mut input_value: uint16_t,
                                             mut table: *mut uint16_t,
                                             mut length: i32)
 -> uint16_t {
    /* Start scaling input_value to the length of the array: 65535*(length-1).
	 * We'll divide out the 65535 next */
    let mut value: uint32_t =
        (input_value as i32 * (length - 1i32)) as
            uint32_t; /* equivalent to ceil(value/65535) */
    let mut upper: uint32_t =
        value.wrapping_add(65534u32).wrapping_div(65535u32); /* equivalent to floor(value/65535) */
    let mut lower: uint32_t =
        value.wrapping_div(65535u32);
    /* interp is the distance from upper to value scaled to 0..65535 */
    let mut interp: uint32_t =
        value.wrapping_rem(65535u32); // 0..65535*65535
    value =
        (*table.offset(upper as isize) as
             libc::c_uint).wrapping_mul(interp).wrapping_add((*table.offset(lower
                                                                                as
                                                                                isize)
                                                                  as
                                                                  libc::c_uint).wrapping_mul((65535u32).wrapping_sub(interp))).wrapping_div(65535u32);
    return value as uint16_t;
}
/* same as above but takes an input_value from 0..PRECACHE_OUTPUT_MAX
 * and returns a uint8_t value representing a range from 0..1 */
unsafe extern "C" fn lut_interp_linear_precache_output(mut input_value:
                                                           uint32_t,
                                                       mut table:
                                                           *mut uint16_t,
                                                       mut length:
                                                           i32)
 -> uint8_t {
    /* Start scaling input_value to the length of the array: PRECACHE_OUTPUT_MAX*(length-1).
	 * We'll divide out the PRECACHE_OUTPUT_MAX next */
    let mut value: uint32_t =
        input_value.wrapping_mul((length - 1i32) as libc::c_uint);
    /* equivalent to ceil(value/PRECACHE_OUTPUT_MAX) */
    let mut upper: uint32_t =
        value.wrapping_add((8192i32 - 1i32) as
                               libc::c_uint).wrapping_sub(1u32).wrapping_div((8192i32
                                                                                              -
                                                                                              1i32)
                                                                                             as
                                                                                             libc::c_uint);
    /* equivalent to floor(value/PRECACHE_OUTPUT_MAX) */
    let mut lower: uint32_t =
        value.wrapping_div((8192i32 - 1i32) as
                               libc::c_uint);
    /* interp is the distance from upper to value scaled to 0..PRECACHE_OUTPUT_MAX */
    let mut interp: uint32_t =
        value.wrapping_rem((8192i32 - 1i32) as
                               libc::c_uint);
    /* the table values range from 0..65535 */
    value =
        (*table.offset(upper as isize) as
             libc::c_uint).wrapping_mul(interp).wrapping_add((*table.offset(lower
                                                                                as
                                                                                isize)
                                                                  as
                                                                  libc::c_uint).wrapping_mul(((8192i32
                                                                                                   -
                                                                                                   1i32)
                                                                                                  as
                                                                                                  libc::c_uint).wrapping_sub(interp))); // 0..(65535*PRECACHE_OUTPUT_MAX)
    /* round and scale */
    value =
        
        (value).wrapping_add(((8192i32 -
                                              1i32) *
                                             65535i32 /
                                             255i32 /
                                             2i32) as
                                            libc::c_uint); // scale to 0..255
    value =
        
        (value).wrapping_div(((8192i32 -
                                              1i32) *
                                             65535i32 /
                                             255i32) as
                                            libc::c_uint);
    return value as uint8_t;
}
/* value must be a value between 0 and 1 */
//XXX: is the above a good restriction to have?
#[no_mangle]
pub unsafe extern "C" fn lut_interp_linear_float(mut value: f32,
                                                 mut table:
                                                     *mut f32,
                                                 mut length: i32)
 -> f32 {
    
    
    value = value * (length - 1i32) as f32;
    
     let mut upper:  i32 =  ceilf(value) as i32; let mut lower:  i32 =  floorf(value) as i32;
    //XXX: can we be more performant here?
    value =
        (*table.offset(upper as isize) as f64 *
             (1.0f64 - (upper as f32 - value) as f64) +
             (*table.offset(lower as isize) *
                  (upper as f32 - value)) as f64) as
            f32;
    /* scale the value */
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn compute_curve_gamma_table_type1(mut gamma_table:
                                                             *mut f32,
                                                         mut gamma:
                                                             uint16_t) {
    
    let mut gamma_float: f32 = u8Fixed8Number_to_float(gamma);
     let mut i:  libc::c_uint =  0u32;
    while i < 256u32 {
        // 0..1^(0..255 + 255/256) will always be between 0 and 1
        *gamma_table.offset(i as isize) =
            pow(i as f64 / 255.0f64, gamma_float as f64)
                as f32;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_curve_gamma_table_type2(mut gamma_table:
                                                             *mut f32,
                                                         mut table:
                                                             *mut uint16_t,
                                                         mut length:
                                                             i32) {
    
     let mut i:  libc::c_uint =  0u32;
    while i < 256u32 {
        *gamma_table.offset(i as isize) =
            lut_interp_linear(i as f64 / 255.0f64, table, length);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_curve_gamma_table_type_parametric(mut gamma_table:
                                                                       *mut f32,
                                                                   mut parameter:
                                                                       *mut f32,
                                                                   mut count:
                                                                       i32) {
    
    let mut interval: f32 = 0.;
    let mut a: f32 = 0.;
    let mut b: f32 = 0.;
    let mut c: f32 = 0.;
    let mut e: f32 = 0.;
    let mut f: f32 = 0.;
    let mut y: f32 = *parameter.offset(0isize);
    if count == 0i32 {
        a = 1f32;
        b = 0f32;
        c = 0f32;
        e = 0f32;
        f = 0f32;
        interval = -1f32
    } else if count == 1i32 {
        a = *parameter.offset(1isize);
        b = *parameter.offset(2isize);
        c = 0f32;
        e = 0f32;
        f = 0f32;
        interval =
            -1f32 *
                *parameter.offset(2isize) /
                *parameter.offset(1isize)
    } else if count == 2i32 {
        a = *parameter.offset(1isize);
        b = *parameter.offset(2isize);
        c = 0f32;
        e = *parameter.offset(3isize);
        f = *parameter.offset(3isize);
        interval =
            -1f32 *
                *parameter.offset(2isize) /
                *parameter.offset(1isize)
    } else if count == 3i32 {
        a = *parameter.offset(1isize);
        b = *parameter.offset(2isize);
        c = *parameter.offset(3isize);
        e = -c;
        f = 0f32;
        interval = *parameter.offset(4isize)
    } else if count == 4i32 {
        a = *parameter.offset(1isize);
        b = *parameter.offset(2isize);
        c = *parameter.offset(3isize);
        e = *parameter.offset(5isize) - c;
        f = *parameter.offset(6isize);
        interval = *parameter.offset(4isize)
    } else {
        if !(0i32 != 0 &&
                 !(b"invalid parametric function type.\x00" as *const u8 as
                       *const libc::c_char).is_null()) as i32 as
               libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 42],
                                                   &[libc::c_char; 42]>(b"compute_curve_gamma_table_type_parametric\x00")).as_ptr(),
                         b"transform_util.c\x00" as *const u8 as
                             *const libc::c_char, 158i32,
                         b"0 && \"invalid parametric function type.\"\x00" as
                             *const u8 as *const libc::c_char);
        } else { };
        a = 1f32;
        b = 0f32;
        c = 0f32;
        e = 0f32;
        f = 0f32;
        interval = -1f32
    }
     let mut X:  size_t =  0u64;
    while X < 256u64 {
        if X as f32 >= interval {
            // XXX The equations are not exactly as defined in the spec but are
                        //     algebraically equivalent.
                        // TODO Should division by 255 be for the whole expression.
            *gamma_table.offset(X as isize) =
                clamp_float((pow((a * X as f32) as f64 /
                                     255.0f64 + b as f64,
                                 y as f64) + c as f64 +
                                 e as f64) as f32)
        } else {
            *gamma_table.offset(X as isize) =
                clamp_float(((c * X as f32) as f64 /
                                 255.0f64 + f as f64) as
                                f32)
        }
        X = X.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_curve_gamma_table_type0(mut gamma_table:
                                                             *mut f32) {
    
     let mut i:  libc::c_uint =  0u32;
    while i < 256u32 {
        *gamma_table.offset(i as isize) =
            (i as f64 / 255.0f64) as f32;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn build_input_gamma_table(mut TRC: *mut curveType)
 -> *mut f32 {
    
    if TRC.is_null() { return 0 as *mut f32 }
     let mut gamma_table:  *mut f32 =
    
        malloc((::std::mem::size_of::<f32>() as
                    libc::c_ulong).wrapping_mul(256u64)) as
            *mut f32;
    if !gamma_table.is_null() {
        if (*TRC).type_0 == 0x70617261u32 {
            compute_curve_gamma_table_type_parametric(gamma_table,
                                                      (*TRC).parameter.as_mut_ptr(),
                                                      (*TRC).count as
                                                          i32);
        } else if (*TRC).count == 0u32 {
            compute_curve_gamma_table_type0(gamma_table);
        } else if (*TRC).count == 1u32 {
            compute_curve_gamma_table_type1(gamma_table,
                                            *(*TRC).data.as_mut_ptr().offset(0isize));
        } else {
            compute_curve_gamma_table_type2(gamma_table,
                                            (*TRC).data.as_mut_ptr(),
                                            (*TRC).count as i32);
        }
    }
    return gamma_table;
}
#[no_mangle]
pub unsafe extern "C" fn build_colorant_matrix(mut p: *mut qcms_profile)
 -> matrix {
    let mut result: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    result.m[0usize][0usize] =
        s15Fixed16Number_to_float((*p).redColorant.X);
    result.m[0usize][1usize] =
        s15Fixed16Number_to_float((*p).greenColorant.X);
    result.m[0usize][2usize] =
        s15Fixed16Number_to_float((*p).blueColorant.X);
    result.m[1usize][0usize] =
        s15Fixed16Number_to_float((*p).redColorant.Y);
    result.m[1usize][1usize] =
        s15Fixed16Number_to_float((*p).greenColorant.Y);
    result.m[1usize][2usize] =
        s15Fixed16Number_to_float((*p).blueColorant.Y);
    result.m[2usize][0usize] =
        s15Fixed16Number_to_float((*p).redColorant.Z);
    result.m[2usize][1usize] =
        s15Fixed16Number_to_float((*p).greenColorant.Z);
    result.m[2usize][2usize] =
        s15Fixed16Number_to_float((*p).blueColorant.Z);
    result.invalid = 0i32 != 0;
    return result;
}
/* The following code is copied nearly directly from lcms.
 * I think it could be much better. For example, Argyll seems to have better code in
 * icmTable_lookup_bwd and icmTable_setup_bwd. However, for now this is a quick way
 * to a working solution and allows for easy comparing with lcms. */
#[no_mangle]
pub unsafe extern "C" fn lut_inverse_interp16(mut Value: uint16_t,
                                              mut LutTable: *mut uint16_t,
                                              mut length: i32)
 -> uint16_fract_t {
    let mut l: i32 =
        1i32; // 'int' Give spacing for negative values
    let mut r: i32 = 0x10000i32;
    let mut x: i32 = 0i32;
    let mut res: i32 = 0;
    
    
    
    
    
    
    
    
    
    
    
    
     let mut NumZeroes:  i32 =  0i32;
    while *LutTable.offset(NumZeroes as isize) as i32 ==
              0i32 && NumZeroes < length - 1i32 {
        NumZeroes += 1
    }
    // There are no zeros at the beginning and we are trying to find a zero, so
        // return anything. It seems zero would be the less destructive choice
	/* I'm not sure that this makes sense, but oh well... */
    if NumZeroes == 0i32 &&
           Value as i32 == 0i32 {
        return 0u16
    }
     let mut NumPoles:  i32 =  0i32;
    while *LutTable.offset((length - 1i32 - NumPoles) as isize) as
              i32 == 0xffffi32 &&
              NumPoles < length - 1i32 {
        NumPoles += 1
    }
    // Does the curve belong to this case?
    if NumZeroes > 1i32 || NumPoles > 1i32 {
        let mut a_0: i32 = 0;
        let mut b_0: i32 = 0;
        // Identify if value fall downto 0 or FFFF zone
        if Value as i32 == 0i32 {
            return 0u16
        }
        // if (Value == 0xFFFF) return 0xFFFF;
        // else restrict to valid zone
        if NumZeroes > 1i32 {
            a_0 =
                (NumZeroes - 1i32) * 0xffffi32 /
                    (length - 1i32);
            l = a_0 - 1i32
        }
        if NumPoles > 1i32 {
            b_0 =
                (length - 1i32 - NumPoles) * 0xffffi32
                    / (length - 1i32);
            r = b_0 + 1i32
        }
    }
    if r <= l {
        // If this happens LutTable is not invertible
        return 0u16
    }
    // Seems not a degenerated case... apply binary search
    while r > l {
        x = (l + r) / 2i32;
        res =
            lut_interp_linear16((x - 1i32) as uint16_fract_t,
                                LutTable, length) as i32;
        if res == Value as i32 {
            // Found exact match.
            return (x - 1i32) as uint16_fract_t
        }
        if res > Value as i32 {
            r = x - 1i32
        } else { l = x + 1i32 }
    }
    // Not found, should we interpolate?
    // Get surrounding nodes
    if !(x >= 1i32) as i32 as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 21],
                                               &[libc::c_char; 21]>(b"lut_inverse_interp16\x00")).as_ptr(),
                     b"transform_util.c\x00" as *const u8 as
                         *const libc::c_char, 307i32,
                     b"x >= 1\x00" as *const u8 as *const libc::c_char);
    } else { };
    
    
     let mut val2:  f64 =
    
        (length - 1i32) as f64 *
            ((x - 1i32) as f64 / 65535.0f64); let mut cell0:  i32 =  floor(val2) as i32; let mut cell1:  i32 =  ceil(val2) as i32;
    if cell0 == cell1 { return x as uint16_fract_t }
    
    
    
    
    
     let mut y0:  f64 =
     *LutTable.offset(cell0 as isize) as f64; let mut x0:  f64 =
    
        65535.0f64 * cell0 as f64 /
            (length - 1i32) as f64; let mut y1:  f64 =
     *LutTable.offset(cell1 as isize) as f64; let mut x1:  f64 =
    
        65535.0f64 * cell1 as f64 /
            (length - 1i32) as f64; let mut a:  f64 =  (y1 - y0) / (x1 - x0); let mut b:  f64 =  y0 - a * x0;
    if fabs(a) < 0.01f64 { return x as uint16_fract_t }
     let mut f:  f64 =  (Value as i32 as f64 - b) / a;
    if f < 0.0f64 { return 0u16 }
    if f >= 65535.0f64 { return 0xffffu16 }
    return floor(f + 0.5f64) as uint16_fract_t;
}
/*
 The number of entries needed to invert a lookup table should not
 necessarily be the same as the original number of entries.  This is
 especially true of lookup tables that have a small number of entries.

 For example:
 Using a table like:
    {0, 3104, 14263, 34802, 65535}
 invert_lut will produce an inverse of:
    {3, 34459, 47529, 56801, 65535}
 which has an maximum error of about 9855 (pixel difference of ~38.346)

 For now, we punt the decision of output size to the caller. */
unsafe extern "C" fn invert_lut(mut table: *mut uint16_t,
                                mut length: i32,
                                mut out_length: i32)
 -> *mut uint16_t {
    
    /* for now we invert the lut by creating a lut of size out_length
         * and attempting to lookup a value for each entry using lut_inverse_interp16 */
    let mut output: *mut uint16_t =
        malloc((::std::mem::size_of::<uint16_t>() as
                    libc::c_ulong).wrapping_mul(out_length as libc::c_ulong))
            as *mut uint16_t;
    if output.is_null() { return 0 as *mut uint16_t }
     let mut i:  i32 =  0i32;
    while i < out_length {
        let mut x: f64 =
            i as f64 * 65535.0f64 /
                (out_length - 1i32) as f64;
        let mut input: uint16_fract_t = floor(x + 0.5f64) as uint16_fract_t;
        *output.offset(i as isize) =
            lut_inverse_interp16(input, table, length);
        i += 1
    }
    return output;
}
unsafe extern "C" fn compute_precache_pow(mut output: *mut uint8_t,
                                          mut gamma: f32) {
    
     let mut v:  uint32_t =  0u32;
    while v < 8192u32 {
        //XXX: don't do integer/float conversion... and round?
        *output.offset(v as isize) =
            (255.0f64 *
                 pow(v as f64 /
                         (8192i32 - 1i32) as
                             f64, gamma as f64)) as
                uint8_t;
        v = v.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_precache_lut(mut output: *mut uint8_t,
                                              mut table: *mut uint16_t,
                                              mut length: i32) {
    
     let mut v:  uint32_t =  0u32;
    while v < 8192u32 {
        *output.offset(v as isize) =
            lut_interp_linear_precache_output(v, table, length);
        v = v.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_precache_linear(mut output: *mut uint8_t) {
    
     let mut v:  uint32_t =  0u32;
    while v < 8192u32 {
        //XXX: round?
        *output.offset(v as isize) =
            v.wrapping_div((8192i32 / 256i32) as
                               libc::c_uint) as uint8_t;
        v = v.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_precache(mut trc: *mut curveType,
                                          mut output: *mut uint8_t) -> bool {
    if (*trc).type_0 == 0x70617261u32 {
        let mut gamma_table: [f32; 256] = [0.; 256];
        let mut gamma_table_uint: [uint16_t; 256] = [0; 256];
        
        
        let mut inverted_size: i32 = 256i32;
        compute_curve_gamma_table_type_parametric(gamma_table.as_mut_ptr(),
                                                  (*trc).parameter.as_mut_ptr(),
                                                  (*trc).count as
                                                      i32);
         let mut i:  uint16_t =  0u16;
        while (i as i32) < 256i32 {
            gamma_table_uint[i as usize] =
                (gamma_table[i as usize] *
                     65535f32) as uint16_t;
            i = i.wrapping_add(1)
        }
        //XXX: the choice of a minimum of 256 here is not backed by any theory, 
                        //     measurement or data, howeve r it is what lcms uses.
                        //     the maximum number we would need is 65535 because that's the 
                        //     accuracy used for computing the pre cache table
        if inverted_size < 256i32 {
            inverted_size = 256i32
        }
         let mut inverted:  *mut uint16_t =
    
            invert_lut(gamma_table_uint.as_mut_ptr(), 256i32,
                       inverted_size);
        if inverted.is_null() { return 0i32 != 0 }
        compute_precache_lut(output, inverted, inverted_size);
        free(inverted as *mut libc::c_void);
    } else if (*trc).count == 0u32 {
        compute_precache_linear(output);
    } else if (*trc).count == 1u32 {
        compute_precache_pow(output,
                             (1.0f64 /
                                  u8Fixed8Number_to_float(*(*trc).data.as_mut_ptr().offset(0isize))
                                      as f64) as f32);
    } else {
        
        let mut inverted_size_0: i32 = (*trc).count as i32;
        //XXX: the choice of a minimum of 256 here is not backed by any theory, 
                        //     measurement or data, howeve r it is what lcms uses.
                        //     the maximum number we would need is 65535 because that's the 
                        //     accuracy used for computing the pre cache table
        if inverted_size_0 < 256i32 {
            inverted_size_0 = 256i32
        } //XXX turn this conversion into a function
         let mut inverted_0:  *mut uint16_t =
    
            invert_lut((*trc).data.as_mut_ptr(), (*trc).count as i32,
                       inverted_size_0);
        if inverted_0.is_null() { return 0i32 != 0 }
        compute_precache_lut(output, inverted_0, inverted_size_0);
        free(inverted_0 as *mut libc::c_void);
    }
    return 1i32 != 0;
}
unsafe extern "C" fn build_linear_table(mut length: i32)
 -> *mut uint16_t {
    
    let mut output: *mut uint16_t =
        malloc((::std::mem::size_of::<uint16_t>() as
                    libc::c_ulong).wrapping_mul(length as libc::c_ulong)) as
            *mut uint16_t;
    if output.is_null() { return 0 as *mut uint16_t }
     let mut i:  i32 =  0i32;
    while i < length {
        let mut x: f64 =
            i as f64 * 65535.0f64 /
                (length - 1i32) as f64;
        let mut input: uint16_fract_t = floor(x + 0.5f64) as uint16_fract_t;
        *output.offset(i as isize) = input;
        i += 1
    }
    return output;
}
unsafe extern "C" fn build_pow_table(mut gamma: f32,
                                     mut length: i32)
 -> *mut uint16_t {
    
    let mut output: *mut uint16_t =
        malloc((::std::mem::size_of::<uint16_t>() as
                    libc::c_ulong).wrapping_mul(length as libc::c_ulong)) as
            *mut uint16_t;
    if output.is_null() { return 0 as *mut uint16_t }
     let mut i:  i32 =  0i32;
    while i < length {
        
        let mut x: f64 =
            i as f64 /
                (length - 1i32) as f64;
        x = pow(x, gamma as f64);
         let mut result:  uint16_fract_t =
     floor(x * 65535.0f64 + 0.5f64) as uint16_fract_t;
        *output.offset(i as isize) = result;
        i += 1
    }
    return output;
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
//XXX: could use a bettername
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
// a < 0 or a is NaN
// 0x0000 = 0.
  // 0x0100 = 1.
  // 0xffff = 255  + 255/256
#[no_mangle]
pub unsafe extern "C" fn build_output_lut(mut trc: *mut curveType,
                                          mut output_gamma_lut:
                                              *mut *mut uint16_t,
                                          mut output_gamma_lut_length:
                                              *mut size_t) {
    if (*trc).type_0 == 0x70617261u32 {
        let mut gamma_table: [f32; 256] = [0.; 256];
        
        let mut output: *mut uint16_t =
            malloc((::std::mem::size_of::<uint16_t>() as
                        libc::c_ulong).wrapping_mul(256u64)) as
                *mut uint16_t;
        if output.is_null() { *output_gamma_lut = 0 as *mut uint16_t; return }
        compute_curve_gamma_table_type_parametric(gamma_table.as_mut_ptr(),
                                                  (*trc).parameter.as_mut_ptr(),
                                                  (*trc).count as
                                                      i32);
        *output_gamma_lut_length = 256u64;
         let mut i:  uint16_t =  0u16;
        while (i as i32) < 256i32 {
            *output.offset(i as isize) =
                (gamma_table[i as usize] *
                     65535f32) as uint16_t;
            i = i.wrapping_add(1)
        }
        *output_gamma_lut = output
    } else if (*trc).count == 0u32 {
        *output_gamma_lut = build_linear_table(4096i32);
        *output_gamma_lut_length = 4096u64
    } else if (*trc).count == 1u32 {
        let mut gamma: f32 =
            (1.0f64 /
                 u8Fixed8Number_to_float(*(*trc).data.as_mut_ptr().offset(0isize))
                     as f64) as f32;
        *output_gamma_lut = build_pow_table(gamma, 4096i32);
        *output_gamma_lut_length = 4096u64
    } else {
        //XXX: the choice of a minimum of 256 here is not backed by any theory, 
                        //     measurement or data, however it is what lcms uses.
        *output_gamma_lut_length = (*trc).count as size_t;
        if *output_gamma_lut_length < 256u64 {
            *output_gamma_lut_length = 256u64
        }
        *output_gamma_lut =
            invert_lut((*trc).data.as_mut_ptr(), (*trc).count as i32,
                       *output_gamma_lut_length as i32)
    };
}
