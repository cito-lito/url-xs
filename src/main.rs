mod controllers;
mod models;
mod routes;
mod server;

use dotenv::dotenv;
use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // for local dev
    init_env();
    dotenv().ok();
    env_logger::init();

    // start http server
    let port = std::env::var("PORT").unwrap_or_else(|_| "3003".to_string());
    let host = "127.0.0.1".to_string();
    let server = Server::new(host, port.parse().unwrap());
    server.run().await
}

fn init_env() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
}
