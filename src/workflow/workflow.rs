use std::path::Path;
use std::{format as s, println as p, eprintln as e};

use crate::args::{cli, BumpType};
use crate::error::{WaffleError, ResultW};
use crate::toml_tools::{self, ValidatedPackage};
use crate::output::Output;


pub fn perform_workflow() {
  match workflow() {
    Ok(value) => p!("{value}"),
    Err(error) => e!("{error}"),
  }
}


pub fn workflow() -> ResultW<Output> {
  let args = cli::get_cli_args();
  // TODO: Supplied file or default to Cargo.toml
  let toml_file = Path::new("./Cargo.toml");
  // TODO: Write out Cargo toml being used
  let current_version = toml_tools::get_current_version(&toml_file)?;
  match args.commands {
    cli::WaffleCommands::Get => {
      // We don't convert this to a ValidatePackage as we are just returning the existing version.
      Ok(Output::Version(current_version))
    },

    cli::WaffleCommands::Bump{ major, minor, patch } => {
      let bump_type = get_bump_type(major, minor, patch)?;
      let validated_current_version: ValidatedPackage = current_version.try_into()?;
      let next_version = bump_version(validated_current_version, bump_type);
      Ok(Output::Bump(next_version))
    },

    cli::WaffleCommands::Tag => {
      // We don't convert this to a ValidatePackage as we are just returning the existing version.
      // Output's Display instance will handle writing out the correct String
      Ok(Output::Tag(current_version))
    },
  }
}

// TODO: Move bump-related functionality into a separate module
fn get_bump_type(major: bool, minor: bool, patch: bool) -> ResultW<BumpType> {
    let set_flags =
      vec![major, minor, patch]
        .into_iter()
        .filter(|v| *v)
        .count();

    if set_flags > 1 {
      Err(WaffleError::InvalidBumpCombination)
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

      Ok(bump_type)
    }
}


fn bump_version(current_version: ValidatedPackage, bump_type: BumpType) -> ValidatedPackage {
  let ValidatedPackage { major, minor, patch } = current_version;
  let (next_major, next_minor, next_patch) = match bump_type {
    BumpType::Major => (major + 1, 0_u16, 0_u16),
    BumpType::Minor => (major, minor + 1, 0_u16),
    BumpType::Patch => (major, minor, patch + 1),
  };

  ValidatedPackage {
    major: next_major,
    minor: next_minor,
    patch: next_patch,
  }
}
