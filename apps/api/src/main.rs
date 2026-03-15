use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    db_connected: bool,
}

#[derive(Serialize)]
struct InfoResponse {
    name: String,
    description: String,
    version: String,
}

struct AppState {
    db_pool: sqlx::PgPool,
}

async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let db_connected = sqlx::query("SELECT 1")
        .execute(&data.db_pool)
        .await
        .is_ok();

    HttpResponse::Ok().json(HealthResponse {
        status: if db_connected {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
        version: env!("CARGO_PKG_VERSION").to_string(),
        db_connected,
    })
}

async fn info() -> impl Responder {
    HttpResponse::Ok().json(InfoResponse {
        name: "SDD Navigator API".to_string(),
        description: "Backend API for SDD Navigator".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn readiness(data: web::Data<AppState>) -> impl Responder {
    match sqlx::query("SELECT 1").execute(&data.db_pool).await {
        Ok(_) => HttpResponse::Ok().body("ready"),
        Err(_) => HttpResponse::ServiceUnavailable().body("not ready"),
    }
}

async fn liveness() -> impl Responder {
    HttpResponse::Ok().body("alive")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/sdd_navigator".to_string());

    let host = env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("API_PORT must be a valid u16");

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Starting server on {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                db_pool: pool.clone(),
            }))
            .route("/health", web::get().to(health_check))
            .route("/api/info", web::get().to(info))
            .route("/readyz", web::get().to(readiness))
            .route("/livez", web::get().to(liveness))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
