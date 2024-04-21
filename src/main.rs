use args::cli;
use toml_edit::DocumentMut;
use std::fs;
use std::path::Path;
use std::{println as p, eprintln as e};
use std::format as s;

mod args;

fn main() {
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


struct Output(String);

type ResultW<T> = Result<T, WaffleError>;

enum WaffleError {
  CouldNotReadTomlFile(String),
  CouldParseTomlFile(String),
}

#[derive(serde::Deserialize)]
struct CargoToml {
  package: Package
}

#[derive(serde::Deserialize)]
struct Package {
  version: String
}

fn get_current_version(file_name: &Path) -> ResultW<Package> {
  let toml_content = load_toml_file(file_name)?;
  let toml_struct: CargoToml =
    toml
      ::from_str(&toml_content)
      .map_err(|e| WaffleError::CouldParseTomlFile(e.to_string()))?;

  Ok(toml_struct.package)
}

fn load_toml_file(file_name: &Path) -> ResultW<String> {
  fs
    ::read_to_string(file_name)
    .map_err(|e| WaffleError::CouldNotReadTomlFile(e.to_string()))
}
