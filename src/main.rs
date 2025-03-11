use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use dotenv::dotenv;
use log::info;

use coinflip_rust::{init_db_pool, register, login};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "message": "Server is running"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Initialize database connection pool
    let pool = match init_db_pool().await {
        Ok(pool) => {
            info!("Connected to database");
            pool
        },
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };
    
    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            // Add database pool to application state
            .app_data(web::Data::new(pool.clone()))
            // Enable logger middleware
            .wrap(Logger::default())
            // Register routes
            .service(health_check)
            .route("/api/auth/register", web::post().to(register))
            .route("/api/auth/login", web::post().to(login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
