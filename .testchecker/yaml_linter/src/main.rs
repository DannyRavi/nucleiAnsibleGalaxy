use std::fs;
use std::process;
use serde_yaml::{Value, Error};

fn lint_yaml_file(file_path: &str) -> Result<(), String> {
    // Read the YAML file
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    // Parse the YAML file
    match serde_yaml::from_str::<Value>(&content) {
        Ok(_) => Ok(()),
        Err(err) => Err(format_yaml_error(err, file_path)),
    }
}

fn format_yaml_error(err: Error, file_path: &str) -> String {
    let location = err.location().map_or("unknown location".to_string(), |loc| {
        format!("line {}, column {}", loc.line(), loc.column())
    });
    format!("Error in file '{}', {}: {}", file_path, location, err)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: yaml_linter <path_to_yaml_file>");
        process::exit(1);
    }

    let file_path = &args[1];

    match lint_yaml_file(file_path) {
        Ok(_) => {
            println!("YAML file '{}' is valid.", file_path);
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
