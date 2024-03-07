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
    let server = Server::new("127.0.0.1".to_string(), 3003);
    server.run().await
}

fn init_env() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
}
