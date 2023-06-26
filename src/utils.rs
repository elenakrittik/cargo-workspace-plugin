use std::path::PathBuf;

use climb::FunctionOption as ClimbFunctionOption;
use toml_edit::Document;

pub fn load_doc(path: &PathBuf) -> anyhow::Result<Document> {
    let cargo_toml = std::fs::read_to_string(path.join("Cargo.toml"))?;
    let doc = cargo_toml.parse::<Document>()?;
    Ok(doc)
}

pub fn save_doc(doc: &Document, path: &PathBuf) -> anyhow::Result<()> {
    let cargo_toml = doc.to_string();
    std::fs::write(path.join("Cargo.toml"), cargo_toml.to_string())?;
    Ok(())
}

pub fn convert_options(options: &Vec<ClimbFunctionOption>) -> Vec<FunctionOption> {
    let converted: Vec<FunctionOption> = options.iter().map(|v| v.into()).collect();
    converted
}

#[derive(Debug)]
pub struct FunctionOption(pub String, pub Option<String>);

impl From<&ClimbFunctionOption> for FunctionOption {
    fn from(value: &ClimbFunctionOption) -> Self {
        Self(value.0.clone(), value.1.clone())
    }
}
