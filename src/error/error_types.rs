use std::path::{Path, PathBuf};
use std::fmt;
use std::format as s;

use crate::wtoml::Package;

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
pub struct TomlContent(String);

impl TomlContent {

  pub fn new(content: &str) -> Self {
    Self(content.to_owned())
  }
}


impl fmt::Display for TomlContent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, PartialEq)]
pub enum WaffleError {
  CouldNotReadTomlFile(FileName, String),
  CouldNotUpdateTomlFile(FileName, TomlContent, String),
  CouldParseTomlFile(FileName, String),
  CouldConvertTomlContentToDocument(FileName, TomlContent, String),
  TooManyBumpCombinations,
  NoBumpCombinations,
  NonNumericVersions(Package),
  NotSemver(Package),
}


impl fmt::Display for WaffleError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let result = match self {
      WaffleError::CouldNotReadTomlFile(filename, error) => s!("Could not read Toml file: {filename}, due to error: {error}"),

      WaffleError::CouldNotUpdateTomlFile(filename, content, error) => s!("Could not update Toml file: {filename}, with content:\n{content} due to error:\n{error}"),

      WaffleError::CouldParseTomlFile(filename, error) => s!("Could not parse Toml file: {filename}, due to error: {error}"),

      WaffleError::CouldConvertTomlContentToDocument(filename, content, error) => s!("Could not parse Toml file: {filename} into Toml document. \nContent: {content}\nerror: {error}"),

      WaffleError::TooManyBumpCombinations => "Only one of Major, Minor or Patch is allowed. Supply a single bump type.".to_owned(),

      WaffleError::NoBumpCombinations => "At least one of Major, Minor or Patch is required".to_owned(),

      WaffleError::NonNumericVersions(package) => s!("Toml package.version: {package} is not numeric. Waffle only supports Semantic Versioning."),

      WaffleError::NotSemver(package) => s!("Toml package.version: {package} is not a valid Semantic Version with format: major.minor.patch. Waffle only supports Semantic Versioning."),
    };

    write!(f, "{}", result)
  }
}
