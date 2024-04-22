use crate::error::WaffleError;
use std::str::FromStr;

#[derive(serde::Deserialize)]
pub struct CargoToml {
  pub package: Package
}


#[derive(serde::Deserialize)]
pub struct Package {
  pub version: String
}


pub struct ValidatedPackage {
  pub major: u16,
  pub minor: u16,
  pub patch: u16,
}

impl TryFrom<Package> for ValidatedPackage {
    type Error = WaffleError;

    fn try_from(package: Package) -> Result<Self, Self::Error> {

      let u16_parts = package.version
        .split(".")
        .map(|v| u16::from_str(v).map_err(|e| WaffleError::UnsupportedVersions("".to_owned())))
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
        _ => Err(WaffleError::UnsupportedVersions("".to_owned())),
      }
    }
}
