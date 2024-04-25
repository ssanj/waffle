use std::path::{Path, PathBuf};
use std::fmt;
use std::format as s;

use crate::toml_tools::Package;

pub type ResultW<T> = Result<T, WaffleError>;

#[derive(Debug, PartialEq)]
pub struct FileName(PathBuf);

impl FileName {

  pub fn new(path: &Path) -> Self {
    Self(path.to_owned())
  }
}

impl fmt::Display for FileName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0.clone().into_os_string().to_string_lossy())
  }
}

#[derive(Debug, PartialEq)]
pub enum WaffleError {
  CouldNotReadTomlFile(FileName, String),
  CouldParseTomlFile(FileName, String),
  TooManyBumpCombinations,
  NoBumpCombinations,
  NonNumericVersions(Package),
  NotSemver(Package),
}


impl fmt::Display for WaffleError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let result = match self {
      WaffleError::CouldNotReadTomlFile(filename, error) => s!("Could not read toml file: {filename}, due to error: {error}"),

      WaffleError::CouldParseTomlFile(filename, error) => s!("Could not parse toml file: {filename}, due to error: {error}"),

      WaffleError::TooManyBumpCombinations => "Only one of Major, Minor or Patch is allowed".to_owned(),

      WaffleError::NoBumpCombinations => "At least one of Major, Minor or Patch is required".to_owned(),

      WaffleError::NonNumericVersions(package) => s!("Toml package.version: {package} is not numeric"),

      WaffleError::NotSemver(package) => s!("Toml package.version: {package} is not a valid Semantic Version with format: major.minor.patch"),
    };

    write!(f, "{}", result)
  }
}
