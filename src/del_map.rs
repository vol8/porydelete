use std::path::{Path, PathBuf};
use std::fs;

type PdError = Result<(), Box<dyn std::error::Error>>;

fn map_exists(map: &str) -> bool {
    let paths = get_paths(map);
    for path in paths {
        if !path.exists() {
            eprintln!("Error: '{}' does not exist! Have you tried to delete this map on your own?\nCheck the spelling of the map again.", path.display());
            return false;
        }
    }
    true
}

fn get_str_to_be_del(map: &str) -> Vec<String> {
    vec![
        // Map name in ./data/maps/map_groups.json
        String::from(map),
        // Name attribute in ./data/layouts/layouts.json
        String::from(map) + "_Layouts",
        // include in event_scripts.s
        String::from("./include \"data/maps/") + map + "/scripts.inc\"",
    ]
}

fn get_paths(map: &str) -> Vec<PathBuf> {
    vec![
        // Path to map folder
        Path::new("./data/maps/").to_path_buf().join(map),
        // Path to 'map_groups.json'
        Path::new("./data/maps/map_groups.json").to_path_buf(),

        // Path to map layouts folder
        Path::new("./data/layouts/").to_path_buf().join(map),
        // Path to 'layouts.json'
        Path::new("./data/layouts/layouts.json").to_path_buf(),

        // Path to 'event_scripts.s'
        Path::new("./data/event_scripts.s").to_path_buf(),
    ]
}

// Todo
fn remove_dirs(map: PathBuf, layouts: PathBuf) -> PdError {
    Ok(())
}

// Todo
fn remove_include(path: PathBuf, string: String) -> PdError {
    Ok(())
}

// Todo
fn remove_layouts_map_object(path: PathBuf, layout_name: String) -> PdError {
    Ok(())
}

// Todo
fn remove_map_groups_map_name(path: PathBuf, name: String) -> PdError {
    Ok(())
}

pub fn execute_del(map: &str) -> PdError {
    let map_exists = map_exists(map); 
    
    if map_exists {
        let strings = get_str_to_be_del(map);
        let paths = get_paths(map);

        remove_dirs(paths[0].clone(), paths[2].clone())?;
        remove_include(paths[4].clone(), strings[2].clone())?;
        remove_layouts_map_object(paths[3].clone(), strings[1].clone())?;
        remove_map_groups_map_name(paths[1].clone(), strings[0].clone())?;
    }

    Ok(())
}