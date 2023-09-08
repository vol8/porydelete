use crate::args::Args;
use fs_extra::dir::{self, CopyOptions};
use std::fs;
use std::path::Path;

pub trait PdFilter {
    fn fil_or_defil(&self, defilter: bool) -> std::io::Result<()>;
    fn do_defilter(self);
    fn do_filter(self);
}

pub struct MaFilter<'a> {
    // Elements which are going to be filtered
    pub elem: &'a Args,
    // Destination directory where the elements are moved to when filtering
    pub start_dir: String,
    pub dest_dir: String,
}

impl<'a> PdFilter for MaFilter<'a> {
    // checks and execute if you want to filter or defilter based on the boolean 'defilter'
    fn fil_or_defil(&self, defilter: bool) -> std::io::Result<()> {
        // Default options for fs_extra::dir::move_dir
        let options = CopyOptions::new();
        if !defilter {
            // Creates a path for the map given via arguments
            let elem_path = Path::new(&self.start_dir).join(self.elem.value.clone());
            // Creates the path where the map should be moved to.
            // Why 'e' as well? Because 'fs_extra::dir::move_dir()' would only move the contents of 'elem_path' and not 'elem_path' itself
            let pd_filter_dir_for_checking_and_creating = Path::new(&self.dest_dir);
            if !pd_filter_dir_for_checking_and_creating.exists() {
                fs::create_dir(&self.dest_dir)?;
            }
            let dest_path = Path::new(&self.dest_dir).join(self.elem.value.clone());
            if !elem_path.exists() {
                eprintln!("Error: Element '{}' not found.", self.elem.value);
            } else {
                if let Err(err) = fs::rename(&elem_path, &dest_path) {
                    eprintln!("Error: Could not move '{}': {}", self.elem.value, err);
                } else {
                    println!("Success: '{}' moved.", self.elem.value);
                }
            }
        } else if defilter {
            // Creates a path for the map given via arguments
            let elem_path = Path::new("./data/maps/porydelete-filter/").join(self.elem.value.clone());
            // Creates the path where the map should be moved to.
            let dest_path = Path::new(&self.start_dir);
            if !elem_path.exists() {
                eprintln!("Error: Element '{}' not found.", self.elem.value);
            } else {
                if let Err(err) = dir::move_dir(&elem_path, &dest_path, &options) {
                    eprintln!("Error: Could not move '{}': {}", self.elem.value, err);
                } else {
                    println!("Success: '{}' moved.", self.elem.value);
                }
            }
        }
        Ok(())
    }

    // This is the function which will be used when defiltering, because of the error handling.
    // Made for my match statements in 'main.rs'
    fn do_defilter(self) {
        if let Err(err) = PdFilter::fil_or_defil(&self, true) {
            eprintln!("Error: {}", err)
        }
    }

    // This is the function which will be used when filtering, because of the error handling.
    // Made for my match statements in 'main.rs'
    fn do_filter(self) {
        if let Err(err) = PdFilter::fil_or_defil(&self, false) {
            eprintln!("Error: {}", err)
        }
    }
}
