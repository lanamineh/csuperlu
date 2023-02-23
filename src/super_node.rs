//! Functions to create matrices in super-node format
//!

use crate::free::c_destroy_super_node_matrix;
use crate::super_matrix::SuperMatrix;
use crate::value_type::ValueType;
use csuperlu_sys::super_matrix::{c_SCformat, c_SuperMatrix};
use csuperlu_sys::super_node::c_Destroy_SuperNode_Matrix;

/// Super-node matrix
///
pub struct SuperNodeMatrix<P: ValueType<P>> {
    c_super_matrix: c_SuperMatrix,
    marker: std::marker::PhantomData<P>,
}

impl<P: ValueType<P>> SuperNodeMatrix<P> {
    /// Create a super-node matrix from a c_SuperMatrix structure
    ///
    pub fn from_super_matrix(c_super_matrix: c_SuperMatrix) -> Self {
        Self {
            c_super_matrix,
            marker: std::marker::PhantomData,
        }
    }
    pub fn values(&mut self) -> &[P] {
        unsafe {
            let c_scformat = &mut *(self.c_super_matrix.Store as *mut c_SCformat);
            std::slice::from_raw_parts(c_scformat.nzval as *mut P, c_scformat.nnz as usize)
        }
    }
}

impl<P: ValueType<P>> SuperMatrix for SuperNodeMatrix<P> {
    fn super_matrix<'a>(&'a self) -> &'a c_SuperMatrix {
        &self.c_super_matrix
    }
    fn print(&self, what: &str) {
        unsafe {
            P::c_print_super_node_matrix(what, &self.c_super_matrix);
        }
    }
}

impl<P: ValueType<P>> Drop for SuperNodeMatrix<P> {
    fn drop(&mut self) {
	unsafe {
            c_destroy_super_node_matrix(&mut self.c_super_matrix);
	}
    }
}
