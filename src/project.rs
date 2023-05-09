use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub author: String,
    pub java_path: String
}

pub fn generate_project_toml(project: Project) {
    let toml = toml::to_string(&project).unwrap();
    let mut file = File::create("project.toml").unwrap();
    file.write(toml.as_bytes());
}
