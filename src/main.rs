use std::ops::Deref;
use regex::Regex;

#[derive(Debug, serde::Deserialize, PartialEq, serde::Serialize)]
struct JsonData {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    galleries: Option<Vec<Gallery>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
}
#[derive(Debug, serde::Deserialize, PartialEq, serde::Serialize)]
struct Gallery {
    title: String,
}

// Any value that is present is considered Some value, including null.
fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: serde::Deserialize<'de>, D: serde::Deserializer<'de> {
    serde::Deserialize::deserialize(deserializer).map(Some)
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() > 4  {
        eprintln!("Too many Arguments!");
        std::process::exit(1);
    }
    if let Some(index) = &arguments.iter().position(|arg| arg == "--directory_path") {
        let directory_path = arguments.get(index + 1).unwrap().deref();
        let dry_run = arguments.clone().iter().any(|arg| arg == "--dry-run");

        if let Ok(entries) = std::fs::read_dir(directory_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    process_entry(entry, dry_run);
                }
            }
        }
    } else {
        eprintln!("Please Provide a valid Path (Stash-exported json files");
    }
}
fn process_entry(entry: std::fs::DirEntry, dry_run: bool) {
    let json_file_path = entry.path();
    let json_string: String = std::fs::read_to_string(&json_file_path).expect("Json File not Readable!");
    let mut image: JsonData = serde_json::from_str(&json_string.to_string().as_str()).expect("Json Config not valid!");

    if image.title.is_none() {
        image.title = get_file_name(&image);
    }
    if image.date.is_none() {
        image.date = get_date(&image);
    }
    update_json(&image, &json_file_path.to_str().unwrap().to_string(), dry_run);
}

fn get_file_name(image: &JsonData) -> Option<String> {
    let path_str: String = image.files.to_owned().unwrap().get(0).unwrap().as_str().to_string();
    let file_name = std::path::Path::new(&path_str).file_stem().unwrap().to_str().unwrap();
    Some(file_name.to_string())
}
fn get_file_metadata(image: &JsonData) -> Option<String> {
    let file_path: String = image.files.to_owned().unwrap().get(0).unwrap().as_str().to_string();

    match std::fs::metadata(file_path) {
        Ok(metadata) => {
            if let Ok(modified_time) = metadata.modified() {
                let datetime: chrono::DateTime<chrono::Utc> = chrono::DateTime::from(modified_time);
                let formatted_date = datetime.format("%Y-%m-%d").to_string();
                return Some(formatted_date)
            } else {
                eprintln!("Failed to get modified time")
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
    None
}

fn get_date(image: &JsonData) -> Option<String> {
    //get_date_from_filename
    let mut date = get_date_from_string(get_file_name(&image));
    if let Some(date) = date {
        return Option::from(date);
    }
    //get_date_from_title
    date = get_date_from_string(Some(image.title.clone().unwrap()));
    if let Some(date) = date {
        println!("get_date_from_title: {}", date);
        return Option::from(date);
    }
    //get_file_metadata
    date = get_file_metadata(&image);
    if let Some(date) = date {
        println!("get_file_metadata: {}", date);
        return Option::from(date);
    }
    fn get_date_from_string(date_string: Option<String>) -> Option<String> {
        fn convert_date(input_date: &str) -> Option<String> {
            let date_formats = &["%Y%m%d", "%Y-%m-%d", "%d%m%Y", "%d-%m-%Y"];
            let output_format = "%Y-%m-%d";

            for date_format in date_formats {
                match chrono::NaiveDate::parse_from_str(input_date, date_format) {
                    Ok(date_obj) => {
                        return Some(date_obj.format(output_format).to_string());
                    },
                    Err(_) => continue,
                }
            }

            None
        }

        if date_string.is_none() {
            return None;
        }

        let date_formats = vec![
            r"\d{4}\d{2}\d{2}",     // YYYYMMDD
            r"\d{4}-\d{2}-\d{2}",   // YYYY-MM-DD
            r"\d{2}\d{2}\d{4}",     // DDMMYYYY
            r"\d{2}-\d{2}-\d{4}",   // DD-MM-YYYY
        ];

        for format in date_formats {
            let re = Regex::new(format).unwrap();
            if let Some(captures) = re.captures(date_string.clone().unwrap().to_string().as_str()) {
                if let Some(date) = captures.get(0) {
                    if let Some(converted_date) = convert_date(date.as_str()) {
                        return Some(converted_date);
                    }
                }
            }
        }
        None
    }
    None
}

fn update_json(json_data: &JsonData, json_file_path: &String, dry_run: bool) {
    if dry_run {
        let json_string = serde_json::to_string(&json_data).unwrap();
        println!("{json_string}");
        return;
    }
    let json_string = serde_json::to_string_pretty(&json_data).unwrap();
    std::fs::write(json_file_path, json_string).expect("Json File not Writeable!");
}
