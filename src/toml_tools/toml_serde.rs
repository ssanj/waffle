use crate::error::WaffleError;
use crate::args::BumpType;
use std::str::FromStr;
use std::fmt;

#[derive(serde::Deserialize)]
pub struct CargoToml {
  pub package: Package
}

#[derive(Debug, PartialEq, serde::Deserialize, Clone)]
pub struct Package {
  pub version: String
}

impl Package {

  #[cfg(test)]
  fn new(version: &str) -> Self {
    Self {
      version: version.to_owned()
    }
  }

}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Package({})", self.version)
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedPackage {
  pub major: u16,
  pub minor: u16,
  pub patch: u16,
}


impl ValidatedPackage {

  #[cfg(test)]
  fn new(version: &str) -> Self {
    let parts: Vec<_> =
      version
        .splitn(3, ".")
        .map(|num| u16::from_str(num).unwrap())
        .collect();

    let (major, minor, patch) = (parts[0], parts[1], parts[2]);
    Self {
      major,
      minor,
      patch
    }
  }

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

#[cfg(test)]
mod tests {

    // TODO: Try out a fuzzer
    // TODO: Move this out of toml_tools

    mod validatedpackage {

      mod bump {
        use super::super::super::ValidatedPackage;
        use crate::args::BumpType;
        use pretty_assertions::assert_eq;

        #[test]
        fn patch_version() {
          assert_bump_version("1.2.3", BumpType::Patch, "1.2.4");
        }

        #[test]
        fn minor_version() {
          assert_bump_version("1.2.3", BumpType::Minor, "1.3.0");
        }

        #[test]
        fn major_version() {
          assert_bump_version("1.2.3", BumpType::Major, "2.0.0");
        }

        fn assert_bump_version(version: &str, bump_type: BumpType, expected_version: &str) {
          let package = ValidatedPackage::new(version);
          let bumped_package = package.bump_version(bump_type);
          let expected_bumped_package = ValidatedPackage::new(expected_version);

          assert_eq!(bumped_package, expected_bumped_package)
        }
      }

      mod try_from {
        use crate::error::{ResultW, WaffleError};

        use super::super::super::{Package, ValidatedPackage};
        use pretty_assertions::assert_eq;

        #[test]
        fn valid_package_conversion() {
          let package = Package::new("1.2.3");

          let validated_package: ValidatedPackage = package.try_into().unwrap();
          let expected_package = ValidatedPackage::new("1.2.3");

          assert_eq!(validated_package, expected_package)
        }

        #[test]
        fn non_semver_package_conversion() {
          let package = Package::new("1.2");

          let validated_package_result: ResultW<ValidatedPackage> = package.clone().try_into();
          let expected_package_error = Err(WaffleError::NotSemver(package));

          assert_eq!(validated_package_result, expected_package_error)
        }

        #[test]
        fn non_numeric_package_conversion() {
          let package = Package::new("1.abc.3");

          let validated_package_result: ResultW<ValidatedPackage> = package.clone().try_into();
          let expected_package_error = Err(WaffleError::NonNumericVersions(package));

          assert_eq!(validated_package_result, expected_package_error)
        }
      }
    }
}
