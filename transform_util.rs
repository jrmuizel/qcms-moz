use ::libc;
extern "C" {
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceilf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floorf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
}
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type int32_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct precache_output {
    pub ref_count: libc::c_int,
    pub data: [uint8_t; 8192],
}
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
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
pub type uint32_t = libc::c_uint;
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
pub type uint16_fract_t = uint16_t;
#[inline]
unsafe extern "C" fn s15Fixed16Number_to_float(mut a: s15Fixed16Number)
 -> libc::c_float {
    return a as libc::c_float / 65536.0f32;
}
#[inline]
unsafe extern "C" fn u8Fixed8Number_to_float(mut x: uint16_t)
 -> libc::c_float {
    return (x as libc::c_int as libc::c_double / 256.0f64) as libc::c_float;
}
#[inline]
unsafe extern "C" fn clamp_float(mut a: libc::c_float) -> libc::c_float {
    if a as libc::c_double > 1.0f64 {
        return 1.0f64 as libc::c_float
    } else if a >= 0 as libc::c_int as libc::c_float {
        return a
    } else { return 0 as libc::c_int as libc::c_float };
}
//'para'
/* value must be a value between 0 and 1 */
//XXX: is the above a good restriction to have?
// the output range of this functions is 0..1
#[no_mangle]
pub unsafe extern "C" fn lut_interp_linear(mut input_value: libc::c_double,
                                           mut table: *mut uint16_t,
                                           mut length: libc::c_int)
 -> libc::c_float {
    let mut upper: libc::c_int = 0; // scale to length of the array
    let mut lower: libc::c_int = 0;
    let mut value: libc::c_float = 0.;
    input_value = input_value * (length - 1 as libc::c_int) as libc::c_double;
    upper = ceil(input_value) as libc::c_int;
    lower = floor(input_value) as libc::c_int;
    //XXX: can we be more performant here?
    value =
        (*table.offset(upper as isize) as libc::c_int as libc::c_double *
             (1.0f64 - (upper as libc::c_double - input_value)) +
             *table.offset(lower as isize) as libc::c_int as libc::c_double *
                 (upper as libc::c_double - input_value)) as libc::c_float;
    /* scale the value */
    return value * (1.0f32 / 65535.0f32);
}
/* same as above but takes and returns a uint16_t value representing a range from 0..1 */
#[no_mangle]
pub unsafe extern "C" fn lut_interp_linear16(mut input_value: uint16_t,
                                             mut table: *mut uint16_t,
                                             mut length: libc::c_int)
 -> uint16_t {
    /* Start scaling input_value to the length of the array: 65535*(length-1).
	 * We'll divide out the 65535 next */
    let mut value: uint32_t =
        (input_value as libc::c_int * (length - 1 as libc::c_int)) as
            uint32_t; /* equivalent to ceil(value/65535) */
    let mut upper: uint32_t =
        value.wrapping_add(65534 as libc::c_int as
                               libc::c_uint).wrapping_div(65535 as libc::c_int
                                                              as
                                                              libc::c_uint); /* equivalent to floor(value/65535) */
    let mut lower: uint32_t =
        value.wrapping_div(65535 as libc::c_int as libc::c_uint);
    /* interp is the distance from upper to value scaled to 0..65535 */
    let mut interp: uint32_t =
        value.wrapping_rem(65535 as libc::c_int as
                               libc::c_uint); // 0..65535*65535
    value =
        (*table.offset(upper as isize) as
             libc::c_uint).wrapping_mul(interp).wrapping_add((*table.offset(lower
                                                                                as
                                                                                isize)
                                                                  as
                                                                  libc::c_uint).wrapping_mul((65535
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint).wrapping_sub(interp))).wrapping_div(65535
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint);
    return value as uint16_t;
}
/* same as above but takes an input_value from 0..PRECACHE_OUTPUT_MAX
 * and returns a uint8_t value representing a range from 0..1 */
