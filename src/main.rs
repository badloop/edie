// use std::fs::{self, File};
// use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Default)]
struct Item {
    name: String,
    count: i8,
}

#[derive(Debug, Default)]
struct Inventory {
    items: Option<Vec<Item>>,
}

#[derive(Debug, Default)]
struct Suit {
    name: Option<String>,
    suit_type: Option<String>,
    inventory: Option<Inventory>,
}

#[derive(Debug, Default)]
struct Commander {
    name: Option<String>,
    suit: Option<Suit>,
}

fn main() {
    println!("Commander: {:?}", Commander::default());
}
