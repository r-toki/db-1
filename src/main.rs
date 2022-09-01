use actix_web::{web, App, HttpServer};
use app_config::ENV;
use app_state::AppState;
use dotenv::dotenv;
use std::sync::Arc;

mod app_config;
mod app_state;
mod controller;
mod dao;
mod error;
mod model;
mod result;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = &ENV.host;
    let port = &ENV.port;
    let database_url = &ENV.database_url;

    let db = dao::db::Database::new(database_url).await;
    let app_state = web::Data::new(AppState { db: Arc::new(db) });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(controller::index_controller::init)
            .configure(controller::tasks_controller::init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
