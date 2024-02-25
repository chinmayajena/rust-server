//this mod file name is required for using server
mod server;
mod http;

use server::Server;
use http::Request;
use http::Method;

fn main() {
    let get = Method::GET;
    let post = Method::POST;
    let delete = Method::DELETE;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
