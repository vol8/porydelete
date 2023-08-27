#![allow(unused_assignments)]
use indexmap::IndexMap;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn overwrite_json_with_changes(map: &IndexMap<String, Value>, path: &Path) {
    // converts it back to a string
    let map_str = serde_json::to_string_pretty(&map).unwrap();
    // writes data to map.json
    fstream::write_text(path, map_str, true).unwrap();
}

pub fn remove_attributes(map: &mut IndexMap<String, Value>, args: &Vec<String>) {
    let keys_to_remove = vec![
        "connections",
        "object_events",
        "warp_events",
        "bg_events",
        "coord_events",
    ];

    let remove_all = args.len() > 2 && args[2] == "-a";

    if remove_all {
        for key in &keys_to_remove {
            map.remove(*key);
            println!("Success: {} removed!", key);
        }
        return;
    }

    for i in 2..args.len() {
        let mut found_arg = false;
        if let Some(key) = keys_to_remove.iter().find(|&k| args[i] == *k) {
            map.remove(*key);
            println!("Succes: {} removed!", key);
            found_arg = true;
        } else if found_arg == false {
            eprintln!("Error: Unknown argument `{}`", args[i]);
        }
    }
}

pub fn parse_file_and_delete_attribute(
    dir: &Path,
    args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            // Subdir
            let entry = entry?;
            // Path to Subdir
            let path = entry.path();

            if path.is_dir() {
                // 'porytiles-filter'
                if path.file_name().and_then(|n| n.to_str()) == Some("porydelete-filter") {
                    continue;
                }
                parse_file_and_delete_attribute(&path, args)?;
            } else if path.file_name().and_then(|n| n.to_str()) == Some("map.json") {
                println!("Path: {}", path.display());
                let text = fstream::read_text(&path).expect("Error: Cannot find this file");
                let mut map = serde_json::from_str(&text).expect("Error: Failed to read file");
                remove_attributes(&mut map, args);
                overwrite_json_with_changes(&map, &path);
            }
        }
    }
    Ok(())
}
