use ::libc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrix {
    pub m: [[f32; 3]; 3],
    pub invalid: bool,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vector {
    pub v: [f32; 3],
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
#[no_mangle]
pub unsafe extern "C" fn matrix_eval(mut mat: matrix, mut v: vector) -> vector {
    let mut result: vector = vector { v: [0.; 3] };
    result.v[0usize] = mat.m[0usize][0usize] * v.v[0usize]
        + mat.m[0usize][1usize] * v.v[1usize]
        + mat.m[0usize][2usize] * v.v[2usize];
    result.v[1usize] = mat.m[1usize][0usize] * v.v[0usize]
        + mat.m[1usize][1usize] * v.v[1usize]
        + mat.m[1usize][2usize] * v.v[2usize];
    result.v[2usize] = mat.m[2usize][0usize] * v.v[0usize]
        + mat.m[2usize][1usize] * v.v[1usize]
        + mat.m[2usize][2usize] * v.v[2usize];
    return result;
}
//XXX: should probably pass by reference and we could
//probably reuse this computation in matrix_invert
#[no_mangle]
pub unsafe extern "C" fn matrix_det(mut mat: matrix) -> f32 {
    let mut det: f32 = mat.m[0usize][0usize] * mat.m[1usize][1usize] * mat.m[2usize][2usize]
        + mat.m[0usize][1usize] * mat.m[1usize][2usize] * mat.m[2usize][0usize]
        + mat.m[0usize][2usize] * mat.m[1usize][0usize] * mat.m[2usize][1usize]
        - mat.m[0usize][0usize] * mat.m[1usize][2usize] * mat.m[2usize][1usize]
        - mat.m[0usize][1usize] * mat.m[1usize][0usize] * mat.m[2usize][2usize]
        - mat.m[0usize][2usize] * mat.m[1usize][1usize] * mat.m[2usize][0usize];
    return det;
}
/* from pixman and cairo and Mathematics for Game Programmers */
/* lcms uses gauss-jordan elimination with partial pivoting which is
 * less efficient and not as numerically stable. See Mathematics for
 * Game Programmers. */
#[no_mangle]
pub unsafe extern "C" fn matrix_invert(mut mat: matrix) -> matrix {
    let mut dest_mat: matrix = matrix {
        m: [[0.; 3]; 3],
        invalid: false,
    };
    let mut i: i32 = 0;

    static mut a: [i32; 3] = [2i32, 2i32, 1i32];
    static mut b: [i32; 3] = [1i32, 0i32, 0i32];
    /* inv  (A) = 1/det (A) * adj (A) */
    let mut det: f32 = matrix_det(mat);
    if det == 0f32 {
        dest_mat.invalid = 1i32 != 0;
        return dest_mat;
    }
    dest_mat.invalid = 0i32 != 0;
    det = 1f32 / det;
    let mut j: i32 = 0i32;
    while j < 3i32 {
        i = 0i32;
        while i < 3i32 {
            let mut ai: i32 = a[i as usize];
            let mut aj: i32 = a[j as usize];
            let mut bi: i32 = b[i as usize];
            let mut bj: i32 = b[j as usize];
            let mut p: f64 = (mat.m[ai as usize][aj as usize] * mat.m[bi as usize][bj as usize]
                - mat.m[ai as usize][bj as usize] * mat.m[bi as usize][aj as usize])
                as f64;
            if i + j & 1i32 != 0i32 {
                p = -p
            }
            dest_mat.m[j as usize][i as usize] = (det as f64 * p) as f32;
            i += 1
        }
        j += 1
    }
    return dest_mat;
}
#[no_mangle]
pub unsafe extern "C" fn matrix_identity() -> matrix {
    let mut i: matrix = matrix {
        m: [[0.; 3]; 3],
        invalid: false,
    };
    i.m[0usize][0usize] = 1f32;
    i.m[0usize][1usize] = 0f32;
    i.m[0usize][2usize] = 0f32;
    i.m[1usize][0usize] = 0f32;
    i.m[1usize][1usize] = 1f32;
    i.m[1usize][2usize] = 0f32;
    i.m[2usize][0usize] = 0f32;
    i.m[2usize][1usize] = 0f32;
    i.m[2usize][2usize] = 1f32;
    i.invalid = 0i32 != 0;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn matrix_invalid() -> matrix {
    let mut inv: matrix = matrix_identity();
    inv.invalid = 1i32 != 0;
    return inv;
}
/* from pixman */
/* MAT3per... */
#[no_mangle]
pub unsafe extern "C" fn matrix_multiply(mut a: matrix, mut b: matrix) -> matrix {
    let mut result: matrix = matrix {
        m: [[0.; 3]; 3],
        invalid: false,
    };
    let mut dx: i32 = 0;

    let mut o: i32 = 0;
    let mut dy: i32 = 0i32;
    while dy < 3i32 {
        dx = 0i32;
        while dx < 3i32 {
            let mut v: f64 = 0f64;
            o = 0i32;
            while o < 3i32 {
                v += (a.m[dy as usize][o as usize] * b.m[o as usize][dx as usize]) as f64;
                o += 1
            }
            result.m[dy as usize][dx as usize] = v as f32;
            dx += 1
        }
        dy += 1
    }
    result.invalid = a.invalid as i32 != 0 || b.invalid as i32 != 0;
    return result;
}
