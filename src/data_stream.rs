use crate::{
    data_extraction::{decode_item_bytes, ItemData},
    data_source::{fetch_auction_page, Auction},
};
use serde::Serialize;
use std::time::Duration;
use std::{collections::BTreeMap, sync::Arc, sync::RwLock};
use tokio::time;
use tokio_stream::StreamExt;

#[derive(Serialize)]
pub struct AttributeItemAuction {
    pub uuid: String,
    pub price: i64,
    pub item_name: String,
    pub item_id: String,
    pub attributes: BTreeMap<String, i32>,
}

impl AttributeItemAuction {
    fn new(auction: Auction, item_data: ItemData) -> AttributeItemAuction {
        AttributeItemAuction {
            uuid: auction.uuid,
            item_name: auction.item_name,
            price: auction.starting_bid,
            item_id: item_data.id,
            attributes: item_data.attributes,
        }
    }
}

lazy_static! {
    static ref KUUDRA_ARMOR_MAPPING: BTreeMap<&'static str, &'static str> = [
        ("AURORA_HELMET", "KUUDRA_HELMET"),
        ("CRIMSON_HELMET", "KUUDRA_HELMET"),
        ("FERVOR_HELMET", "KUUDRA_HELMET"),
        ("HOLLOW_HELMET", "KUUDRA_HELMET"),
        ("TERROR_HELMET", "KUUDRA_HELMET"),
        ("AURORA_CHESTPLATE", "KUUDRA_CHESTPLATE"),
        ("CRIMSON_CHESTPLATE", "KUUDRA_CHESTPLATE"),
        ("FERVOR_CHESTPLATE", "KUUDRA_CHESTPLATE"),
        ("HOLLOW_CHESTPLATE", "KUUDRA_CHESTPLATE"),
        ("TERROR_CHESTPLATE", "KUUDRA_CHESTPLATE"),
        ("AURORA_LEGGINGS", "KUUDRA_LEGGINGS"),
        ("CRIMSON_LEGGINGS", "KUUDRA_LEGGINGS"),
        ("FERVOR_LEGGINGS", "KUUDRA_LEGGINGS"),
        ("HOLLOW_LEGGINGS", "KUUDRA_LEGGINGS"),
        ("TERROR_LEGGINGS", "KUUDRA_LEGGINGS"),
        ("AURORA_BOOTS", "KUUDRA_BOOTS"),
        ("CRIMSON_BOOTS", "KUUDRA_BOOTS"),
        ("FERVOR_BOOTS", "KUUDRA_BOOTS"),
        ("HOLLOW_BOOTS", "KUUDRA_BOOTS"),
        ("TERROR_BOOTS", "KUUDRA_BOOTS"),
    ]
    .iter()
    .cloned()
    .collect();
}

pub fn get_item_bucket(item_id: &str) -> String {
    match KUUDRA_ARMOR_MAPPING.get(item_id) {
        Some(bucket_id) => bucket_id.to_string(),
        None => item_id.to_string(),
    }
}

async fn load_data(
) -> Result<BTreeMap<String, Vec<AttributeItemAuction>>, Box<dyn std::error::Error>> {
    let auction_page = fetch_auction_page(0).await?;
    let mut futures = futures::stream::FuturesUnordered::new();

    for page_number in 0..auction_page.total_pages {
        let future = fetch_auction_page(page_number.clone());
        futures.push(future);
    }

    let mut new_item_auction_map: BTreeMap<String, Vec<AttributeItemAuction>> = BTreeMap::new();
    while let Some(auction_page) = futures.next().await {
        let auction_page = auction_page?;
        for auction in auction_page.auctions {
            if !auction.bin {
                continue;
            }

            if let Ok(item_data) = decode_item_bytes(&auction.item_bytes) {
                let item_id = item_data.id.clone();
                let item_auction = AttributeItemAuction::new(auction, item_data);

                new_item_auction_map
                    .entry(get_item_bucket(&item_id))
                    .or_insert_with(Vec::new)
                    .push(item_auction);
            }
        }
    }

    for auctions in new_item_auction_map.values_mut() {
        auctions.sort_by(|a, b| a.price.cmp(&b.price));
    }

    Ok(new_item_auction_map)
}

pub fn update_loop(
    item_auctions_ref: Arc<RwLock<BTreeMap<String, Vec<AttributeItemAuction>>>>,
    refresh_rate: Duration,
) {
    tokio::spawn(async move {
        loop {
            match load_data().await {
                Ok(new_item_auctions) => *item_auctions_ref.write().unwrap() = new_item_auctions,
                Err(e) => println!("Refresh failed: {}", e),
            }
            time::sleep(refresh_rate).await;
        }
    });
}
