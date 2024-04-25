use std::path::{Path, PathBuf};
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

  let toml_file = get_toml_file(args.toml_file);
  p!("Using toml file: {}", toml_file.to_string_lossy());

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

fn get_toml_file(toml_file_arg: Option<String>) -> PathBuf {
  let default_toml_file = PathBuf::from("./Cargo.toml");
  toml_file_arg
    .map_or_else(|| default_toml_file, |tf| PathBuf::from(tf))
}
