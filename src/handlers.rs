use crate::data_stream::{get_item_bucket, AttributeItemAuction};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Deserialize)]
pub struct AttributePricesParams {
    item_id: String,
    attribute: String,
    attribute_level: i32,
}

#[get("/getAttributePrices")]
pub async fn get_attribute_prices(
    params: web::Query<AttributePricesParams>,
    item_auctions_ref: web::Data<Arc<RwLock<BTreeMap<String, Vec<AttributeItemAuction>>>>>,
) -> impl Responder {
    let item_auctions = item_auctions_ref.read().unwrap();

    if let Some(auctions) = item_auctions.get(&get_item_bucket(&params.item_id)) {
        let select_auctions: Vec<&AttributeItemAuction> = auctions
            .iter()
            .filter(|auction| {
                auction.attributes.get(&params.attribute) == Some(&params.attribute_level)
            })
            .collect();

        HttpResponse::Ok().json(select_auctions)
    } else {
        HttpResponse::Ok().json(Vec::<&AttributeItemAuction>::new())
    }
}
