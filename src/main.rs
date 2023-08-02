use serde_json::Value;
use std::env;
use indexmap::IndexMap;

extern crate fstream;

// test
fn read_settings() -> Result<Vec<u8>, String> {
    Ok(vec![1,2,3])
}

// test
fn exec_for_settings(settings: Vec<u8>) {
    
}

fn main() {
    //let args: Vec<String> = env::args().collect();
    let settings = read_settings().unwrap();
    exec_for_settings(settings);

    // reads text in map.json and stores it into data
    let data = fstream::read_text("map.json").expect("Cannot find this file");

    // Converts the plain text of &data to an indexmap which just contains the ordered json ready for being used
    // Why IndexMap instead of Map? Because otherwise the json wouldn't be in order
    let mut map: IndexMap<String, Value> = serde_json::from_str(&data)
    .expect("failed to read file");
    
    // removes "warp_events" in the copy of `data`
    map.remove("warp_events");

    // converts it back to a string 
    let map_str = serde_json::to_string_pretty(&map).unwrap();

    // writes data to map.json
    fstream::write_text("map.json", map_str, true).unwrap();
}
