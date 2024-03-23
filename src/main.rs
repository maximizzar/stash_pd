use std::option::Option;
use chrono::{DateTime, Utc};
use std::fs;
use std::fs::DirEntry;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct JsonData {
    title: Option<String>,
    date: Option<String>,
    galleries: Option<Vec<Gallery>>,
    performers: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    files: Option<Vec<String>>,
    created_at: Option<String>,
    updated_at: Option<String>,
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
    image.title = get_title(&image);
    image.date = get_date(&image);
    image.galleries = get_galleries(&image);
    image.performers = get_performers(&image);
    image.tags = get_tags(&image);

    //write_to_json(&image, json_file_path.to_str().unwrap());
}
fn get_title(image: &JsonData) -> Option<String> {

    if let Some(files) = &image.files {
        if let Some(first_file) = files.get(0) {
            // Extract the file path as a String
            let file_path = Path::new(&first_file.to_string());
            // Return the title as an Option<String>
            Some(file_path.parent().unwrap().to_string_lossy().to_string()).unwrap();
        } else {
            return None;
            // Return None if the files vector is empty
        }
    } else {
        return None;
        // Return None if the files field is None
    }

    let file_path: String = (&image.files.unwrap().get(0).unwrap()).to_string();
    match &image.files {
        Some(files) => Path::new(&file_path),
        None => {None.unwrap()}
    };
    None
}
fn get_date(image: &JsonData) -> Option<String> {
    match fs::metadata(image.files.unwrap().as_slice().get(0).unwrap()) {
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
    return None
}
fn get_galleries(image: &JsonData) -> Option<Vec<Gallery>> {
    None
}
fn get_performers(image: &JsonData) -> Option<Vec<String>> {
    if image.files.is_some() {
        let file_path = image.files.unwrap().get(0);
        let parent_dir = Path::parent((&file_path.unwrap().to_string().as_str()).as_ref());
        Some(parent_dir);
    }
    None
}
fn get_tags(image: &JsonData) -> Option<Vec<String>> {
    None
}

fn write_to_json(image: &mut JsonData, json_file_path: &str) {
    let image_string = serde_json::to_string_pretty(image).expect("Image Struct was damaged.");
    fs::write(&json_file_path, image_string).expect("Json File not Writeable!");
}
