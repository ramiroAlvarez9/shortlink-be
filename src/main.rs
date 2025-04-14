mod controllers;
use actix_web::{web, App, HttpServer, Responder};
use controllers::link_controller::create_link;
use dotenv::dotenv;
use std::env;
use tokio_postgres::{Error, NoTls};


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "9090".to_string());
    let _db_connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
        env::var("DB_PORT").unwrap_or_else(|_| "111111".to_string()),
        env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()),
        env::var("DB_PASSWORD").unwrap_or_else(|_| "1111111".to_string()),
        env::var("DB_NAME").unwrap_or_else(|_| "1111111".to_string())
    );

    let (client, connection) = tokio_postgres::connect(&_db_connection_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.batch_execute("SELECT 1").await?;
    println!("Database connected successfully!");
   let client_data = web::Data::new(client); 

   HttpServer::new(move || {
        App::new()
            .app_data(client_data.clone())
            .route("/create", web::post().to(create_link))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}
