use climb::{FunctionInput, FunctionOptions, FunctionResult, Command};
use owo_colors::OwoColorize;

use crate::utils::{load_doc, convert_options, FunctionOption};

use std::path::PathBuf;

pub fn new() -> Command {
    Command::new(
        "list",
        "List all crate workspace members.",
    )
}

pub fn run(_: FunctionInput, options: FunctionOptions) -> FunctionResult {
    let opt = ListOptions::from(convert_options(&options));

    let cargo_toml = load_doc(&opt.path).map_err(|e| e.to_string())?;

    let workspace = match cargo_toml.get("workspace") {
        Some(ws) => ws,
        None => return Err("No workspace table found in Cargo.toml".to_string()),
    };

    let members = match workspace.get("members") {
        Some(mbrs) => mbrs,
        None => return Err("No members key found in [workspace] table".to_string()),
    };

    let members_array: Vec<String> = match members.as_array() {
        Some(arr) => arr.into_iter().map(|v| v.as_str().unwrap().to_string()).collect(),
        None => return Err("Members key is not an array".to_string()),
    };

    let workspace_path = opt.path.canonicalize().unwrap();
    let workspace_name = workspace_path.file_name().unwrap().to_str().unwrap();

    if members_array.is_empty() {
        println!("{} {} {}", "Workspace".white().bold(), workspace_name.blue().bold(), "has no members".white().bold());
        return Ok(None);
    };

    println!("{} {}{}", "Workspace", workspace_name.blue().bold(), ":");

    for member in members_array {
        let member_path_binding = opt.path.join(member.clone()).canonicalize().unwrap();
        let member_path = member_path_binding.to_str().unwrap();
        println!("  {} {}{}{}", member.bold(), "(", member_path.cyan(), ")");
    };

    Ok(None)
}

#[derive(Debug)]
pub struct ListOptions {
    pub path: PathBuf,
}

impl From<Vec<FunctionOption>> for ListOptions {
    fn from(value: Vec<FunctionOption>) -> Self {
        let mut path = PathBuf::from(".");

        for option in value {
            match option.0.as_str() {
                "--path" => path = PathBuf::from(option.1.unwrap()).canonicalize().unwrap(),
                _ => (),
            };
        };

        Self { path }
    }
}
