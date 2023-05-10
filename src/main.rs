use clap::{arg, command, Command};
use crate::project::*;

// fn generate_java_project(project_name: String) -> JavaProject {
//    
// }

mod project;

fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Initializes a new Java project in the current directory.")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("add")
                .about("Adds a dependacy to the project.")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("build")
                .about("Builds the Java project.")
                .arg(arg!([NAME])),
        )
        .get_matches();

    
    
    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            let process = match std::process::Command::new("ls").spawn() {
                Ok(process) => process,
                Err(err) => panic!("Running process error: {}", err),
            };
        },
        Some(("init", sub_matches)) => {
            println!("Init!");
            
            let project = Project {
                details: Details {
                    name: "String".to_string(),
                    version: "String".to_string(),
                    author: "String".to_string(),
                    java_path: "String".to_string()
                },
                dependancies: Vec::new()
            };
            
            generate_project_toml(project)
        },
        Some(("add", sub_matches)) => {
            println!("Add Dep!");
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}