extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate rusty_machine as rm;

use iron::prelude::*;
use router::Router;

mod server;
mod models;

fn main() {
    let mut router = Router::new();
    router.post("/kmeans",
                server::LearningHandler::new(models::k_means::KMeansHandler));

    Iron::new(router).http("localhost:3000").unwrap();
}
