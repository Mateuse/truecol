mod repository;

use repository::db;
use repository::cities::get_cities;

use actix_web::{web, App, HttpServer};
use actix_web::web::Data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db::connect().await.expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .route("/cities", web::get().to(get_cities))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
