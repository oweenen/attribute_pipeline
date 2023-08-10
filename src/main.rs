mod data_extraction;
mod data_source;
mod data_stream;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    let item_auctions_ref = Arc::new(RwLock::new(HashMap::new()));

    let update_thread_item_auctions = item_auctions_ref.clone();
    tokio::spawn(async move {
        loop {
            let new_item_auctions = data_stream::load_data().await.unwrap();
            *update_thread_item_auctions.write().unwrap() = new_item_auctions;
            time::sleep(Duration::from_secs(60)).await;
        }
    });

    loop {
        time::sleep(Duration::from_secs(10)).await;
        let item_auctions = item_auctions_ref.read().unwrap();
        let auctions = item_auctions.get("AURORA_CHESTPLATE");

        match auctions {
            Some(auctions) => println!("There are {} aurora chestplates", auctions.len()),
            None => println!("There are no aurora chestplates :("),
        }
    }
}
