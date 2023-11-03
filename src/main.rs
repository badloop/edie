// use std::fs::{self, File};
// use std::io::{Read, Seek, SeekFrom};
use axum::{
    extract::Path,
    routing::{delete, get},
    Router,
};
use read_iter::ReadIter;
use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Result, Value};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

struct Travel {
    systems: Vec<System>,
}

struct System {
    name: String,
    star: Star,
    bodies: Vec<Body>,
}

struct Star {
    id: u32,
    name: String,
    star_type: String,
}

struct Body {
    id: u8,
    name: String,
    parent: String,
    children: Vec<Body>,
    star_type: String,
    discovered_by: String,
    mapped_by: String,
}
fn main() {
    // Main object initialization
    let mut cmdr = Commander::default();

    // Read and parse ED log files to create user objects
    let file_name = "/home/aaron/.local/share/Steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/Journal.2023-10-17T132535.01.log";
    let file_read = File::open(file_name);
    let file = match file_read {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", e),
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        let j = get_json(l.clone());
        let e = j["event"].as_str();
        match e {
            Some("Commander") => {
                cmdr.name = serde_json::from_str(&j["Name"].to_string()).expect("null");
            }
            Some("Scan") => println!("Scan => {:#?}", j),
            _ => (),
        }
    }

    println!(
        "Commander => {}",
        serde_json::to_string_pretty(&cmdr).unwrap()
    );
    // let app = Router::new()
    //     .route("/", get(root))
    //     .route("/commander/", get(get_commander));
    //
    // cmdr.name = String::from("badloop");
    // let j = serde_json::to_string(&cmdr).unwrap();
    // println!("{}", j);
}

// async fn root() -> &'static string {
//     return &String::from("");
// }
// async fn get_commander(Path(id): Path<String>) {}
//
fn get_json(line: String) -> Value {
    let v: Value = serde_json::from_str(line.as_str()).unwrap();
    v
}
