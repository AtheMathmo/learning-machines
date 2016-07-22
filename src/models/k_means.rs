use rm::learning::UnSupModel;
use rm::learning::k_means::KMeansClassifier;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json::Object;

use super::ModelHandler;

pub struct KMeansHandler;

impl ModelHandler for KMeansHandler {
    fn handle(&self, input: Object) -> IronResult<Response> {
        let mut model = KMeansClassifier::new(3);
        let input_data = super::get_matrix_from_data(&input["data"]).unwrap();
        model.train(&input_data);
        let centroids = model.centroids().as_ref().unwrap();

        let output = model.predict(&input_data);
        Ok(Response::with((status::Ok, format!("{0}\n{1:?}", centroids, output))))
    } 
}