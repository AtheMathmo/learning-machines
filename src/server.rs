//! Module to handle server wide logic

use std::io::Read;

use iron::prelude::*;
use iron::status;
use iron::Handler;

use rustc_serialize::json::Json;

use models::ModelHandler;

/// A wrapper for a ModelHandler.
///
/// This struct will parse the incoming request and
/// pass the relevant data to the model handler.
pub struct LearningHandler<T: ModelHandler> {
    model_handler: T,
}

impl<T: ModelHandler> LearningHandler<T> {
    pub fn new(model: T) -> LearningHandler<T> {
        LearningHandler { model_handler: model }
    }

    fn parse_json<R: Read>(&self, mut body: &mut R) -> IronResult<Json> {
        Json::from_reader(&mut body).map_err(|e| IronError::new(e, status::BadRequest))
    }
}

impl<T: ModelHandler> Handler for LearningHandler<T> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let body_json = try!(self.parse_json(&mut req.body));

        match body_json {
            Json::Object(map) => self.model_handler.handle(map),
            _ => Ok(Response::with((status::BadRequest, "Json must be a map containing data."))),
        }

    }
}
