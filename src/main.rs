mod application;
mod domain;
mod infrastructure;
mod routes;
mod server;

use dotenv::dotenv;
use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();
    env_logger::init();

    // start http server
    let port = std::env::var("PORT").unwrap_or_else(|_| 3003.to_string());
    let host = "0.0.0.0".to_string();
    let server = Server::new(host, port);
    server.run().await
}
