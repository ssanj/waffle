use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{value, DocumentMut};

use crate::error::{FileName, ResultW, TomlContent, WaffleError};
use crate::toml_tools::CargoToml;
use super::{TomlData, ValidatedPackage};

pub fn get_current_version(file_name: &Path) -> ResultW<TomlData> {
  let toml_content = load_toml_file(file_name)?;
  let toml_struct: CargoToml =
    toml
      ::from_str(&toml_content)
      .map_err(|e| WaffleError::CouldParseTomlFile(FileName::new(file_name), e.to_string()))?;

  let toml_data =
    TomlData {
      package: toml_struct.package,
      content: toml_content
    };

  Ok(toml_data)
}


pub fn load_toml_file(file_name: &Path) -> ResultW<String> {
  fs
    ::read_to_string(file_name)
    .map_err(|e| WaffleError::CouldNotReadTomlFile(FileName::new(file_name), e.to_string()))
}


pub fn get_toml_file(toml_file_arg: Option<String>) -> PathBuf {
  let default_toml_file = PathBuf::from("./Cargo.toml");
  toml_file_arg
    .map_or_else(|| default_toml_file, PathBuf::from)
}


pub fn write_updated_version<P: AsRef<Path>>(toml_file: P, toml_content: &str, next_version: ValidatedPackage) -> ResultW<String> {

  let updated_toml = update_toml(&toml_file, toml_content, next_version)?;
  let new_toml_content = updated_toml.to_string();
  write_toml_file(toml_file, &new_toml_content)?;

  Ok(new_toml_content)
}


pub fn update_toml<P: AsRef<Path>>(toml_file: P, toml_content: &str, next_version: ValidatedPackage) -> ResultW<DocumentMut> {
  let mut doc =
    toml_content.parse::<DocumentMut>()
    .map_err(|e| WaffleError::CouldConvertTomlContentToDocument(FileName::new(toml_file.as_ref()), TomlContent::new(toml_content), e.to_string()))?;

  doc["package"]["version"] = value(next_version.clone());

  Ok(doc)
}


fn write_toml_file<P: AsRef<Path>>(toml_file: P, content: &str) -> ResultW<()> {
  std::fs::write(toml_file.as_ref(), content)
    .map_err(|e| WaffleError::CouldNotUpdateTomlFile(FileName::new(toml_file.as_ref()), TomlContent::new(content), e.to_string()))
}
