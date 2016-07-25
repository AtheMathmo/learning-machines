use rm::learning::UnSupModel;
use rm::learning::k_means::KMeansClassifier;

use iron::prelude::*;
use iron::status;
use iron::error::HttpError;
use std::io::{Error, ErrorKind};
use rustc_serialize::json::{ToJson, Object};

use super::ModelHandler;

pub struct KMeansHandler;

impl ModelHandler for KMeansHandler {
    fn handle(&self, input: Object) -> IronResult<Response> {
        let input_data = super::get_matrix_from_data(&input["data"]).unwrap();

        let clusters = match input.get("k") {
            Some(k) => {
                match k.as_u64() {
                    Some(k) => k as usize,
                    None => {
                        return Err(IronError::new(HttpError::Io(Error::new(ErrorKind::InvalidData,
                                                                       "clusters 'k' must be an \
                                                                        unsigned int.")),
                                              status::BadRequest));
                    }
                }
            }
            None => 2,
        };
        let mut model = KMeansClassifier::new(clusters);

        model.train(&input_data);

        // let centroids = model.centroids().as_ref().unwrap();
        let output = model.predict(&input_data);

        Ok(Response::with((status::Ok, format!("{0}", output.data().to_json()))))
    }
}
