// gTileset_General is not deleteable. Instead use porytiles to edit it. However every other tileset is deleteable
// Step 1: Regex: const struct Tileset \w+ =\n\{\n    .isCompressed = \w+,\n    .isSecondary = \w+,\n    .tiles = \w+,\n    .palettes = \w+,\n    .metatiles = \w+,\n    .metatileAttributes = \w+,\n    .callback = \w+,\n\};

#![allow(unused_variables)]

use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

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
        eprintln!(
            "Step 1: Error: Couldn't find tileset definition in './src/data/tilesets/headers.h'."
        );
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
    let re_tiles_suffix =
        "\\[\\] = INCBIN_U32\\(\"data\\/tilesets\\/\\w+\\/\\w+\\/tiles\\.4bpp\\.lz\"\\);";

    let re_pals_prefix = "const u16 ";
    let re_pals_suffix = r#"\[\]\[16\] =\s*\{(?:\s*\n\s*INCBIN_U16\("data/tilesets/\w+/\w+/palettes/\d{2}\.gbapal"\),)*\s*\};"#;

    // Names for the tileset you specified
    let def_name_tiles = ts_name.replace("gTileset", "gTilesetTiles");
    let def_name_pals = ts_name.replace("gTileset", "gTilesetPalettes");

    // Regular expressions
    let re_tiles =
        Regex::new(format!("{}{}{}", re_tiles_prefix, def_name_tiles, re_tiles_suffix).as_str())
            .unwrap();
    let re_pals =
        Regex::new(format!("{}{}{}", re_pals_prefix, def_name_pals, re_pals_suffix).as_str())
            .unwrap();
    let path = Path::new("src/data/tilesets/graphics.h").to_path_buf();

    let contents = fs::read_to_string(&path).unwrap();

    let tiles_def = re_tiles.find(&contents);
    let pals_def = re_pals.find(&contents);

    if tiles_def.is_none() || pals_def.is_none() {
        eprintln!("Step 2: Error: Couldn't find tileset gfx and palettes gfx definitions in './src/data/tilesets/graphics.h'.");
        Ok(())
    } else {
        let new = contents
            .replace(pals_def.unwrap().as_str(), "")
            .replace(tiles_def.unwrap().as_str(), "");
        fs::write(path, new)?;
        println!("Step 2: Found and deleted tileset gfx and palettes gfx definitions!");
        Ok(())
    }
}

fn remove_metatiles_def(ts_name: &str) -> PdError {
    let def_name_metatiles = ts_name.replace("gTileset", "gMetatiles");
    let def_name_metatiles_attr = ts_name.replace("gTileset", "gMetatileAttributes");

    let prefix = "const u16 ";
    let suffix = "\\[\\] = INCBIN_U16\\(\"\\w+\\/\\w+\\/\\w+\\/\\w+\\/\\w+.\\w+\"\\);";

    let re_metatiles =
        Regex::new(format!("{}{}{}", prefix, def_name_metatiles, suffix).as_str()).unwrap();
    let re_metatiles_attr =
        Regex::new(format!("{}{}{}", prefix, def_name_metatiles_attr, suffix).as_str()).unwrap();

    let path = Path::new("./src/data/tilesets/metatiles.h").to_path_buf();
    let contents = fs::read_to_string(&path).unwrap();

    let found_metatiles = re_metatiles.find(&contents);
    let found_metatiles_attr = re_metatiles_attr.find(&contents);

    if found_metatiles.is_none() || found_metatiles_attr.is_none() {
        eprintln!("Step 3: Error: Couldn't find metatile and metatile attribute definitions in 'src/data/tilesets/metatiles.h'.");
        Ok(())
    } else {
        let new = contents
            .replace(found_metatiles.unwrap().as_str(), "")
            .replace(found_metatiles_attr.unwrap().as_str(), "");
        fs::write(path, new)?;
        println!("Step 3: Found and deleted metatile and metatile attribute definitions!");
        Ok(())
    }
}

fn remove_folder(ts_name: &str) -> PdError {
    let dir_name = ts_name.replace("gTileset_", "/").to_lowercase();
    let path_primary =
        Path::new(format!("./data/tilesets/primary{}", dir_name).as_str()).to_path_buf();
    let path_secondary =
        Path::new(format!("./data/tilesets/secondary{}", dir_name).as_str()).to_path_buf();

    if path_primary.exists() {
        fs::remove_dir_all(&path_primary)?;
        if !path_primary.exists() {
            println!(
                "Step 4: Removed tileset directory '{}'.",
                path_primary.display()
            );
        }
    } else if path_secondary.exists() {
        fs::remove_dir_all(&path_secondary)?;
        if !path_secondary.exists() {
            println!(
                "Step 4: Removed tileset directory '{}'.",
                path_secondary.display()
            );
        }
    } else {
        println!("Step 4: Something went wrong!?");
    }
    Ok(())
}

fn dlanims_init_tileset_anim(ts_name: &str) {
    let header_path = Path::new("./include/tileset_anims.h");
    let source_path = Path::new("./src/tileset_anims.c");

    let fn_name = ts_name.replace("gTileset", "InitTilesetAnim");

    let header_regex = Regex::new(format!(r"\w+ {}\(\w+\);", fn_name).as_str()).unwrap();
    // Have to properly create this regex duuhhhh 
    //let source_regex = Regex::new(r"\w+\s*" + r"\(\w+\)\n\{\n(\s*\w+\s*=\s*\w+;)*\n}");
    
    let header_contents = fs::read_to_string(header_path).unwrap();
    let source_contents = fs::read_to_string(source_path);

    let header_match = header_regex.find(&header_contents).unwrap();
    // let source_match = source_regex.find(&source_contents).unwrap();

    // todo: continue with the removal, by rewriting the file
}

fn remove_animations(ts_name: &str) -> PdError {
    dlanims_init_tileset_anim(ts_name);
    Ok(())
}

pub fn execute_del(ts_name: &str) -> PdError {
    //let ts_exists: bool = tileset_exists(ts_name);
    if ts_name == "gTileset_General" {
        eprintln!("Sorry: Can't delete 'gTileset_General'. Instead, try to reuse it, by using Porytiles: https://github.com/grunt-lucas/porytiles");
        Ok(())
    } else {
        //if ts_exists {
        remove_tileset_def(ts_name)?; // Finished 'Step 1'
        remove_tiles_pal_def(ts_name)?; // Finished 'Step 2'
        remove_metatiles_def(ts_name)?; // Finished 'Step 3'
        remove_folder(ts_name)?; // Finished 'Step 4'
                                 //}
        Ok(())
    }
}
