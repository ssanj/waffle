use clap::{Parser, Subcommand, ValueEnum};

/// Tasty version information
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
  /// Verbose debug logging
  #[arg(long)]
  pub verbose: bool,

  #[command(subcommand)]
  pub commands: WaffleCommands
}

#[derive(Debug, Clone, Subcommand)]
pub enum WaffleCommands {
    /// Get the current version
    Get,
    /// Bump the current version to the next version. One of Major, Minor or Patch Updates Cargo.toml.
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
    },
    /// Displays command to Git tag current version
    Tag,
}


pub fn get_cli_args() -> Args {
  Args::parse()
}
