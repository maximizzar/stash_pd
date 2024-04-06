use std::ops::Deref;
use crate::performer::{init_performers_vector, Performer};

mod image;
mod performer;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() > 4 {
        eprintln!("Too many Arguments!");
        std::process::exit(1);
    }
    if let Some(index) = &arguments.iter().position(|arg| arg == "--directory_path") {
        let directory_path = arguments.get(index + 1).unwrap().deref();
        let dry_run = arguments.clone().iter().any(|arg| arg == "--dry-run");

        let mut performers: Vec<Performer> = init_performers_vector(directory_path.to_string() + "/performers");
        let images = directory_path.to_string() + "/images";

        if let Ok(entries) = std::fs::read_dir(images) {
            for entry in entries {
                if let Ok(entry) = entry {
                    image::process_entry(entry, &mut performers, dry_run);
                }
            }
        }
    } else {
        eprintln!("Please Provide a valid Path (Stash-exported json files");
    }
}
