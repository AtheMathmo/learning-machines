use rm::learning::UnSupModel;
use rm::learning::dbscan::DBSCAN;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json::{ToJson, Object};

use super::ModelHandler;

pub struct DBSCANHandler;

impl ModelHandler for DBSCANHandler {
    fn handle(&self, input: Object) -> IronResult<Response> {
        let mut model = DBSCAN::new(40.0, 3);
        model.set_predictive(true);

        let input_data = super::get_matrix_from_data(&input["data"]).unwrap();

        model.train(&input_data);
        let output = model.predict(&input_data);
        let class_output = output.data().iter().map(|&x| match x {
            Some(x) => x as i32,
            None => -1,
        }).collect::<Vec<_>>();

        Ok(Response::with((status::Ok, format!("{0}", class_output.to_json()))))
    }
}
