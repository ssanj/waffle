use std::fs;
use std::path::{Path, PathBuf};
use crate::error::{WaffleError, ResultW, FileName};
use crate::toml_tools::{CargoToml, Package};


pub fn get_current_version(file_name: &Path) -> ResultW<Package> {
  let toml_content = load_toml_file(file_name)?;
  let toml_struct: CargoToml =
    toml
      ::from_str(&toml_content)
      .map_err(|e| WaffleError::CouldParseTomlFile(FileName::new(file_name), e.to_string()))?;

  Ok(toml_struct.package)
}


pub fn load_toml_file(file_name: &Path) -> ResultW<String> {
  fs
    ::read_to_string(file_name)
    .map_err(|e| WaffleError::CouldNotReadTomlFile(FileName::new(file_name), e.to_string()))
}


pub fn get_toml_file(toml_file_arg: Option<String>) -> PathBuf {
  let default_toml_file = PathBuf::from("./Cargo.toml");
  toml_file_arg
    .map_or_else(|| default_toml_file, |tf| PathBuf::from(tf))
}
