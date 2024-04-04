#[derive(Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub(crate) struct Performer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
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

pub fn init_performers_vector(performers: String) -> Vec<Performer> {
    let mut performer_vector: Vec<Performer> = vec![];

    if let Ok(entries) = std::fs::read_dir(performers) {
        for entry in entries {
            if let Ok(entry) = entry {
                let performer_vector_entry = process_entry(entry);
                if performer_vector_entry.is_some() {
                    performer_vector.push(performer_vector_entry.unwrap());
                }
            }
        }
    }
    return performer_vector;
}
fn process_entry(entry: std::fs::DirEntry) -> Option<Performer> {
    let json_file_path = entry.path();
    let json_string: String = std::fs::read_to_string(&json_file_path).expect("Json File not Readable!");
    let performer: Performer = serde_json::from_str(&json_string.to_string().as_str()).expect("Json Config not valid!");
    return Option::from(performer);
}