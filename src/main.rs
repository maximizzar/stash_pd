use chrono::{DateTime, Utc};
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Image {
    title: String,
    date: String,
    performers: Vec<String>,
    tags: Vec<String>,
    files: Vec<String>,
    created_at: String,
    updated_at: String
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
                let file_path = entry.path();
                if let Some(extension) = file_path.extension() {
                    if extension.eq("json") {
                        update_metadata(file_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading directory");
    }
}
fn update_metadata(json_file_path: String) {
    let image_string: String = fs::read_to_string(&json_file_path.as_str()).expect("Json File not Readable!");
    let mut image: Image = serde_json::from_str(&image_string.as_str()).expect("Json Config not valid!");
    image.files = vec!["C:\\Users\\maximizzar\\Downloads\\S_20240308_Girl_VB.png".to_string()];

    let new_date = get_date(&image);
    let new_performers = get_performers(&image);
    write_to_json(new_date, new_performers, &mut image, json_file_path);
}
fn get_date(image: &Image) -> Option<String> {
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
fn get_performers(image: &Image) -> Option<Vec<String>> {
    if image.performers.is_empty() {
        let file_path = "C:\\Users\\maximizzar\\Desktop\\Desk";
        let parent_dir = Path::new(file_path).parent();
        Some(parent_dir);
    }
    None
}
fn write_to_json(new_date: Option<String>, new_performers: Option<Vec<String>>, image: &mut Image,
                 json_file_path: String) {
    if new_date.is_none() && new_performers.is_none() {
        return;
    }
    if new_date.is_some() {
        image.date = new_date.unwrap();
    }
    if new_performers.is_some() {
        image.performers = new_performers.unwrap();
    }
    let image_string = serde_json::to_string_pretty(image).expect("Image Struct was damaged.");
    fs::write(&json_file_path, image_string).expect("Json File not Writeable!");
}
