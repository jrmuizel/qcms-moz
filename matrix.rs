use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matrix {
    pub m: [[libc::c_float; 3]; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub v: [libc::c_float; 3],
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
pub unsafe extern "C" fn matrix_eval(mut mat: matrix, mut v: vector)
 -> vector {
    let mut result: vector = vector{v: [0.; 3],};
    result.v[0 as libc::c_int as usize] =
        mat.m[0 as libc::c_int as usize][0 as libc::c_int as usize] *
            v.v[0 as libc::c_int as usize] +
            mat.m[0 as libc::c_int as usize][1 as libc::c_int as usize] *
                v.v[1 as libc::c_int as usize] +
            mat.m[0 as libc::c_int as usize][2 as libc::c_int as usize] *
                v.v[2 as libc::c_int as usize];
    result.v[1 as libc::c_int as usize] =
        mat.m[1 as libc::c_int as usize][0 as libc::c_int as usize] *
            v.v[0 as libc::c_int as usize] +
            mat.m[1 as libc::c_int as usize][1 as libc::c_int as usize] *
                v.v[1 as libc::c_int as usize] +
            mat.m[1 as libc::c_int as usize][2 as libc::c_int as usize] *
                v.v[2 as libc::c_int as usize];
    result.v[2 as libc::c_int as usize] =
        mat.m[2 as libc::c_int as usize][0 as libc::c_int as usize] *
            v.v[0 as libc::c_int as usize] +
            mat.m[2 as libc::c_int as usize][1 as libc::c_int as usize] *
                v.v[1 as libc::c_int as usize] +
            mat.m[2 as libc::c_int as usize][2 as libc::c_int as usize] *
                v.v[2 as libc::c_int as usize];
    return result;
}
//XXX: should probably pass by reference and we could
//probably reuse this computation in matrix_invert
#[no_mangle]
pub unsafe extern "C" fn matrix_det(mut mat: matrix) -> libc::c_float {
    let mut det: libc::c_float = 0.;
    det =
        mat.m[0 as libc::c_int as usize][0 as libc::c_int as usize] *
            mat.m[1 as libc::c_int as usize][1 as libc::c_int as usize] *
            mat.m[2 as libc::c_int as usize][2 as libc::c_int as usize] +
            mat.m[0 as libc::c_int as usize][1 as libc::c_int as usize] *
                mat.m[1 as libc::c_int as usize][2 as libc::c_int as usize] *
                mat.m[2 as libc::c_int as usize][0 as libc::c_int as usize] +
            mat.m[0 as libc::c_int as usize][2 as libc::c_int as usize] *
                mat.m[1 as libc::c_int as usize][0 as libc::c_int as usize] *
                mat.m[2 as libc::c_int as usize][1 as libc::c_int as usize] -
            mat.m[0 as libc::c_int as usize][0 as libc::c_int as usize] *
                mat.m[1 as libc::c_int as usize][2 as libc::c_int as usize] *
                mat.m[2 as libc::c_int as usize][1 as libc::c_int as usize] -
            mat.m[0 as libc::c_int as usize][1 as libc::c_int as usize] *
                mat.m[1 as libc::c_int as usize][0 as libc::c_int as usize] *
                mat.m[2 as libc::c_int as usize][2 as libc::c_int as usize] -
            mat.m[0 as libc::c_int as usize][2 as libc::c_int as usize] *
                mat.m[1 as libc::c_int as usize][1 as libc::c_int as usize] *
                mat.m[2 as libc::c_int as usize][0 as libc::c_int as usize];
    return det;
}
/* from pixman and cairo and Mathematics for Game Programmers */
/* lcms uses gauss-jordan elimination with partial pivoting which is
 * less efficient and not as numerically stable. See Mathematics for
 * Game Programmers. */
#[no_mangle]
pub unsafe extern "C" fn matrix_invert(mut mat: matrix) -> matrix {
    let mut dest_mat: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    static mut a: [libc::c_int; 3] =
        [2 as libc::c_int, 2 as libc::c_int, 1 as libc::c_int];
    static mut b: [libc::c_int; 3] =
        [1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
    /* inv  (A) = 1/det (A) * adj (A) */
    let mut det: libc::c_float = matrix_det(mat);
    if det == 0 as libc::c_int as libc::c_float {
        dest_mat.invalid = 1 as libc::c_int != 0;
        return dest_mat
    }
    dest_mat.invalid = 0 as libc::c_int != 0;
    det = 1 as libc::c_int as libc::c_float / det;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            let mut p: libc::c_double = 0.;
            let mut ai: libc::c_int = a[i as usize];
            let mut aj: libc::c_int = a[j as usize];
            let mut bi: libc::c_int = b[i as usize];
            let mut bj: libc::c_int = b[j as usize];
            p =
                (mat.m[ai as usize][aj as usize] *
                     mat.m[bi as usize][bj as usize] -
                     mat.m[ai as usize][bj as usize] *
                         mat.m[bi as usize][aj as usize]) as libc::c_double;
            if i + j & 1 as libc::c_int != 0 as libc::c_int { p = -p }
            dest_mat.m[j as usize][i as usize] =
                (det as libc::c_double * p) as libc::c_float;
            i += 1
        }
        j += 1
    }
    return dest_mat;
}
#[no_mangle]
pub unsafe extern "C" fn matrix_identity() -> matrix {
    let mut i: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    i.m[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        1 as libc::c_int as libc::c_float;
    i.m[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    i.m[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    i.m[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    i.m[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        1 as libc::c_int as libc::c_float;
    i.m[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    i.m[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    i.m[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    i.m[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        1 as libc::c_int as libc::c_float;
    i.invalid = 0 as libc::c_int != 0;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn matrix_invalid() -> matrix {
    let mut inv: matrix = matrix_identity();
    inv.invalid = 1 as libc::c_int != 0;
    return inv;
}
/* from pixman */
/* MAT3per... */
#[no_mangle]
pub unsafe extern "C" fn matrix_multiply(mut a: matrix, mut b: matrix)
 -> matrix {
    let mut result: matrix = matrix{m: [[0.; 3]; 3], invalid: false,};
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    dy = 0 as libc::c_int;
    while dy < 3 as libc::c_int {
        dx = 0 as libc::c_int;
        while dx < 3 as libc::c_int {
            let mut v: libc::c_double = 0 as libc::c_int as libc::c_double;
            o = 0 as libc::c_int;
            while o < 3 as libc::c_int {
                v +=
                    (a.m[dy as usize][o as usize] *
                         b.m[o as usize][dx as usize]) as libc::c_double;
                o += 1
            }
            result.m[dy as usize][dx as usize] = v as libc::c_float;
            dx += 1
        }
        dy += 1
    }
    result.invalid =
        a.invalid as libc::c_int != 0 || b.invalid as libc::c_int != 0;
    return result;
}
