use rm::learning::UnSupModel;
use rm::learning::dbscan::DBSCAN;

use iron::prelude::*;
use iron::status;
use iron::error::HttpError;
use std::io::{Error, ErrorKind};
use rustc_serialize::json::{ToJson, Object};

use super::ModelHandler;

pub struct DBSCANHandler;

impl ModelHandler for DBSCANHandler {
    fn handle(&self, input: Object) -> IronResult<Response> {
        let input_data = super::get_matrix_from_data(&input["data"]).unwrap();

        let eps = match input.get("eps") {
            Some(eps) => {
                match eps.as_f64() {
                    Some(eps) => eps,
                    None => {
                        return Err(IronError::new(HttpError::Io(Error::new(ErrorKind::InvalidData,
                                                                       "eps must be a float.")),
                                              status::BadRequest));
                    }
                }
            }
            None => 40.0,
        };

        let min_count = match input.get("minCount") {
            Some(min_count) => {
                match min_count.as_u64() {
                    Some(min_count) => min_count as usize,
                    None => {
                        return Err(IronError::new(HttpError::Io(Error::new(ErrorKind::InvalidData,
                                                                       "minCount must be an \
                                                                        unsigned int.")),
                                              status::BadRequest));
                    }
                }
            }
            None => 3,
        };

        let mut model = DBSCAN::new(eps, min_count);
        model.set_predictive(true);

        model.train(&input_data);
        let output = model.predict(&input_data);
        let class_output = output.data()
            .iter()
            .map(|&x| match x {
                Some(x) => x as i32,
                None => -1,
            })
            .collect::<Vec<_>>();

        Ok(Response::with((status::Ok, format!("{0}", class_output.to_json()))))
    }
}
