#![allow(dead_code)]

mod server;
mod http;
mod website_handler;

use std::env;
use server::Server;
use http::Method;
use http::Request;
use website_handler::WebsiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let pub_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    println!("public path: {}", pub_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(pub_path));

}
