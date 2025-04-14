use actix_web::{web, HttpResponse, Responder};
use regex::Regex;
use serde::Deserialize;
use tokio_postgres::Client; // Importar el cliente de PostgreSQL
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LinkData {
    url: String,
}
// TO DO: add middleware put an api key 
pub async fn create_link(
    link_data: web::Json<LinkData>,
    db_client: web::Data<Client> 
) -> impl Responder {
    let original_link = link_data.url.clone();
    let id = generate_short_id(); 
    if is_valid_url(&original_link) {
        match db_client.execute(
            "INSERT INTO links (id, original_url) VALUES ($1, $2)", 
            &[&id, &original_link]
        ).await {
            Ok(_) => HttpResponse::Ok().json(id),
            Err(e) => {
                eprintln!("Database error: {}", e);
                HttpResponse::InternalServerError().json("Failed to save URL")
            }
        }
    } else {
        HttpResponse::BadRequest().json("Invalid URL format")
    }
}

fn is_valid_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^(https?://)?([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(/[a-zA-Z0-9-._~:/?#\[\]@!$&'()*+,;=]*)?$").unwrap();
    url_regex.is_match(url)
}

fn generate_short_id() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .take(6)
        .collect::<String>()
}
