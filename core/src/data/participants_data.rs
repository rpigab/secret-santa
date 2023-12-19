use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct ParticipantsData {
    pub participants: Vec<String>,
    pub already_gifted_before: Option<HashMap<String, Vec<String>>>,
    pub couples: Option<Vec<(String, String)>>,
}

impl TryFrom<PathBuf> for ParticipantsData {
    type Error = anyhow::Error;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        // Read YAML file into a String
        let mut file = File::open(input_file)
            .context("error loading file")?;

        let mut yaml_content = String::new();
        file.read_to_string(&mut yaml_content)
            .context("error reading file to yaml String")?;

        let res = yaml_content.try_into()?;

        Ok(res)
    }
}

impl TryFrom<String> for ParticipantsData {
    type Error = anyhow::Error;

    /// String should contain valid yaml matching ParticipantsData's structure
    fn try_from(yaml_content: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&yaml_content)
            .context("cannot deserialize yaml data from String")
    }
}
