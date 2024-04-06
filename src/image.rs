use regex::{Regex, Replacer};
use crate::performer::Performer;

#[derive(Debug, serde::Deserialize, PartialEq, serde::Serialize)]
struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organized: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) zip_files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) folder_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
}
pub(crate) fn process_entry(entry: std::fs::DirEntry, performers: &Vec<Performer>, dry_run: bool) {
    let json_file_path = entry.path();
    let json_string: String = std::fs::read_to_string(&json_file_path).expect("Json File not Readable");
    let mut image: Image = serde_json::from_str(&json_string.to_string().as_str()).expect("Json Config not valid");

    if image.title.is_none() {
        image.title = get_file_name(&image.files.to_owned().unwrap().get(0).unwrap().as_str());
    }
    if image.date.is_none() {
        image.date = get_date(&image);
    }
    if image.performers.is_none() {
        image.performers = get_performers(&image, &performers);
    }
    update_json(&image, &json_file_path.to_str().unwrap().to_string(), dry_run);
}
fn get_file_name(path_str: &str) -> Option<String> {
    let file_name = std::path::Path::new(&path_str).file_stem().unwrap().to_str().unwrap();
    Some(file_name.to_string())
}
fn get_file_mtime(image: &Image) -> Option<String> {
    let file_path: String = image.files.to_owned().unwrap().get(0).unwrap().as_str().to_string();

    match std::fs::metadata(file_path.clone()) {
        Ok(metadata) => {
            if let Ok(modified_time) = metadata.modified() {
                let datetime: chrono::DateTime<chrono::Utc> = chrono::DateTime::from(modified_time);
                let formatted_date = datetime.format("%Y-%m-%d").to_string();
                return Some(formatted_date);
            } else {
                eprintln!("Failed to get modified time")
            }
        }
        Err(e) => eprintln!("Error: {}\nPath: {}\n", e, file_path.to_string().as_str()),
    }
    None
}
fn get_date(image: &Image) -> Option<String> {
    //get_date_from_filename
    let mut date = get_date_from_string(get_file_name(&image.files.to_owned().unwrap().get(0).unwrap().as_str()));
    if let Some(date) = date {
        return Option::from(date);
    }
    //get_date_from_title
    date = get_date_from_string(Some(image.title.clone().unwrap()));
    if let Some(date) = date {
        return Option::from(date);
    }
    //get_file_metadata
    date = get_file_mtime(&image);
    if let Some(date) = date {
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
                    }
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
fn get_performers(image: &Image, performers: &Vec<Performer>) -> Option<Vec<String>> {
    let binding = image.files.as_ref().unwrap().get(0).unwrap().as_str();
    let image_parent_directory = std::path::Path::new(&binding).parent().unwrap().to_str().unwrap();

    if image.performers.as_ref().is_none() {
        let normalized_dir_name = normalize_string(get_file_name(image_parent_directory).unwrap().as_str());
        for performer in performers {
            if normalized_dir_name.eq(&normalize_string(performer.name.as_ref().unwrap().as_str())) {
                return Some(vec![performer.name.as_ref().unwrap().to_string()]);
            }
        }
    }
    fn normalize_string(raw_str: &str) -> String {
      return raw_str.to_lowercase().replace(|c: char| c == '-' || c == '_' || c == ' ', "").to_string();
    }
    None
}

fn update_json(json_data: &Image, json_file_path: &String, dry_run: bool) {
    if dry_run {
        let json_string = serde_json::to_string(&json_data).unwrap();
        println!("{json_string}");
        return;
    }
    let json_string = serde_json::to_string_pretty(&json_data).unwrap();
    std::fs::write(json_file_path, json_string).expect("Json File not Writeable!");
}
