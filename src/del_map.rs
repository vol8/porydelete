#![allow(unused_variables)] // temp
#![allow(unused_imports)] // temp
use indexmap::IndexMap;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};

type PdError = Result<(), Box<dyn std::error::Error>>;

fn get_paths(map: &str) -> Vec<PathBuf> {
    vec![
        // Path to map folder
        Path::new("./data/maps/").to_path_buf().join(map),
        // Path to map layouts folder
        Path::new("./data/layouts/").to_path_buf().join(map),
    ]
}

fn map_exists(map: &str) -> bool {
    let paths = get_paths(map);
    let mut count = 0;
    for path in paths {
        if path.exists() {
            count += 1;
        }
    }
    if count == 2 {
        return true;

    } else if count == 0 {
        eprintln!("Error: Couldn't find the map '{}'.", map);
        return false;
    } 
    else {
        eprintln!("Error: Some paths couldn't be found. Have you tried to delete a map manually?\nIf you never touched the map files before, except with Porydelete and/or Porymap, please file an issue on my github page.");
        return false;
    }
    
}

fn remove_dirs(map_name: &str) -> PdError {
    let map_path = Path::new("./data/maps/").to_path_buf().join(map_name);
    let layout_path = Path::new("./data/layouts/").to_path_buf().join(map_name);
    fs::remove_dir_all(map_path.clone())?;
    if !map_path.exists() {
        println!(
            "Step 1: Map folder deleted! Folder directory '{}'",
            map_path.display()
        );
    } else {
        eprintln!("Error: Failed to delete map folder '{}'", map_path.display());
    }
    fs::remove_dir_all(layout_path.clone())?;
    if !layout_path.exists() {
        println!(
            "Step 2: Map folder deleted! Folder directory '{}'",
            layout_path.display()
        );
    } else {
        eprintln!(
            "Error: Failed to delete layouts folder '{}'",
            layout_path.display()
        );
    }
    Ok(())
}

fn remove_include(map_name: &str) -> PdError {
    let string = String::from(".include \"data/maps/") + map_name + "/scripts.inc\"";
    let path = Path::new("./data/event_scripts.s");
    if path.exists() {
        // logic: Reads file and the second line just writes the changes to the file. First line does contain error handling by '?'
        let content = fs::read_to_string(path)?.replace(&string, "");
        fstream::write_text(path, content, true);
        println!("Step 3: Include file deleted!");
    }
    Ok(())
}

// Could be better but it works for now
fn remove_layouts_map_object(map: &str) -> PdError {
    let path = Path::new("./data/layouts/layouts.json");
    let layout_name = String::from(map) + "_Layout";
    if path.is_file() {
        if path.file_name().and_then(|n| n.to_str()) == Some("layouts.json") {
            let contents = fs::read_to_string(&path)?;
            let mut map: serde_json::Value =
                serde_json::from_str(&contents).expect("Error: failed to parse map.json");

            if let Some(layouts) = map.get_mut("layouts") {
                // If "layouts" is an array, filter out the object with name "MyMap_Layouts"
                if let Some(layouts_array) = layouts.as_array_mut() {
                    layouts_array.retain(|layout| {
                        if let Some(layout_id) = layout.get("name") {
                            return layout_id != layout_name.as_str();
                        }
                        true
                    });
                    let del_obj_json = serde_json::to_string_pretty(&map)
                        .expect("Error: failed to serialize map.json");
                    let mut indexmap: IndexMap<String, serde_json::Value> =
                        serde_json::from_str(&del_obj_json)?;
                    indexmap.insert(
                        String::from("layouts_table_label"),
                        serde_json::Value::String(String::from("gMapLayouts")),
                    );
                    let modified_json = serde_json::to_string_pretty(&indexmap)?;
                    fstream::write_text(path, modified_json, true).unwrap();
                    println!("Step 4: Layouts.json file modified!");
                } else {
                    // Handle the case where the JSON data is not an array
                    eprintln!("Error: layouts.json is not an array.");
                }
            }
        }
    }
    Ok(())
}

// Could be better but it works for now
fn remove_map_groups_map_name(map: &str) -> PdError {
    let path = Path::new("./data/maps/map_groups.json").to_path_buf();
    let name = map;
    if path.is_file() {
        if path.file_name().and_then(|n| n.to_str()) == Some("map_groups.json") {
            let contents = fs::read_to_string(&path)?;
            let mut map: serde_json::Value =
                serde_json::from_str(&contents).expect("Error: failed to parse map.json");
            if let Some(array) = map.pointer_mut("/gMapGroup_TownsAndRoutes") {
                // Check if the array is an array and not null
                if let Some(array) = array.as_array_mut() {
                    // Remove "Route123" from the array
                    array.retain(|item| item != name);
                }
                let modified_json = serde_json::to_string_pretty(&map)
                    .expect("Error: failed to serialize map.json");
                println!("Step 5: map_groups.json file modified!");
                fstream::write_text(path, modified_json, true).unwrap();
            } else {
                // Handle the case where the JSON data is not an array
                eprintln!("Error: map_groups.json is not an array.");
            }
        }
    }
    Ok(())
}

pub fn execute_del(map: &str) -> PdError {
    let map_exists = map_exists(map);

    if map_exists {
        // Remove map folder and layouts folder
        remove_dirs(map)?;
        // Removes include line from event_scripts.s
        remove_include(map)?;
        // Removes the Map object (Could be better but it works for now)
        remove_layouts_map_object(map)?;
        // Removes the map-name from map-group object (Could be better but it works for now)
        remove_map_groups_map_name(map)?;
        println!("Success: Deleted map '{}'.", map);
        println!("\nInportant Note: Other maps may use '{}' in their files. This can cause errors while compiling.\nMake sure to fix those errors!", map);
    }
    Ok(())
}
