use std::env;

use actix_web::{App, HttpServer, web};

mod database;
mod shapes;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let database_url = env::var("DATABASE_URL")?;
    let database = web::Data::new(database::connect(&database_url).await?);

    println!("App is running on http://localhost:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .service(routes::generate_short_url)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
