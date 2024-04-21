use toml_edit::DocumentMut;
use std::fs;
use std::path::Path;
use std::{println as p, eprintln as e};
use std::format as s;

use crate::args::cli;
use crate::error::{WaffleError, ResultW};
use crate::toml_tools::{CargoToml, Package, get_current_version};

struct Output(String);

pub fn perform_workflow() {
  let args = cli::get_cli_args();
  let toml_file = Path::new("./Cargo.toml");
  // TODO: Write out Cargo toml being used
  let result = match args.commands {
    cli::WaffleCommands::Get => {
      get_current_version(&toml_file)
        .map(|v| Output(v.version))
    },
    cli::WaffleCommands::Bump => {
      todo!()
    },
    cli::WaffleCommands::Tag => {
      get_current_version(&toml_file)
        .map(|v| Output(s!("git tag 'v{}'", v.version)))
    },
  };

  match result {
    Ok(Output(value)) => p!("{value}"),
    Err(WaffleError::CouldNotReadTomlFile(error)) => e!("{error}"),
    Err(WaffleError::CouldParseTomlFile(error)) => e!("{error}"),
  }
}
