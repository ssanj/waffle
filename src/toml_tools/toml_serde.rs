use crate::error::WaffleError;
use crate::args::BumpType;
use std::str::FromStr;
use std::fmt;

#[derive(serde::Deserialize)]
pub struct CargoToml {
  pub package: Package
}


#[derive(serde::Deserialize, Clone)]
pub struct Package {
  pub version: String
}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Package({})", self.version)
  }
}


pub struct ValidatedPackage {
  pub major: u16,
  pub minor: u16,
  pub patch: u16,
}


impl ValidatedPackage {
  pub fn bump_version(&self, bump_type: BumpType) -> ValidatedPackage {
    let (next_major, next_minor, next_patch) = match bump_type {
      BumpType::Major => (self.major + 1, 0_u16, 0_u16),
      BumpType::Minor => (self.major, self.minor + 1, 0_u16),
      BumpType::Patch => (self.major, self.minor, self.patch + 1),
    };

    ValidatedPackage {
      major: next_major,
      minor: next_minor,
      patch: next_patch,
    }
  }
}

impl TryFrom<Package> for ValidatedPackage {
    type Error = WaffleError;

    fn try_from(package: Package) -> Result<Self, Self::Error> {

      let u16_parts =
        package
          .clone()
          .version
          .split(".")
          .map(|v| u16::from_str(v)
          .map_err(|_| WaffleError::NonNumericVersions(package.clone())))
          .collect::<Result<Vec<u16>, Self::Error>>()?;

      match u16_parts[..] {
        [major, minor, patch] => {
          let validated =
            ValidatedPackage {
              major,
              minor,
              patch
            };

            Ok(validated)
        },
        _ => Err(WaffleError::NotSemver(package)),
      }
    }
}
