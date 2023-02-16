use crate::c::super_matrix::{c_SuperMatrix, Dtype_t, Mtype_t, Stype_t};
use libc;
use std::mem::MaybeUninit;

#[link(name = "superlu")]
extern "C" {
    fn dCreate_Dense_Matrix(
        X: *mut c_SuperMatrix,
        m: libc::c_int,
        n: libc::c_int,
        x: *mut libc::c_double,
        ldx: libc::c_int,
        stype: Stype_t,
        dtype: Dtype_t,
        mtype: Mtype_t,
    );
    fn sCreate_Dense_Matrix(
        X: *mut c_SuperMatrix,
        m: libc::c_int,
        n: libc::c_int,
        x: *mut libc::c_float,
        ldx: libc::c_int,
        stype: Stype_t,
        dtype: Dtype_t,
        mtype: Mtype_t,
    );
    fn Destroy_Dense_Matrix(A: *mut c_SuperMatrix);
    fn dPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn sPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
}

/// Create dense matrix of particular precision
///
/// Trait for access to low level C functions from SuperLU, which
/// dispatches correctly based on the desired precision (and picks
/// the right value for the Dtype argument).
///
pub trait CDenseMatrix {
    type Value;
    fn c_create_dense_matrix(
        x: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        values: &mut Vec<Self::Value>,
        ldx: i32,
        mtype: Mtype_t,
    );
    fn c_print_dense_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix);
}

impl CDenseMatrix for f64 {
    type Value = f64;
    fn c_create_dense_matrix(
        x: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        values: &mut Vec<Self::Value>,
        ldx: i32,
        mtype: Mtype_t,
    ) {
        unsafe {
            dCreate_Dense_Matrix(
                x.as_mut_ptr(),
                m,
                n,
                values.as_mut_ptr(),
                ldx,
                Stype_t::SLU_DN,
                Dtype_t::SLU_D,
                mtype,
            );
        }
    }

    fn c_print_dense_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix) {
        unsafe {
            dPrint_Dense_Matrix(what, a);
        }
    }
}

impl CDenseMatrix for f32 {
    type Value = f32;
    fn c_create_dense_matrix(
        x: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        values: &mut Vec<Self::Value>,
        ldx: i32,
        mtype: Mtype_t,
    ) {
        unsafe {
            sCreate_Dense_Matrix(
                x.as_mut_ptr(),
                m,
                n,
                values.as_mut_ptr(),
                ldx,
                Stype_t::SLU_DN,
                Dtype_t::SLU_D,
                mtype,
            );
        }
    }

    fn c_print_dense_matrix(what: *mut libc::c_char, a: *mut c_SuperMatrix) {
        unsafe {
            sPrint_Dense_Matrix(what, a);
        }
    }
}

/// This will attempt to deallocate the three input vectors used to
/// create the comp_col matrix.
#[allow(non_snake_case)]
pub fn c_Destroy_Dense_Matrix(A: *mut c_SuperMatrix) {
    unsafe {
        Destroy_Dense_Matrix(A);
    }
}
