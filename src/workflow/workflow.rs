use std::fmt::Display;
use std::{println as p, eprintln as e};

use ansi_term::Colour;
use similar::{ChangeTag, TextDiff};

use crate::args::{cli, BumpType};
use crate::error::ResultW;
use crate::toml_tools::{self, TomlData, ValidatedPackage};
use crate::output::Output;


pub fn perform_workflow() {
  match workflow() {
    Ok(value) => p!("{value}"),
    Err(error) => e!("{error}"),
  }
}


pub fn workflow() -> ResultW<Output> {
  let args = cli::get_cli_args();

  let toml_file = toml_tools::get_toml_file(args.toml_file);
  p!("Using toml file: {}", toml_file.to_string_lossy());

  let toml_data = toml_tools::get_current_version(&toml_file)?;
  match args.commands {
    cli::WaffleCommands::Get => {
      // We don't convert this to a ValidatePackage as we are just returning the existing version.
      Ok(Output::Version(toml_data.package))
    },

    cli::WaffleCommands::Bump{ major, minor, patch } => {
      let TomlData { package, content } = toml_data;
      let bump_type = BumpType::get_bump_type(major, minor, patch)?;
      let validated_current_version: ValidatedPackage = package.try_into()?;
      let next_version = validated_current_version.bump_version(bump_type);
      let new_content = toml_tools::write_updated_version(toml_file, &content, next_version.clone())?;

      if  args.verbose {
        show_diff(&content, &new_content)
      }

      Ok(Output::Bump(validated_current_version, next_version))
    },

    cli::WaffleCommands::Tag => {
      // We don't convert this to a ValidatePackage as we are just returning the existing version.
      // Output's Display instance will handle writing out the correct String
      Ok(Output::Tag(toml_data.package))
    },
  }
}


fn show_diff(content: &str, new_content: &str) {
    let text_diff = TextDiff::from_lines(content, new_content);
    for change in text_diff.iter_all_changes() {
      match change.tag() {
        ChangeTag::Delete => print_diff("-", Colour::Red, change),
        ChangeTag::Insert => print_diff("+", Colour::Green, change),
        ChangeTag::Equal => (),
      }
    }
}

fn print_diff<T: Display>(prefix: &str, colour: Colour, message: T) {
  print!("  {}{}", colour.paint(prefix), message.to_string())
}
