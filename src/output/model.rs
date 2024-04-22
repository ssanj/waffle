use std::format as s;
use std::fmt;

use crate::toml_tools::Package;

pub enum Output {
  Version(Package),
  Tag(Package),
  Bump(Package)
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let output = match self {
        Output::Version(Package { version }) => version.to_owned(),
        Output::Tag(Package { version }) => s!("git tag v{}", version),
        Output::Bump(Package { version }) => version.to_owned(),
      };

      write!(f, "{output}")
    }
}
