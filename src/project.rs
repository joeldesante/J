use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub details: Details,
    pub dependancies: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
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

/*pub fn load_project_toml() -> Project {

    let project_file = File::open("project.toml");

    Project {
        name: "String".to_string(),
        version: "String".to_string(),
        author: "String".to_string(),
        java_path: "String".to_string()
    };
}*/
