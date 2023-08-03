use indexmap::IndexMap;
use std::path::Path;
use serde_json::Value;
use std::fs;

// reads text in map.json
pub fn read_contents_of_map_json(path: &Path) -> String {
    fstream::read_text(path).expect("Cannot find this file")
}

//Converts the plain text of &text to an indexmap which just contains the ordered json ready for being used
// Why IndexMap instead of Map? Because otherwise the json wouldn't be in order
pub fn create_json_indexmap(text: &String) -> IndexMap<String, Value> {
    serde_json::from_str(text).expect("failed to read file")
}

pub fn overwrite_json_with_mod(map: &IndexMap<String, Value>, path: &Path) {
    // converts it back to a string
    let map_str = serde_json::to_string_pretty(&map).unwrap();
    // writes data to map.json
    fstream::write_text(path, map_str, true).unwrap();
}

pub fn remove_for_given_args(map: &mut IndexMap<String, Value>, args: &Vec<String>) {

    let keys_to_remove = vec!["connections", "object_events", "warp_events", "bg_events", "coord_events"];

    for i in 2..args.len() {
        let mut found_arg = false;
        if let Some(key) = keys_to_remove.iter().find(|&k| args[i] == *k) {
            map.remove(*key);
            println!("{}: Removed!", key);
            found_arg = true;
        } else if found_arg == false {
            eprintln!("Error: Unknown argument `{}`", args[i]);
        }
    }
}

pub fn process_file_and_delete_attribute(dir: &Path, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Skip the directory which was specified in args[] to be skipped
                // test not final
                // ignore for now
                if path.file_name().and_then(|n| n.to_str()) == Some("test") {
                    continue;
                }
                process_file_and_delete_attribute(&path, args)?;
            } else if path.file_name().and_then(|n| n.to_str()) == Some("map.json") {
                println!("Path: {}", path.display());
                let text = read_contents_of_map_json(&path);
                let mut map = create_json_indexmap(&text);
                remove_for_given_args(&mut map, args);
                overwrite_json_with_mod(&map, &path);
            }
        }
    }
    Ok(())
}