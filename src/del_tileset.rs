#![allow(unused_variables)]

use std::path::{Path, PathBuf};


type PdError = Result<(), Box<dyn std::error::Error>>;

fn tileset_exists(ts_name: &str) -> bool {
    let paths = get_paths(ts_name);

    for path in paths {
        if !path.exists() {
            println!("Error: '{}' does not exist! Make sure that this executable is located in the root of your decompilation project.", ts_name);
            return false;
        }
    }
    true
}

fn get_paths(ts_name: &str) -> Vec<PathBuf> {
    vec![
        // tileset struct definition (for general and other tilesets)
        Path::new("./src/data/tilesets/headers.h").to_path_buf(),
        // tileset definitions for palettes and tiles (ONLY OTHER TILES)
        /*other  */Path::new("src/data/tilesets/graphics.h").to_path_buf(),
        /*general*/Path::new("src/graphics.h").to_path_buf(),
        
        // Metatiles and MetatilsAttribute definitons (for general and other tilesets)
        Path::new("src/data/tilesets/metatiles.h").to_path_buf(),
        // Location of tile and palette data for tilesets
        /*PRIMARY  */Path::new("/data/tilesets/primary/").join(ts_name).to_path_buf(),
        /*SECONDARY*/Path::new("/data/tilesets/secondary/").join(ts_name).to_path_buf(),

        // Path to tileset_anims.h
        Path::new("include/tileset_anims.h").to_path_buf(),
        // Path to tileset_anims.c\
        Path::new("src/tileset_anims.c").to_path_buf(),

    ]
}

pub fn execute_del(ts_name: &str) -> PdError {
    let ts_exists: bool = tileset_exists(ts_name);

    if ts_exists {

    }
    Ok(())
}
