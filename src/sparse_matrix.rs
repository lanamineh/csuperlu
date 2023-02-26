use itertools::Itertools;
use std::collections::HashMap;

use crate::comp_col::CompColMatrix;

#[derive(Debug)]
pub struct SparseMatrix {
    num_rows: usize,
    num_cols: usize,
    // TODO: Handle f32 and complex numbers
    values: HashMap<(usize, usize), f64>,
}

impl SparseMatrix {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        Self {
            num_rows,
            num_cols,
            values: HashMap::new(),
        }
    }

    // TODO: Can we overload something to make the input nicer, e.g a[row, col] = value
    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        if row >= self.num_rows || col >= self.num_cols {
            panic!("Index out of range");
        }
	// TODO: Do not insert into map when value = 0?
        self.values.insert((row, col), value);
    }

    pub fn get_value(&self, row: usize, col: usize) -> f64 {
        if row >= self.num_rows || col >= self.num_cols {
            panic!("Index out of range");
        }
        self.values.get(&(row, col)).copied().unwrap_or(0.0)
    }

    pub fn compressed_column_format(&self) -> CompColMatrix<f64> {
	// Sort in column order
	let sorted_keys = self.values.keys()
	    .sorted_unstable_by_key(|a| (a.1, a.0)); 

	let num_non_zeros = self.values.len();
	let mut non_zero_values = Vec::<f64>::with_capacity(num_non_zeros);
	let mut column_offsets = Vec::<i32>::with_capacity(self.num_cols + 1);
	let mut row_indices = Vec::<i32>::with_capacity(num_non_zeros);

	column_offsets.push(0);
	let mut current_col = 0usize;
	
	for key in sorted_keys {
	    if key.1 > current_col {
		// TODO: handle empty columns 
		column_offsets.push(non_zero_values.len() as i32);
		current_col = key.1;
	    }
	    non_zero_values.push(self.values[key]);
	    row_indices.push(key.0 as i32);
	}
	column_offsets.push(num_non_zeros as i32);

	CompColMatrix::from_vectors(self.num_rows, non_zero_values, row_indices, column_offsets)
    }

    pub fn print(&self) {
	println!("{} x {} matrix, {} non-zero values", self.num_rows, self.num_cols, self.values.len());
	let sorted_keys = self.values.keys()
	    .sorted_unstable_by_key(|a| (a.1, a.0)); 
	for key in sorted_keys {
	    println!("({}, {}) = {}", key.0, key.1, self.values[key]);
	}
    }
}