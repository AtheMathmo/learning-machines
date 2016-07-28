extern crate iron;
#[macro_use]
extern crate router;
extern crate mount;
extern crate rustc_serialize;
extern crate staticfile;
extern crate handlebars_iron as hbsi;
extern crate rusty_machine as rm;

mod server;
mod models;

use iron::prelude::*;
use mount::Mount;
use staticfile::Static;
use hbsi::{HandlebarsEngine, DirectorySource};

use std::str::FromStr;
use std::path::Path;
use std::env;

#[cfg(feature="dev")]
use std::sync::Arc;
#[cfg(feature="dev")]
use hbsi::Watchable;

static TEMPLATES_DIR : &'static str = "./static/templates/";

fn get_server_port() -> u16 {
    let port_str = env::var("PORT");

    match port_str {
        Ok(p) => FromStr::from_str(&p).unwrap_or(3000),
        Err(_) => 3000,
    }
}

#[cfg(feature="dev")]
fn chain_handlebars(hbse: HandlebarsEngine, chain: &mut Chain) {
    let hbse_ref = Arc::new(hbse);
    hbse_ref.watch(TEMPLATES_DIR);
    chain.link_after(hbse_ref);
}

#[cfg(not(feature="dev"))]
fn chain_handlebars(hbse: HandlebarsEngine, chain: &mut Chain) {
    chain.link_after(hbse);
}


fn main() {
    let mut hbse = HandlebarsEngine::new();
    let templates = Box::new(DirectorySource::new(TEMPLATES_DIR, ".hbs"));
    hbse.add(templates);

    if let Err(r) = hbse.reload() {
        panic!("Failed to reload Handlebars Engine: {}", r);
    }

    let mut mount = Mount::new();

    // Mount the underlying server logic
    mount.mount("/models/",
                router!(
        post "/kmeans" => server::LearningHandler::new(models::k_means::KMeansHandler),
        post "/dbscan" => server::LearningHandler::new(models::dbscan::DBSCANHandler),
    ));

    mount.mount("/assets/", Static::new(Path::new("static/assets")))
        .mount("/css/", Static::new(Path::new("static/css")))
        .mount("/js/", Static::new(Path::new("static/js")));

    // Mount the templating
    mount.mount("/",
                router!(
        get "/" => server::TemplateHandler::new("index"),
        get "/kmeans" => server::TemplateHandler::new("kmeans"),
        get "/dbscan" => server::TemplateHandler::new("dbscan"),
    ));

    let mut chain = Chain::new(mount);
    chain_handlebars(hbse, &mut chain);
        
    Iron::new(chain).http(("0.0.0.0", get_server_port())).unwrap();
}
