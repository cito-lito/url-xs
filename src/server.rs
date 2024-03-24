use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Compress, DefaultHeaders, Logger},
    web, App, HttpServer,
};

use sqlx::postgres::PgPool;

use crate::routes::config_routes;

pub struct AppState {
    pub db: PgPool,
}

pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
    pub async fn run(&self) -> std::io::Result<()> {
        log::info!("Starting server at http://{}:{}", self.host, self.port);

        let pool = PgPool::connect(
            &std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set: export DATABASE_URL=..."),
        )
        .await
        .expect("Failed to create pool.");

        let origin = std::env::var("ORIGIN_URL").unwrap_or("http://localhost:5173".to_string());

        log::info!("ORIGIN URL: {}", origin);
        HttpServer::new(move || {
            let cors = Cors::default()
                .allowed_origin(&origin)
                .allowed_methods(vec!["GET", "POST"])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600);
            App::new()
                .wrap(Compress::default())
                .wrap(Logger::default())
                .wrap(cors)
                .wrap(
                    DefaultHeaders::new()
                        .add(("X-FRAME-OPTIONS", "SAMEORIGIN"))
                        .add(("X-Content-Type-Options", "nosniff"))
                        .add(("X-XSS-Protection", "1; mode=block")),
                )
                .app_data(web::Data::new(AppState { db: pool.clone() }))
                .configure(config_routes)
        })
        .bind(format!("{}:{}", self.host, self.port))?
        .run()
        .await
    }
}
