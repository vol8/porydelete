use fs_extra::dir::{self, CopyOptions};
use std::fs;
use std::path::Path;

pub struct PdFilter {
    // Elements which are going to be filtered
    pub elem: Vec<String>,
    // Destination directory where the elements are moved to when filtering
    pub start_dir: String,
    pub dest_dir: String,
}

impl PdFilter {
    // checks and execute if you want to filter or defilter based on the boolean 'defilter'
    fn fil_or_defil(&self, defilter: bool) -> std::io::Result<()> {
        // Default options for fs_extra::dir::move_dir
        let options = CopyOptions::new();
        if &self.elem.len() < &4 {
            eprintln!("Error: Too few arguments. Elementnames are missing. Consider using '-a' for all elements.");
        } else if !defilter {
            for e in &self.elem {
                // We only want the map names not the first 3 arguments
                if e == &self.elem[0] || e == &self.elem[1] || e == &self.elem[2] {
                    continue;
                } else {
                    // Creates a path for the map given via arguments
                    let elem_path = Path::new(&self.start_dir).join(e);
                    // Creates the path where the map should be moved to.
                    // Why 'e' as well? Because 'fs_extra::dir::move_dir()' would only move the contents of 'elem_path' and not 'elem_path' itself
                    let pd_filter_dir_for_checking_and_creating = Path::new(&self.dest_dir);
                    if !pd_filter_dir_for_checking_and_creating.exists() {
                        fs::create_dir(&self.dest_dir)?;
                    }
                    let dest_path = Path::new(&self.dest_dir).join(e);
                    if !elem_path.exists() {
                        eprintln!("Error: Element '{}' not found.", e);
                    } else {
                        if let Err(err) = fs::rename(&elem_path, &dest_path) {
                            eprintln!("Error: Could not move '{}': {}", e, err);
                        } else {
                            println!("Success: '{}' moved.", e);
                        }
                    }
                }
            }
        } else if defilter {
            if &self.elem.len() >= &4 && &self.elem[3] != "-a" {
                for e in &self.elem {
                    if e == &self.elem[0] || e == &self.elem[1] || e == &self.elem[2] {
                        continue;
                    }
                    // Creates a path for the map given via arguments
                    let elem_path = Path::new("./data/maps/porydelete-filter/").join(e);
                    // Creates the path where the map should be moved to.
                    let dest_path = Path::new(&self.start_dir);
                    if !elem_path.exists() {
                        eprintln!("Error: Element '{}' not found.", e);
                    } else {
                        if let Err(err) = dir::move_dir(&elem_path, &dest_path, &options) {
                            eprintln!("Error: Could not move '{}': {}", e, err);
                        } else {
                            println!("Success: '{}' moved.", e);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // This is the function which will be used when defiltering, because of the error handling.
    // Made for my match statements in 'main.rs'
    pub fn do_defilter(self) {
        if let Err(err) = PdFilter::fil_or_defil(&self, true) {
            eprintln!("Error: {}", err)
        }
    }

    // This is the function which will be used when filtering, because of the error handling.
    // Made for my match statements in 'main.rs'
    pub fn do_filter(self) {
        if let Err(err) = PdFilter::fil_or_defil(&self, false) {
            eprintln!("Error: {}", err)
        }
    }
}
