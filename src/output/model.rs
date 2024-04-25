use std::format as s;
use std::fmt;

use crate::toml_tools::{Package, ValidatedPackage};

pub enum Output {
  Version(Package),
  Tag(Package),
  // TODO: Add before and after versions here
  Bump(ValidatedPackage)
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let output = match self {
        Output::Version(Package { version }) => version.to_owned(),
        Output::Tag(Package { version }) => s!("git tag v{}", version),
        // TODO: Add before and after versions here
        Output::Bump(ValidatedPackage { major, minor, patch }) => s!("updated to: {major}.{minor}.{patch}"),
      };

      write!(f, "{output}")
    }
}
