use std::path::PathBuf;

use dialoguer::{theme::ColorfulTheme, Select};
use crate::utils::{load_doc, save_doc, FunctionOption, convert_options};
use climb::{FunctionInput, FunctionOptions, FunctionResult};

pub fn run(_: FunctionInput, options: FunctionOptions) -> FunctionResult {
    let opt = CreateOptions::from(convert_options(&options));

    Ok(None)
}


#[derive(Debug)]
pub struct CreateOptions {
    pub path: PathBuf,
}

impl From<Vec<FunctionOption>> for CreateOptions {
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