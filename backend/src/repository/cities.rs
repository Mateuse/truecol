use serde::{Deserialize, Serialize};
use actix_web::{web, Responder, HttpResponse};
use mongodb::{Database, bson};
use futures::stream::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub rent_1br: String
}

#[derive(Deserialize)]
pub struct CitiesQuery {
    cities: Option<String>,
}

pub async fn get_cities(data: web::Data<Database>, query: web::Query<CitiesQuery>) -> impl Responder {
    let cities_collection = data.collection::<City>("cities");

    let mut filter = bson::doc! {};
    if let Some(cities) = &query.cities {
        let city_list: Vec<&str> = cities.split(',').collect();
        filter = bson::doc! {"name": {"$in": city_list}};
    }

    match cities_collection.find(filter, None).await {
        Ok(mut cursor) => {
            let mut cities = Vec::new();
            while let Some(city) = cursor.try_next().await.unwrap_or(None) {
                cities.push(city);
            }
            HttpResponse::Ok().json(cities)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}