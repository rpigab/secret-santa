use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantsData {
    pub participants: Vec<String>,
    pub already_gifted_before: Option<HashMap<String, Vec<String>>>,
    pub couples: Option<Vec<(String, String)>>,
}

impl ParticipantsData {
    pub(crate) fn new(input_file: PathBuf) -> Self {
        // Read YAML file into a String
        let mut file = File::open(input_file)
            .expect("Failed to open file");

        let mut yaml_content = String::new();
        file.read_to_string(&mut yaml_content)
            .expect("Failed to read file");

        serde_yaml::from_str(&yaml_content)
            .expect("Failed to deserialize YAML")
    }
}
