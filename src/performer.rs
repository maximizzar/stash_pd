#[derive(Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub(crate) struct Performer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>
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