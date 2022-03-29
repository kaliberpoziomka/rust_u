#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;

use std::env;

mod server; // this is for rust compuler to figure out that there should be a file "server" which will be used as a module
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // if we use > cargo expand | code -
    // then we se that: let default_path = "/home/krzysztof/rust_u/http_server/public";
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    // if there is PUBLIC_PATH, then it uses it, else it uses default_path
    dbg!(&public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
