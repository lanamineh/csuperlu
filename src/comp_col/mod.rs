//! Functions to create matrices in compressed column format.
//!
//! Compressed-column matrices are very important in SuperLU, because
//! the main solver routines assume that the imput matrix $A$ is in
//! column-major format.
//!
//! A compressed-column matrix stored stores a sparse matrix in
//! column-major order, but only stores the non-zero elements in
//! each column. In order to identify which elements in the column
//! are non-zero, a vector of integers is maintained which stores
//! the row indices of the elements in the column. Arrays like this
//! are stored one after the other, one for each column in the matrix.
//! Since each column may be a different length, a third vector of
//! integers is maintained showing where each new column starts.

use crate::c::comp_col::{
    c_Destroy_CompCol_Matrix, CCreateCompColMatrix
};
use crate::c::super_matrix::{c_SuperMatrix, Mtype_t, Stype_t};
use crate::super_matrix::{SuperMatrix};
use std::mem::MaybeUninit;

/// Compressed-column matrix
///
///
pub struct CompColMatrix<P: CCreateCompColMatrix<P>> {
    nzval: Vec<P>,
    rowind: Vec<i32>,
    colptr: Vec<i32>,
    c_super_matrix: c_SuperMatrix,
}

impl<P: CCreateCompColMatrix<P>> CompColMatrix<P> {
    /// Specify a compressed column matrix from input vectors.
    ///
    /// Use this function to make a c_SuperMatrix in compressed column
    /// format, from the vector of values, row indices, and column
    /// offsets. Compressed column format is documented in Section
    /// 2.3 of the SuperLU manual.
    ///
    pub fn new(
        m: i32,
        n: i32,
        nnz: i32,
        mut nzval: Vec<P>,
        mut rowind: Vec<i32>,
        mut colptr: Vec<i32>,
        stype: Stype_t,
        mtype: Mtype_t,
    ) -> Self {
        let c_super_matrix = unsafe {
            let mut c_super_matrix = MaybeUninit::<c_SuperMatrix>::uninit();
	    P::c_create_comp_col_matrix(
		&mut c_super_matrix,
		m,
		n,
		nnz,
		&mut nzval,
		&mut rowind,
		&mut colptr,
		stype,
		mtype,
            );
            c_super_matrix.assume_init()
        };

        // When the CompCol matrix is created, the vectors a, asub and xa are
        // considered to be owned by the matrix. This means that the matrix free
        // function also frees these vectors. In order to avoid rust also freeing
        // them, forget them here.
        // std::mem::forget(nzval);
        // std::mem::forget(rowind);
        // std::mem::forget(colptr);

        Self {
            nzval,
            rowind,
            colptr,
            c_super_matrix,
        }
    }
}

impl<P: CCreateCompColMatrix<P>> SuperMatrix for CompColMatrix<P> {
    fn super_matrix<'a>(&'a mut self) -> &'a mut c_SuperMatrix {
        &mut self.c_super_matrix
    }
    fn print(&mut self, what: &str) {
	let c_str = std::ffi::CString::new(what).unwrap();
	P::c_print_comp_col_matrix(c_str.as_ptr() as *mut libc::c_char,
				   self.super_matrix());
    }
}

impl<P: CCreateCompColMatrix<P>> Drop for CompColMatrix<P> {
    fn drop(&mut self) {
        c_Destroy_CompCol_Matrix(&mut self.c_super_matrix);
    }
}

// #[allow(non_snake_case)]
// pub fn dPrint_CompCol_Matrix(what: &str, A: &mut c_SuperMatrix) {
//     let c_str = std::ffi::CString::new(what).unwrap();
//     c_dPrint_CompCol_Matrix(c_str.as_ptr() as *mut libc::c_char, A);
// }
