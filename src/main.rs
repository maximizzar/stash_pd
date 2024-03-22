use chrono::{DateTime, Utc};
use std::fs;
use std::fs::DirEntry;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct JsonData {
    title: String,
    date: String,
    galleries: Vec<Gallery>,
    performers: Vec<String>,
    tags: Vec<String>,
    files: Vec<String>,
    created_at: String,
    updated_at: String,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Gallery {
    title: String,
}
fn main() {
    let mut arguments: Vec<String> = std::env::args().collect();
    if arguments.len() > 4 {
        println!("Too many Arguments:")
    }
    let directory_path = arguments.get(2)
        .expect("Please provide a Metadata dir. (Stash-exported json files)");
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                process_entry(entry);
            }
        }
    } else {
        eprintln!("Error reading directory");
    }
}
fn process_entry(entry: DirEntry) {
    let json_file_path = entry.path();
    let json_string: String = fs::read_to_string(&json_file_path.as_os_str()).expect("Json File not Readable!");
    let image: JsonData = serde_json::from_str(&json_string.to_string().as_str()).expect("Json Config not valid!");

    if let Some(extension) = json_file_path.extension() {
        if extension.eq("json") {
            update_metadata(image, json_file_path);
        }
    } else {
        eprintln!("Not a json config!");
    }
}
fn update_metadata(mut image: JsonData, json_file_path: PathBuf) {
    image.title = get_title(&image).unwrap();
    image.date = get_date(&image).unwrap();
    image.galleries = get_galleries(&image).unwrap();
    image.performers = get_performers(&image).unwrap();
    image.tags = get_tags(&image).unwrap();

    write_to_json(&mut image, json_file_path.to_str().unwrap());
}
fn get_title(image: &JsonData) -> Option<String> {
    if image.title.is_empty() {
        let image_path = Path::new(image.files.get(0).unwrap());
        Some(image_path.file_name());
    }
    None
}
fn get_date(image: &JsonData) -> Option<String> {
    if image.date.is_empty() {
        match fs::metadata(image.files.get(0).unwrap()) {
            Ok(metadata) => {
                if let Ok(modified_time) = metadata.modified() {
                    let datetime: DateTime<Utc> = DateTime::from(modified_time);
                    let formatted_date = datetime.format("%Y-%m-%d").to_string();
                    return Some(formatted_date)
                } else {
                    eprintln!("Failed to get modified time")
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    return None
}
fn get_galleries(image: &JsonData) -> Option<Vec<Gallery>> {
    if image.galleries.is_empty() {

    }
    None
}
fn get_performers(image: &JsonData) -> Option<Vec<String>> {
    if image.performers.is_empty() {
        let file_path = &image.files.get(0).unwrap().to_string();
        let parent_dir = Path::new(file_path).parent();
        Some(parent_dir.is_some());
    }
    None
}
fn get_tags(image: &JsonData) -> Option<Vec<String>> {
    if image.tags.is_empty() {

    }
    None
}

fn write_to_json(image: &mut JsonData, json_file_path: &str) {
    image.title = get_title(&image).is_some().to_string();
    image.date = get_date(&image).is_some().to_string();
    image.performers = get_performers(&image).unwrap();

    let image_string = serde_json::to_string_pretty(image).expect("Image Struct was damaged.");
    fs::write(&json_file_path, image_string).expect("Json File not Writeable!");
}
