use std::format as s;
use std::fmt;

use crate::wtoml::{Package, ValidatedPackage};

pub enum Output {
  Version(Package),
  Tag(Package),
  Bump(ValidatedPackage, ValidatedPackage)
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let output = match self {
        Output::Version(Package { version }) => version.to_owned(),
        Output::Tag(Package { version }) => s!("git tag v{}", version),
        Output::Bump(before, after) => s!("Updated version from: {before} -> {after}"),
      };

      write!(f, "{output}")
    }
}
