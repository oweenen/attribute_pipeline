use fastnbt::from_bytes;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

#[derive(Debug, Deserialize, Serialize)]
struct NbtData {
    i: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    tag: Tag,
}

#[derive(Debug, Deserialize, Serialize)]
struct Tag {
    #[serde(rename = "ExtraAttributes")]
    item_data: ItemData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemData {
    pub id: String,
    pub attributes: HashMap<String, i32>,
}

pub fn decode_item_bytes(item_bytes: &String) -> Result<ItemData, Box<dyn std::error::Error>> {
    // decode base64
    let compressed_bytes = base64::decode(item_bytes)?;

    // gzip Decompress
    let mut gz_decoder = GzDecoder::new(Cursor::new(compressed_bytes));
    let mut decompressed_bytes = Vec::new();
    gz_decoder.read_to_end(&mut decompressed_bytes)?;

    // read nbt data
    let mut nbt_data: NbtData = from_bytes(&decompressed_bytes[..])?;

    // extract item data from nbt data
    if let Some(item) = nbt_data.i.pop() {
        let item_data = item.tag.item_data;
        Ok(item_data)
    } else {
        Err("No item found in NBT data".into())
    }
}
