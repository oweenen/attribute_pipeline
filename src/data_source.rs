use serde::{Deserialize, Serialize};
extern crate reqwest;
use reqwest::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionPage {
    pub success: bool,
    pub page: i32,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
    #[serde(rename = "totalAuctions")]
    pub total_auctions: i32,
    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,
    pub auctions: Vec<Auction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auction {
    pub uuid: String,
    pub bin: bool,
    pub start: i64,
    pub end: i64,
    pub item_name: String,
    pub item_lore: String,
    pub item_bytes: String,
    pub extra: String,
    pub category: String,
    pub starting_bid: i64,
}

pub async fn fetch_auction_page(page_number: i32) -> Result<AuctionPage, Error> {
    let url = format!(
        "https://api.hypixel.net/skyblock/auctions?page={}",
        page_number
    );
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let status = response.status();
    if status != 200 {
        println!("error status {}", status);
    }

    let auction_page: AuctionPage = response.json().await?;

    Ok(auction_page)
}