unsafe extern "C" fn lut_interp_linear_precache_output(mut input_value:
                                                           uint32_t,
                                                       mut table:
                                                           *mut uint16_t,
                                                       mut length:
                                                           libc::c_int)
 -> uint8_t {
    /* Start scaling input_value to the length of the array: PRECACHE_OUTPUT_MAX*(length-1).
	 * We'll divide out the PRECACHE_OUTPUT_MAX next */
    let mut value: uint32_t =
        input_value.wrapping_mul((length - 1 as libc::c_int) as libc::c_uint);
    /* equivalent to ceil(value/PRECACHE_OUTPUT_MAX) */
    let mut upper: uint32_t =
        value.wrapping_add((8192 as libc::c_int - 1 as libc::c_int) as
                               libc::c_uint).wrapping_sub(1 as libc::c_int as
                                                              libc::c_uint).wrapping_div((8192
                                                                                              as
                                                                                              libc::c_int
                                                                                              -
                                                                                              1
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                             as
                                                                                             libc::c_uint);
    /* equivalent to floor(value/PRECACHE_OUTPUT_MAX) */
    let mut lower: uint32_t =
        value.wrapping_div((8192 as libc::c_int - 1 as libc::c_int) as
                               libc::c_uint);
    /* interp is the distance from upper to value scaled to 0..PRECACHE_OUTPUT_MAX */
    let mut interp: uint32_t =
        value.wrapping_rem((8192 as libc::c_int - 1 as libc::c_int) as
                               libc::c_uint);
    /* the table values range from 0..65535 */
    value =
        (*table.offset(upper as isize) as
             libc::c_uint).wrapping_mul(interp).wrapping_add((*table.offset(lower
                                                                                as
                                                                                isize)
                                                                  as
                                                                  libc::c_uint).wrapping_mul(((8192
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   -
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int)
                                                                                                  as
                                                                                                  libc::c_uint).wrapping_sub(interp))); // 0..(65535*PRECACHE_OUTPUT_MAX)
    /* round and scale */
    value =
        (value as
             libc::c_uint).wrapping_add(((8192 as libc::c_int -
                                              1 as libc::c_int) *
                                             65535 as libc::c_int /
                                             255 as libc::c_int /
                                             2 as libc::c_int) as
                                            libc::c_uint) as uint32_t as
            uint32_t; // scale to 0..255
    value =
        (value as
             libc::c_uint).wrapping_div(((8192 as libc::c_int -
                                              1 as libc::c_int) *
                                             65535 as libc::c_int /
                                             255 as libc::c_int) as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    return value as uint8_t;
}
/* value must be a value between 0 and 1 */
//XXX: is the above a good restriction to have?
#[no_mangle]
pub unsafe extern "C" fn lut_interp_linear_float(mut value: libc::c_float,
                                                 mut table:
                                                     *mut libc::c_float,
                                                 mut length: libc::c_int)
 -> libc::c_float {
    let mut upper: libc::c_int = 0;
    let mut lower: libc::c_int = 0;
    value = value * (length - 1 as libc::c_int) as libc::c_float;
    upper = ceilf(value) as libc::c_int;
    lower = floorf(value) as libc::c_int;
    //XXX: can we be more performant here?
    value =
        (*table.offset(upper as isize) as libc::c_double *
             (1.0f64 - (upper as libc::c_float - value) as libc::c_double) +
             (*table.offset(lower as isize) *
                  (upper as libc::c_float - value)) as libc::c_double) as
            libc::c_float;
    /* scale the value */
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn compute_curve_gamma_table_type1(mut gamma_table:
                                                             *mut libc::c_float,
                                                         mut gamma:
                                                             uint16_t) {
    let mut i: libc::c_uint = 0;
    let mut gamma_float: libc::c_float = u8Fixed8Number_to_float(gamma);
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        // 0..1^(0..255 + 255/256) will always be between 0 and 1
        *gamma_table.offset(i as isize) =
            pow(i as libc::c_double / 255.0f64, gamma_float as libc::c_double)
                as libc::c_float;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_curve_gamma_table_type2(mut gamma_table:
                                                             *mut libc::c_float,
                                                         mut table:
                                                             *mut uint16_t,
                                                         mut length:
                                                             libc::c_int) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        *gamma_table.offset(i as isize) =
            lut_interp_linear(i as libc::c_double / 255.0f64, table, length);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_curve_gamma_table_type_parametric(mut gamma_table:
                                                                       *mut libc::c_float,
                                                                   mut parameter:
                                                                       *mut libc::c_float,
                                                                   mut count:
                                                                       libc::c_int) {
    let mut X: size_t = 0;
    let mut interval: libc::c_float = 0.;
    let mut a: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut e: libc::c_float = 0.;
    let mut f: libc::c_float = 0.;
    let mut y: libc::c_float = *parameter.offset(0 as libc::c_int as isize);
    if count == 0 as libc::c_int {
        a = 1 as libc::c_int as libc::c_float;
        b = 0 as libc::c_int as libc::c_float;
        c = 0 as libc::c_int as libc::c_float;
        e = 0 as libc::c_int as libc::c_float;
        f = 0 as libc::c_int as libc::c_float;
        interval = -(1 as libc::c_int) as libc::c_float
    } else if count == 1 as libc::c_int {
        a = *parameter.offset(1 as libc::c_int as isize);
        b = *parameter.offset(2 as libc::c_int as isize);
        c = 0 as libc::c_int as libc::c_float;
        e = 0 as libc::c_int as libc::c_float;
        f = 0 as libc::c_int as libc::c_float;
        interval =
            -(1 as libc::c_int) as libc::c_float *
                *parameter.offset(2 as libc::c_int as isize) /
                *parameter.offset(1 as libc::c_int as isize)
    } else if count == 2 as libc::c_int {
        a = *parameter.offset(1 as libc::c_int as isize);
        b = *parameter.offset(2 as libc::c_int as isize);
        c = 0 as libc::c_int as libc::c_float;
        e = *parameter.offset(3 as libc::c_int as isize);
        f = *parameter.offset(3 as libc::c_int as isize);
        interval =
            -(1 as libc::c_int) as libc::c_float *
                *parameter.offset(2 as libc::c_int as isize) /
                *parameter.offset(1 as libc::c_int as isize)
    } else if count == 3 as libc::c_int {
        a = *parameter.offset(1 as libc::c_int as isize);
        b = *parameter.offset(2 as libc::c_int as isize);
        c = *parameter.offset(3 as libc::c_int as isize);
        e = -c;
        f = 0 as libc::c_int as libc::c_float;
        interval = *parameter.offset(4 as libc::c_int as isize)
    } else if count == 4 as libc::c_int {
        a = *parameter.offset(1 as libc::c_int as isize);
        b = *parameter.offset(2 as libc::c_int as isize);
        c = *parameter.offset(3 as libc::c_int as isize);
        e = *parameter.offset(5 as libc::c_int as isize) - c;
        f = *parameter.offset(6 as libc::c_int as isize);
        interval = *parameter.offset(4 as libc::c_int as isize)
    } else {
        if !(0 as libc::c_int != 0 &&
                 !(b"invalid parametric function type.\x00" as *const u8 as
                       *const libc::c_char).is_null()) as libc::c_int as
               libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 42],
                                                   &[libc::c_char; 42]>(b"compute_curve_gamma_table_type_parametric\x00")).as_ptr(),
                         b"transform_util.c\x00" as *const u8 as
                             *const libc::c_char, 158 as libc::c_int,
                         b"0 && \"invalid parametric function type.\"\x00" as
                             *const u8 as *const libc::c_char);
        } else { };
        a = 1 as libc::c_int as libc::c_float;
        b = 0 as libc::c_int as libc::c_float;
        c = 0 as libc::c_int as libc::c_float;
        e = 0 as libc::c_int as libc::c_float;
        f = 0 as libc::c_int as libc::c_float;
        interval = -(1 as libc::c_int) as libc::c_float
    }
    X = 0 as libc::c_int as size_t;
    while X < 256 as libc::c_int as libc::c_ulong {
        if X as libc::c_float >= interval {
            // XXX The equations are not exactly as defined in the spec but are
                        //     algebraically equivalent.
                        // TODO Should division by 255 be for the whole expression.
            *gamma_table.offset(X as isize) =
                clamp_float((pow((a * X as libc::c_float) as libc::c_double /
                                     255.0f64 + b as libc::c_double,
                                 y as libc::c_double) + c as libc::c_double +
                                 e as libc::c_double) as libc::c_float)
        } else {
            *gamma_table.offset(X as isize) =
                clamp_float(((c * X as libc::c_float) as libc::c_double /
                                 255.0f64 + f as libc::c_double) as
                                libc::c_float)
        }
        X = X.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_curve_gamma_table_type0(mut gamma_table:
                                                             *mut libc::c_float) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        *gamma_table.offset(i as isize) =
            (i as libc::c_double / 255.0f64) as libc::c_float;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn build_input_gamma_table(mut TRC: *mut curveType)
 -> *mut libc::c_float {
    let mut gamma_table: *mut libc::c_float = 0 as *mut libc::c_float;
    if TRC.is_null() { return 0 as *mut libc::c_float }
    gamma_table =
        malloc((::std::mem::size_of::<libc::c_float>() as
                    libc::c_ulong).wrapping_mul(256 as libc::c_int as
                                                    libc::c_ulong)) as
            *mut libc::c_float;
    if !gamma_table.is_null() {
        if (*TRC).type_0 == 0x70617261 as libc::c_int as libc::c_uint {
            compute_curve_gamma_table_type_parametric(gamma_table,
                                                      (*TRC).parameter.as_mut_ptr(),
                                                      (*TRC).count as
                                                          libc::c_int);
        } else if (*TRC).count == 0 as libc::c_int as libc::c_uint {
            compute_curve_gamma_table_type0(gamma_table);
        } else if (*TRC).count == 1 as libc::c_int as libc::c_uint {
            compute_curve_gamma_table_type1(gamma_table,
                                            *(*TRC).data.as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
        } else {
            compute_curve_gamma_table_type2(gamma_table,
                                            (*TRC).data.as_mut_ptr(),
                                            (*TRC).count as libc::c_int);
        }
    }
    return gamma_table;
}
#[no_mangle]
pub unsafe extern "C" fn build_colorant_matrix(mut p: *mut qcms_profile)
 -> matrix {
    let mut result: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    result.m[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        s15Fixed16Number_to_float((*p).redColorant.X);
    result.m[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        s15Fixed16Number_to_float((*p).greenColorant.X);
    result.m[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        s15Fixed16Number_to_float((*p).blueColorant.X);
    result.m[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        s15Fixed16Number_to_float((*p).redColorant.Y);
    result.m[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        s15Fixed16Number_to_float((*p).greenColorant.Y);
    result.m[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        s15Fixed16Number_to_float((*p).blueColorant.Y);
    result.m[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        s15Fixed16Number_to_float((*p).redColorant.Z);
    result.m[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        s15Fixed16Number_to_float((*p).greenColorant.Z);
    result.m[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        s15Fixed16Number_to_float((*p).blueColorant.Z);
    result.invalid = 0 as libc::c_int != 0;
    return result;
}
/* The following code is copied nearly directly from lcms.
 * I think it could be much better. For example, Argyll seems to have better code in
 * icmTable_lookup_bwd and icmTable_setup_bwd. However, for now this is a quick way
 * to a working solution and allows for easy comparing with lcms. */
#[no_mangle]
pub unsafe extern "C" fn lut_inverse_interp16(mut Value: uint16_t,
                                              mut LutTable: *mut uint16_t,
                                              mut length: libc::c_int)
 -> uint16_fract_t {
    let mut l: libc::c_int =
        1 as libc::c_int; // 'int' Give spacing for negative values
    let mut r: libc::c_int = 0x10000 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0;
    let mut NumZeroes: libc::c_int = 0;
    let mut NumPoles: libc::c_int = 0;
    let mut cell0: libc::c_int = 0;
    let mut cell1: libc::c_int = 0;
    let mut val2: libc::c_double = 0.;
    let mut y0: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut x0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    // July/27 2001 - Expanded to handle degenerated curves with an arbitrary
        // number of elements containing 0 at the begining of the table (Zeroes)
        // and another arbitrary number of poles (FFFFh) at the end.
        // First the zero and pole extents are computed, then value is compared.
    NumZeroes = 0 as libc::c_int;
    while *LutTable.offset(NumZeroes as isize) as libc::c_int ==
              0 as libc::c_int && NumZeroes < length - 1 as libc::c_int {
        NumZeroes += 1
    }
    // There are no zeros at the beginning and we are trying to find a zero, so
        // return anything. It seems zero would be the less destructive choice
	/* I'm not sure that this makes sense, but oh well... */
    if NumZeroes == 0 as libc::c_int &&
           Value as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as uint16_fract_t
    }
    NumPoles = 0 as libc::c_int;
    while *LutTable.offset((length - 1 as libc::c_int - NumPoles) as isize) as
              libc::c_int == 0xffff as libc::c_int &&
              NumPoles < length - 1 as libc::c_int {
        NumPoles += 1
    }
    // Does the curve belong to this case?
    if NumZeroes > 1 as libc::c_int || NumPoles > 1 as libc::c_int {
        let mut a_0: libc::c_int = 0;
        let mut b_0: libc::c_int = 0;
        // Identify if value fall downto 0 or FFFF zone
        if Value as libc::c_int == 0 as libc::c_int {
            return 0 as libc::c_int as uint16_fract_t
        }
        // if (Value == 0xFFFF) return 0xFFFF;
        // else restrict to valid zone
        if NumZeroes > 1 as libc::c_int {
            a_0 =
                (NumZeroes - 1 as libc::c_int) * 0xffff as libc::c_int /
                    (length - 1 as libc::c_int);
            l = a_0 - 1 as libc::c_int
        }
        if NumPoles > 1 as libc::c_int {
            b_0 =
                (length - 1 as libc::c_int - NumPoles) * 0xffff as libc::c_int
                    / (length - 1 as libc::c_int);
            r = b_0 + 1 as libc::c_int
        }
    }
    if r <= l {
        // If this happens LutTable is not invertible
        return 0 as libc::c_int as uint16_fract_t
    }
    // Seems not a degenerated case... apply binary search
    while r > l {
        x = (l + r) / 2 as libc::c_int;
        res =
            lut_interp_linear16((x - 1 as libc::c_int) as uint16_fract_t,
                                LutTable, length) as libc::c_int;
        if res == Value as libc::c_int {
            // Found exact match.
            return (x - 1 as libc::c_int) as uint16_fract_t
        }
        if res > Value as libc::c_int {
            r = x - 1 as libc::c_int
        } else { l = x + 1 as libc::c_int }
    }
    // Not found, should we interpolate?
    // Get surrounding nodes
    if !(x >= 1 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 21],
                                               &[libc::c_char; 21]>(b"lut_inverse_interp16\x00")).as_ptr(),
                     b"transform_util.c\x00" as *const u8 as
                         *const libc::c_char, 307 as libc::c_int,
                     b"x >= 1\x00" as *const u8 as *const libc::c_char);
    } else { };
    val2 =
        (length - 1 as libc::c_int) as libc::c_double *
            ((x - 1 as libc::c_int) as libc::c_double / 65535.0f64);
    cell0 = floor(val2) as libc::c_int;
    cell1 = ceil(val2) as libc::c_int;
    if cell0 == cell1 { return x as uint16_fract_t }
    y0 = *LutTable.offset(cell0 as isize) as libc::c_double;
    x0 =
        65535.0f64 * cell0 as libc::c_double /
            (length - 1 as libc::c_int) as libc::c_double;
    y1 = *LutTable.offset(cell1 as isize) as libc::c_double;
    x1 =
        65535.0f64 * cell1 as libc::c_double /
            (length - 1 as libc::c_int) as libc::c_double;
    a = (y1 - y0) / (x1 - x0);
    b = y0 - a * x0;
    if fabs(a) < 0.01f64 { return x as uint16_fract_t }
    f = (Value as libc::c_int as libc::c_double - b) / a;
    if f < 0.0f64 { return 0 as libc::c_int as uint16_fract_t }
    if f >= 65535.0f64 { return 0xffff as libc::c_int as uint16_fract_t }
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
                                mut length: libc::c_int,
                                mut out_length: libc::c_int)
 -> *mut uint16_t {
    let mut i: libc::c_int = 0;
    /* for now we invert the lut by creating a lut of size out_length
         * and attempting to lookup a value for each entry using lut_inverse_interp16 */
    let mut output: *mut uint16_t =
        malloc((::std::mem::size_of::<uint16_t>() as
                    libc::c_ulong).wrapping_mul(out_length as libc::c_ulong))
            as *mut uint16_t;
    if output.is_null() { return 0 as *mut uint16_t }
    i = 0 as libc::c_int;
    while i < out_length {
        let mut x: libc::c_double =
            i as libc::c_double * 65535.0f64 /
                (out_length - 1 as libc::c_int) as libc::c_double;
        let mut input: uint16_fract_t = floor(x + 0.5f64) as uint16_fract_t;
        *output.offset(i as isize) =
            lut_inverse_interp16(input, table, length);
        i += 1
    }
    return output;
}
unsafe extern "C" fn compute_precache_pow(mut output: *mut uint8_t,
                                          mut gamma: libc::c_float) {
    let mut v: uint32_t = 0 as libc::c_int as uint32_t;
    v = 0 as libc::c_int as uint32_t;
    while v < 8192 as libc::c_int as libc::c_uint {
        //XXX: don't do integer/float conversion... and round?
        *output.offset(v as isize) =
            (255.0f64 *
                 pow(v as libc::c_double /
                         (8192 as libc::c_int - 1 as libc::c_int) as
                             libc::c_double, gamma as libc::c_double)) as
                uint8_t;
        v = v.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_precache_lut(mut output: *mut uint8_t,
                                              mut table: *mut uint16_t,
                                              mut length: libc::c_int) {
    let mut v: uint32_t = 0 as libc::c_int as uint32_t;
    v = 0 as libc::c_int as uint32_t;
    while v < 8192 as libc::c_int as libc::c_uint {
        *output.offset(v as isize) =
            lut_interp_linear_precache_output(v, table, length);
        v = v.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_precache_linear(mut output: *mut uint8_t) {
    let mut v: uint32_t = 0 as libc::c_int as uint32_t;
    v = 0 as libc::c_int as uint32_t;
    while v < 8192 as libc::c_int as libc::c_uint {
        //XXX: round?
        *output.offset(v as isize) =
            v.wrapping_div((8192 as libc::c_int / 256 as libc::c_int) as
                               libc::c_uint) as uint8_t;
        v = v.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn compute_precache(mut trc: *mut curveType,
                                          mut output: *mut uint8_t) -> bool {
    if (*trc).type_0 == 0x70617261 as libc::c_int as libc::c_uint {
        let mut gamma_table: [libc::c_float; 256] = [0.; 256];
        let mut gamma_table_uint: [uint16_t; 256] = [0; 256];
        let mut i: uint16_t = 0;
        let mut inverted: *mut uint16_t = 0 as *mut uint16_t;
        let mut inverted_size: libc::c_int = 256 as libc::c_int;
        compute_curve_gamma_table_type_parametric(gamma_table.as_mut_ptr(),
                                                  (*trc).parameter.as_mut_ptr(),
                                                  (*trc).count as
                                                      libc::c_int);
        i = 0 as libc::c_int as uint16_t;
        while (i as libc::c_int) < 256 as libc::c_int {
            gamma_table_uint[i as usize] =
                (gamma_table[i as usize] *
                     65535 as libc::c_int as libc::c_float) as uint16_t;
            i = i.wrapping_add(1)
        }
        //XXX: the choice of a minimum of 256 here is not backed by any theory, 
                        //     measurement or data, howeve r it is what lcms uses.
                        //     the maximum number we would need is 65535 because that's the 
                        //     accuracy used for computing the pre cache table
        if inverted_size < 256 as libc::c_int {
            inverted_size = 256 as libc::c_int
        }
        inverted =
            invert_lut(gamma_table_uint.as_mut_ptr(), 256 as libc::c_int,
                       inverted_size);
        if inverted.is_null() { return 0 as libc::c_int != 0 }
        compute_precache_lut(output, inverted, inverted_size);
        free(inverted as *mut libc::c_void);
    } else if (*trc).count == 0 as libc::c_int as libc::c_uint {
        compute_precache_linear(output);
    } else if (*trc).count == 1 as libc::c_int as libc::c_uint {
        compute_precache_pow(output,
                             (1.0f64 /
                                  u8Fixed8Number_to_float(*(*trc).data.as_mut_ptr().offset(0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize))
                                      as libc::c_double) as libc::c_float);
    } else {
        let mut inverted_0: *mut uint16_t = 0 as *mut uint16_t;
        let mut inverted_size_0: libc::c_int = (*trc).count as libc::c_int;
        //XXX: the choice of a minimum of 256 here is not backed by any theory, 
                        //     measurement or data, howeve r it is what lcms uses.
                        //     the maximum number we would need is 65535 because that's the 
                        //     accuracy used for computing the pre cache table
        if inverted_size_0 < 256 as libc::c_int {
            inverted_size_0 = 256 as libc::c_int
        } //XXX turn this conversion into a function
        inverted_0 =
            invert_lut((*trc).data.as_mut_ptr(), (*trc).count as libc::c_int,
                       inverted_size_0);
        if inverted_0.is_null() { return 0 as libc::c_int != 0 }
        compute_precache_lut(output, inverted_0, inverted_size_0);
        free(inverted_0 as *mut libc::c_void);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn build_linear_table(mut length: libc::c_int)
 -> *mut uint16_t {
    let mut i: libc::c_int = 0;
    let mut output: *mut uint16_t =
        malloc((::std::mem::size_of::<uint16_t>() as
                    libc::c_ulong).wrapping_mul(length as libc::c_ulong)) as
            *mut uint16_t;
    if output.is_null() { return 0 as *mut uint16_t }
    i = 0 as libc::c_int;
    while i < length {
        let mut x: libc::c_double =
            i as libc::c_double * 65535.0f64 /
                (length - 1 as libc::c_int) as libc::c_double;
        let mut input: uint16_fract_t = floor(x + 0.5f64) as uint16_fract_t;
        *output.offset(i as isize) = input;
        i += 1
    }
    return output;
}
unsafe extern "C" fn build_pow_table(mut gamma: libc::c_float,
                                     mut length: libc::c_int)
 -> *mut uint16_t {
    let mut i: libc::c_int = 0;
    let mut output: *mut uint16_t =
        malloc((::std::mem::size_of::<uint16_t>() as
                    libc::c_ulong).wrapping_mul(length as libc::c_ulong)) as
            *mut uint16_t;
    if output.is_null() { return 0 as *mut uint16_t }
    i = 0 as libc::c_int;
    while i < length {
        let mut result: uint16_fract_t = 0;
        let mut x: libc::c_double =
            i as libc::c_double /
                (length - 1 as libc::c_int) as libc::c_double;
        x = pow(x, gamma as libc::c_double);
        result = floor(x * 65535.0f64 + 0.5f64) as uint16_fract_t;
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
    if (*trc).type_0 == 0x70617261 as libc::c_int as libc::c_uint {
        let mut gamma_table: [libc::c_float; 256] = [0.; 256];
        let mut i: uint16_t = 0;
        let mut output: *mut uint16_t =
            malloc((::std::mem::size_of::<uint16_t>() as
                        libc::c_ulong).wrapping_mul(256 as libc::c_int as
                                                        libc::c_ulong)) as
                *mut uint16_t;
        if output.is_null() { *output_gamma_lut = 0 as *mut uint16_t; return }
        compute_curve_gamma_table_type_parametric(gamma_table.as_mut_ptr(),
                                                  (*trc).parameter.as_mut_ptr(),
                                                  (*trc).count as
                                                      libc::c_int);
        *output_gamma_lut_length = 256 as libc::c_int as size_t;
        i = 0 as libc::c_int as uint16_t;
        while (i as libc::c_int) < 256 as libc::c_int {
            *output.offset(i as isize) =
                (gamma_table[i as usize] *
                     65535 as libc::c_int as libc::c_float) as uint16_t;
            i = i.wrapping_add(1)
        }
        *output_gamma_lut = output
    } else if (*trc).count == 0 as libc::c_int as libc::c_uint {
        *output_gamma_lut = build_linear_table(4096 as libc::c_int);
        *output_gamma_lut_length = 4096 as libc::c_int as size_t
    } else if (*trc).count == 1 as libc::c_int as libc::c_uint {
        let mut gamma: libc::c_float =
            (1.0f64 /
                 u8Fixed8Number_to_float(*(*trc).data.as_mut_ptr().offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize))
                     as libc::c_double) as libc::c_float;
        *output_gamma_lut = build_pow_table(gamma, 4096 as libc::c_int);
        *output_gamma_lut_length = 4096 as libc::c_int as size_t
    } else {
        //XXX: the choice of a minimum of 256 here is not backed by any theory, 
                        //     measurement or data, however it is what lcms uses.
        *output_gamma_lut_length = (*trc).count as size_t;
        if *output_gamma_lut_length < 256 as libc::c_int as libc::c_ulong {
            *output_gamma_lut_length = 256 as libc::c_int as size_t
        }
        *output_gamma_lut =
            invert_lut((*trc).data.as_mut_ptr(), (*trc).count as libc::c_int,
                       *output_gamma_lut_length as libc::c_int)
    };
}
