// use std::fs::{self, File};
// use std::io::{Read, Seek, SeekFrom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Item {
    name: String,
    count: i8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Inventory {
    items: Vec<Item>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Suit {
    name: String,
    suit_type: String,
    inventory: Inventory,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Commander {
    name: String,
    suit: Suit,
}

fn main() {
    let mut cmdr = Commander::default();
    cmdr.name = String::from("badloop");
    let j = serde_json::to_string(&cmdr).unwrap();
    println!("{}", j);
}
