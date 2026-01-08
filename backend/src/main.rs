use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::mssql::MssqlPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;
use std::env;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Try connecting to the specified database (RustCrudApp)
    let pool = match MssqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await 
    {
        Ok(p) => {
            println!("Connected to RustCrudApp database.");
            p
        },
        Err(_) => {
            println!("Failed to connect to RustCrudApp. Attempting to connect to 'master' for initialization...");
            // Replace database name with master
            // Assumption: connection string format ...database=RustCrudApp...
            // Simple replace for this specific format
            let master_url = database_url.replace("database=RustCrudApp", "database=master");
            
            MssqlPoolOptions::new()
                .max_connections(5)
                .connect(&master_url)
                .await
                .expect("Failed to connect to master database. Please check your connection string.")
        }
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/users", get(handlers::get_users).post(handlers::create_user))
        .route("/users/:id", put(handlers::update_user).delete(handlers::delete_user))
        .route("/init-db", post(handlers::initialize_db))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}