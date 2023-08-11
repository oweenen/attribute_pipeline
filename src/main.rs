#[macro_use]
extern crate lazy_static;

mod data_extraction;
mod data_source;
mod data_stream;
mod handlers;

use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let item_auctions_ref = Arc::new(RwLock::new(HashMap::new()));

    data_stream::update_loop(item_auctions_ref.clone(), Duration::from_secs(60));

    let port = env::var("PORT").unwrap_or_else(|_| "9090".to_string());
    let address = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(item_auctions_ref.clone()))
            .service(handlers::get_attribute_prices)
    })
    .bind(address)?
    .run()
    .await
}
