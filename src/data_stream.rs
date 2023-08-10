use crate::{
    data_extraction::{decode_item_bytes, ItemData},
    data_source::{fetch_auction_page, Auction},
};
use std::collections::HashMap;
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct AttributeItemAuction {
    pub uuid: String,
    pub price: i64,
    pub item_id: String,
    pub attributes: HashMap<String, i32>,
}

impl AttributeItemAuction {
    fn new(auction: Auction, item_data: ItemData) -> AttributeItemAuction {
        AttributeItemAuction {
            uuid: auction.uuid.clone(),
            price: auction.starting_bid,
            item_id: item_data.id.clone(),
            attributes: item_data.attributes.clone(),
        }
    }
}

pub async fn load_data(
) -> Result<HashMap<String, Vec<AttributeItemAuction>>, Box<dyn std::error::Error>> {
    println!("Fetching first auction page");
    let auction_page = fetch_auction_page(0).await?;
    let mut futures = futures::stream::FuturesUnordered::new();

    for page_number in 0..auction_page.total_pages {
        println!("Fetching auction page {}", page_number);
        let future = fetch_auction_page(page_number.clone());
        futures.push(future);
    }

    println!("Parsing auction pages...");
    let mut new_item_auction_map: HashMap<String, Vec<AttributeItemAuction>> = HashMap::new();
    while let Some(auction_page) = futures.next().await {
        let auction_page = auction_page?;
        for auction in auction_page.auctions {
            if let Ok(item_data) = decode_item_bytes(&auction.item_bytes) {
                let item_id = item_data.id.clone();
                let item_auction = AttributeItemAuction::new(auction, item_data);

                new_item_auction_map
                    .entry(item_id)
                    .or_insert_with(Vec::new)
                    .push(item_auction);
            }
        }
    }

    println!("Sorting auction pages...");
    for auctions in new_item_auction_map.values_mut() {
        auctions.sort_by(|a, b| a.price.cmp(&b.price));
    }
    println!("Done sorting!");

    Ok(new_item_auction_map)
}

//pub fn get_auctions_with_attribute(
//    &self,
//    item_id: &str,
//    attribute: &str,
//    attribute_level: &i32,
//) -> Vec<AttributeItemAuction> {
//    let item_auction_map = self.item_auction_map.lock().unwrap();
//    let item_auctions = item_auction_map.get(item_id).unwrap();
//
//    let mut select_auctions: Vec<AttributeItemAuction> = vec![];
//    for auction in item_auctions {
//        if auction.attributes.get(attribute).unwrap_or(&0) == attribute_level {
//            select_auctions.push(auction.clone());
//        }
//    }
//
//    return select_auctions;
//}
