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

use std::str::FromStr;
use std::env;
use std::path::Path;

fn get_server_port() -> u16 {
    let port_str = env::var("PORT");

    match port_str {
        Ok(p) => FromStr::from_str(&p).unwrap_or(3000),
        Err(_) => 3000,
    }
}

fn main() {
    let mut mount = Mount::new();

    mount.mount("/models/",
                router!(
        post "/kmeans" => server::LearningHandler::new(models::k_means::KMeansHandler),
        post "/dbscan" => server::LearningHandler::new(models::dbscan::DBSCANHandler)
    ));

    mount.mount("/", Static::new(Path::new("static")));
    Iron::new(mount).http(("0.0.0.0", get_server_port())).unwrap();
}
