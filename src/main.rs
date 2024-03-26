use std::ops::Deref;

mod image;

#[derive(Debug, serde::Deserialize, PartialEq, serde::Serialize)]
struct Performer {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instagram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birthdate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    death_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ethnicity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hair_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eye_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    measurements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fake_tits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    career_length: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tattoos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    piercings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rating: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

#[derive(Debug, serde::Deserialize, PartialEq, serde::Serialize)]
struct Studio {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rating: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}


fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() > 4 {
        eprintln!("Too many Arguments!");
        std::process::exit(1);
    }
    if let Some(index) = &arguments.iter().position(|arg| arg == "--directory_path") {
        let directory_path = arguments.get(index + 1).unwrap().deref();
        let dry_run = arguments.clone().iter().any(|arg| arg == "--dry-run");

        let images = directory_path.to_string() + "/images";
        let _scenes = directory_path.to_string() + "/scenes";

        if let Ok(entries) = std::fs::read_dir(images) {
            for entry in entries {
                if let Ok(entry) = entry {
                    image::process_entry(entry, dry_run);
                }
            }
        }
    } else {
        eprintln!("Please Provide a valid Path (Stash-exported json files");
    }
}
