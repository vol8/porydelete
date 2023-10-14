// gTileset_General is not deleteable. Instead use porytiles to edit it. However every other tileset is deleteable
// Step 1: Regex: const struct Tileset \w+ =\n\{\n    .isCompressed = \w+,\n    .isSecondary = \w+,\n    .tiles = \w+,\n    .palettes = \w+,\n    .metatiles = \w+,\n    .metatileAttributes = \w+,\n    .callback = \w+,\n\};

#![allow(unused_variables)]

use std::path::{Path, PathBuf};
use regex::Regex;
use std::fs;

type PdError = Result<(), Box<dyn std::error::Error>>;

fn get_paths(ts_name: &str) -> Vec<PathBuf> {
    vec![
        // tileset struct definition (for general and other tilesets)
        Path::new("./src/data/tilesets/headers.h").to_path_buf(),
        // tileset definitions for palettes and tiles (ONLY OTHER TILES)
        /*other  */
        Path::new("src/data/tilesets/graphics.h").to_path_buf(),
        /*general*/ Path::new("src/graphics.h").to_path_buf(),
        // Metatiles and MetatilsAttribute definitons (for general and other tilesets)
        Path::new("src/data/tilesets/metatiles.h").to_path_buf(),
        // Location of tile and palette data for tilesets
        /*PRIMARY  */
        Path::new("/data/tilesets/primary/")
            .join(ts_name)
            .to_path_buf(),
        /*SECONDARY*/
        Path::new("/data/tilesets/secondary/")
            .join(ts_name)
            .to_path_buf(),
        // Path to tileset_anims.h
        Path::new("include/tileset_anims.h").to_path_buf(),
        // Path to tileset_anims.c\
        Path::new("src/tileset_anims.c").to_path_buf(),
    ]
}

fn tileset_exists(ts_name: &str) -> bool {
    let paths = get_paths(ts_name);

    for path in paths {
        if !path.exists() {
            println!("Error: '{}' does not exist!", ts_name);
            return false;
        }
    }
    true
}

fn remove_tileset_def(ts_name: &str) -> PdError {
    let re_prefix = r"const struct Tileset ";
    let re_suffix = r" =\n\{\n    .isCompressed = \w+,\n    .isSecondary = \w+,\n    .tiles = \w+,\n    .palettes = \w+,\n    .metatiles = \w+,\n    .metatileAttributes = \w+,\n    .callback = \w+,\n\};";

    let re = Regex::new(format!(r"{}{}{}", re_prefix, ts_name, re_suffix).as_str()).unwrap();
    let path = Path::new("./src/data/tilesets/headers.h").to_path_buf();

    let contents = fs::read_to_string(&path).unwrap();

    let tileset_def = re.find(&contents);

    if tileset_def.is_none() {
        eprintln!("Step 1: Error: Couldn't find tileset definition in './src/data/tilesets/headers.h'.");
        Ok(())
    } else {
        let new = contents.replace(tileset_def.unwrap().as_str(), "");
        fs::write(path, new)?;
        println!("Step 1: Found and deleted tileset definition!");
        Ok(())
    }
}

fn remove_tiles_pal_def(ts_name: &str) -> PdError {
    let re_tiles_prefix = "const u32 ";
    let re_tiles_suffix = "\\[\\] = INCBIN_U32\\(\"data\\/tilesets\\/\\w+\\/\\w+\\/tiles.4bpp.lz\"\\);";

    let re_pals_prefix = "const u16 ";
    let re_pals_suffix = "\\[\\]\\[16] =\\n\\{\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/00.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/01.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/02.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/03.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/04.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/05.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/06.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/07.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/08.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/09.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/10.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/11.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/12.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/13.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/14.gbapal\"\\),\\n\\tINCBIN_U16\\(\"\\w+\\/tilesets\\/\\w+\\/\\w+\\/palettes\\/15.gbapal\"\\),\\n\\};";

    // Names for the tileset you specified
    let def_name_tiles = ts_name.replace("gTileset", "gTilesetTiles");
    let def_name_pals = ts_name.replace("gTileset", "gTilesetPalettes");

    // Regular expressions
    let re_tiles = Regex::new(format!("{}{}{}", re_tiles_prefix, def_name_tiles, re_tiles_suffix).as_str()).unwrap();
    let re_pals  = Regex::new(format!("{}{}{}", re_pals_prefix, def_name_pals, re_pals_suffix).as_str()).unwrap();
    let path = Path::new("src/data/tilesets/graphics.h").to_path_buf();

    let contents = fs::read_to_string(&path).unwrap();

    let tiles_def = re_tiles.find(&contents);
    let pals_def = re_pals.find(&contents);

    if tiles_def.is_none() || pals_def.is_none() {
        eprintln!("Step 2: Error: Couldn't find tileset gfx and palettes gfx definitions in './src/data/tilesets/graphics.h'.");
        Ok(())
    } else {
        let new = contents.replace(pals_def.unwrap().as_str(), "").replace(tiles_def.unwrap().as_str(), "");
        fs::write(path, new)?;
        println!("Step 2: Found and deleted tileset gfx and palettes gfx definitions!");
        Ok(())
    }
}

pub fn execute_del(ts_name: &str) -> PdError {
    //let ts_exists: bool = tileset_exists(ts_name);

    //if ts_exists {
        remove_tileset_def(ts_name)?; // Finished 'Step 1'
        remove_tiles_pal_def(ts_name)?; // Finished 'Step 2'
    //}
    Ok(())
}
