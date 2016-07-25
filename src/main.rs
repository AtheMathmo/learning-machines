extern crate iron;
#[macro_use]
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustc_serialize;
extern crate rusty_machine as rm;

mod server;
mod models;

use iron::prelude::*;
use mount::Mount;
use staticfile::Static;

use std::path::Path;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/models/",
                router!(
        post "/kmeans" => server::LearningHandler::new(models::k_means::KMeansHandler),
        post "/dbscan" => server::LearningHandler::new(models::dbscan::DBSCANHandler)
    ));

    mount.mount("/", Static::new(Path::new("static")));
    Iron::new(mount).http("0.0.0.0:4000").unwrap();
}
