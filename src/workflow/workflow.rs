use std::path::Path;
use std::{format as s, println as p, eprintln as e};

use crate::args::{cli, BumpType};
use crate::error::{WaffleError, ResultW};
use crate::toml_tools::{self, Package, ValidatedPackage};
use crate::output::Output;


pub fn perform_workflow() {
  match workflow() {
    Ok(value) => p!("{value}"),
    Err(WaffleError::CouldNotReadTomlFile(error)) => e!("{error}"),
    Err(WaffleError::CouldParseTomlFile(error)) => e!("{error}"),
    Err(WaffleError::InvalidBumpCombination(error)) => e!("{error}"),
    Err(WaffleError::UnsupportedVersions(error)) => e!("{error}"),
  }
}


pub fn workflow() -> ResultW<Output> {
  let args = cli::get_cli_args();
  let toml_file = Path::new("./Cargo.toml");
  // TODO: Write out Cargo toml being used
  // TODO: Support a supplied toml path
  let current_version = toml_tools::get_current_version(&toml_file)?;
  match args.commands {
    cli::WaffleCommands::Get => {
      Ok(Output::Version(current_version))
    },
    cli::WaffleCommands::Bump{ major, minor, patch} => {
      let set_flags =
        vec![major, minor, patch]
          .into_iter()
          .filter(|v| *v)
          .count();

      if set_flags > 1 {
        Err(WaffleError::InvalidBumpCombination("Only one of Major, Minor or Patch is allowed".to_owned()))
      } else {
        let bump_type = {
          if major {
            BumpType::Major
          } else if minor {
            BumpType::Minor
          } else {
            BumpType::Patch
          }
        };

        let validated_current_version: ValidatedPackage = current_version.try_into()?;
        let next_version = bump_version(validated_current_version, bump_type);
        Ok(Output::Bump(next_version))
      }
    },
    cli::WaffleCommands::Tag => {
      // Output's Display instance will handle writing out the correct String
      Ok(Output::Tag(current_version))
    },
  }
}


fn bump_version(current_version: ValidatedPackage, bump_type: BumpType) -> Package {

  match (&current_version.major, &current_version.minor, &current_version.patch) {
    (major, minor, patch) => {
      let next_version = match bump_type {
        BumpType::Major => s!("{}.0.0", major + 1),
        BumpType::Minor => s!("{}.{}.0", major, minor + 1),
        BumpType::Patch => s!("{}.{}.{}", major, minor, patch + 1),
      };

      Package {
        version: next_version
      }
    }
  }
}
