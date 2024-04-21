use std::fs;
use std::path::Path;
use crate::error::{WaffleError, ResultW};
use crate::toml_tools::{CargoToml, Package};


pub fn get_current_version(file_name: &Path) -> ResultW<Package> {
  let toml_content = load_toml_file(file_name)?;
  let toml_struct: CargoToml =
    toml
      ::from_str(&toml_content)
      .map_err(|e| WaffleError::CouldParseTomlFile(e.to_string()))?;

  Ok(toml_struct.package)
}

pub fn load_toml_file(file_name: &Path) -> ResultW<String> {
  fs
    ::read_to_string(file_name)
    .map_err(|e| WaffleError::CouldNotReadTomlFile(e.to_string()))
}
