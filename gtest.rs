use crate::transform_util::lut_inverse_interp16;

#[test]
fn test_lut_inverse() {
    // mimic sRGB_v4_ICC mBA Output
    //
    //       XXXX
    //      X
    //     X
    // XXXX
    let mut value: u16 = 0;
    let mut lutTable: [u16; 256] = [0; 256];

    for i in 0..20 {
      lutTable[i] = 0;
    }
  
    for i in 20..200 {
      lutTable[i] = ((i - 20) * 0xFFFF / (200 - 20)) as u16;
    }
  
    for i in 200..lutTable.len() {
      lutTable[i] = 0xFFFF;
    }
  
    for i in 0..65535 {
      unsafe { lut_inverse_interp16(i, lutTable.as_mut_ptr(), lutTable.len() as i32) };
    }
  
    // Lookup the interesting points
  
    value =
        unsafe { lut_inverse_interp16(0, lutTable.as_mut_ptr(), lutTable.len() as i32) };
    assert!(value <= 20 * 256);
  
    value =
        unsafe { lut_inverse_interp16(1, lutTable.as_mut_ptr(), lutTable.len() as i32) };
    assert!(value > 20 * 256);
  
    value = unsafe { lut_inverse_interp16(65535, lutTable.as_mut_ptr(),
                                 lutTable.len() as i32) };
    assert!(value < 201 * 256);
}
