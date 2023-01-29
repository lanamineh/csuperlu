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
    fn Destroy_SuperMatrix_Store(A: *mut c_SuperMatrix);
    fn dPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
    fn sPrint_Dense_Matrix(what: *mut libc::c_char, A: *mut c_SuperMatrix);
}

/// Create dense matrix of particular precision
///
/// Trait for access to low level C functions from SuperLU, which
/// dispatches correctly based on the desired precision (and picks
/// the right value for the Dtype argument).
///
pub trait CCreateDenseMatrix<P> {
    fn c_create_dense_matrix(
        x: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        values: &mut Vec<P>,
        ldx: i32,
        mtype: Mtype_t,	
    );
    fn c_print_dense_matrix(
    	what: *mut libc::c_char,
	a: *mut c_SuperMatrix,
    );
}

impl CCreateDenseMatrix<f64> for f64 {
    fn c_create_dense_matrix(
        x: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        values: &mut Vec<f64>,
        ldx: i32,
        mtype: Mtype_t,	
    ) {
	unsafe {
	    dCreate_Dense_Matrix(x.as_mut_ptr(), m, n,
				 values.as_mut_ptr(), ldx,
				 Stype_t::SLU_DN,
				 Dtype_t::SLU_D, mtype);
	}
    }
    
    fn c_print_dense_matrix(
    	what: *mut libc::c_char,
	a: *mut c_SuperMatrix,
    ) {
	unsafe {
            dPrint_Dense_Matrix(what, a);
	}	
    }
}

impl CCreateDenseMatrix<f32> for f32 {
    fn c_create_dense_matrix(
        x: &mut MaybeUninit<c_SuperMatrix>,
        m: i32,
        n: i32,
        values: &mut Vec<f32>,
        ldx: i32,
        mtype: Mtype_t,	
    ) {
	unsafe {
	    sCreate_Dense_Matrix(x.as_mut_ptr(), m, n,
				 values.as_mut_ptr(), ldx,
				 Stype_t::SLU_DN,
				 Dtype_t::SLU_D, mtype);
	}
    }
    
    fn c_print_dense_matrix(
    	what: *mut libc::c_char,
	a: *mut c_SuperMatrix,
    ) {
	unsafe {
            sPrint_Dense_Matrix(what, a);
	}	
    }
}




// This will deallocate only the data structure allocated by
// the Create_*_Matrix routine (leaving the input vectors to
// be freed by the caller).
#[allow(non_snake_case)]
pub fn c_Destroy_SuperMatrix_Store(A: *mut c_SuperMatrix) {
    unsafe {
        Destroy_SuperMatrix_Store(A);
    }
}
