use clap::{Parser, Subcommand};

/// Tasty version information
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
  #[command(subcommand)]
  pub commands: WaffleCommands,

  /// Location of toml file. If not specified defaults to Cargo.toml in the current directory
  #[arg(long)]
  pub toml_file: Option<String>
}

#[derive(Debug, Clone, Subcommand)]
pub enum WaffleCommands {
    /// Get the project current version
    Get,
    /// Bump the current project version to the next version. One of Major, Minor or Patch.
    Bump {
      /// Major
      #[arg(short = 'M')]
      major: bool,

      /// Minor
      #[arg(short)]
      minor: bool,

      /// Patch
      #[arg(short)]
      patch: bool,

      /// Hide change diff
      #[arg(long)]
      no_diff: bool,

    },
    /// Displays command to Git tag current project version
    Tag,
}


pub fn get_cli_args() -> Args {
  Args::parse()
}
