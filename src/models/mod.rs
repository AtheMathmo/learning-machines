//! This module contains the handlers for the various
//! rusty-machine models.

use iron::prelude::*;
use rustc_serialize::json::{Json, Object};

use rm::linalg::Matrix;

pub mod k_means;

pub trait ModelHandler: Send + Sync + 'static {
    fn handle(&self, input: Object) -> IronResult<Response>;
}

fn get_matrix_from_data(input: &Json) -> Result<Matrix<f64>, &'static str> {
    if input.is_array() {
        let arr = input.as_array().unwrap();
        let mut raw_data = Vec::new();
        let mut rows = 0;
        for row in arr {
            rows += 1;
            if row.is_array() {
                raw_data.extend(row.as_array()
                    .unwrap()
                    .iter()
                    .map(|ref x| x.as_f64().unwrap())
                    .collect::<Vec<f64>>())
            } else {
                return Err("Data must be array of arrays.");
            }
        }
        let cols = raw_data.len() / rows;

        if cols * rows == raw_data.len() {
            Ok(Matrix::new(rows, cols, raw_data))
        } else {
            Err("All rows must be of equal size.")
        }


    } else {
        Err("Data must be an array")
    }
}
