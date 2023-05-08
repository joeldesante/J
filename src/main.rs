use clap::{arg, command, Command};

struct JavaProject {
    name: String,
    version: String,
    author: String,
    java_path: String
}

// fn generate_java_project(project_name: String) -> JavaProject {
//    
// }

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
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}