use actix_web::post;
use std::{env};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::shapes::{GenerateRequestBody, GenerateResponseBody};


#[post("/urls")]
pub async fn generate_short_url(
    data: web::Json<GenerateRequestBody>,
    database: web::Data<PgPool>,
) -> impl Responder {
    println!("Received URL: {}", data.original_url);

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let short_code = crate::utils::generate_short_code(); 

    let result = sqlx::query(r#"
        INSERT INTO urls (original_url, short_code, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '30 days')
    "#)
        .bind(&data.original_url)
        .bind(&short_code)
        .execute(database.get_ref())
        .await; 

    match result {
        Ok(e) => println!("Inserted URL into database: {:?}", e),
        Err(e) => {
            eprintln!("Failed to insert URL into database: {}", e);
            return HttpResponse::InternalServerError().body("Failed to insert URL into database");
        }
    }

    HttpResponse::Ok().json(GenerateResponseBody { base_url: format!("{}/{}", base_url, short_code) })
}

