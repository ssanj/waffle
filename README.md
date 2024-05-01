# Waffle

Tasty version information from your Cargo.toml file.

Waffle lets you do the following:

1. Get the current version of your Rust project
1. Bump the current version by major, minor or patch increments
1. Gets the Git tag command for the current version of your Rust project

## Installation

### Building through Cargo

You can build Waffle through Cargo with:

```
cargo install --git https://github.com/ssanj/waffle
```

This will install Waffle into your Cargo home directory; usually `~/.cargo/bin`.

### Building from Source

Ensure you have Cargo installed.

Run:

```
cargo build --release
Copy binary file from target/release/waffle to a directory on your PATH.
```


## Usage

### General information

`waffle -h`:

```
Tasty version information

Usage: waffle [OPTIONS] <COMMAND>

Commands:
  get   Get the current version
  bump  Bump the current version to the next version. One of Major, Minor or Patch Updates Cargo.toml
  tag   Displays command to Git tag current version
  help  Print this message or the help of the given subcommand(s)

Options:
      --verbose                Verbose debug logging
      --toml-file <TOML_FILE>  Location of toml file. If not specified defaults to Cargo.toml in the current directory
  -h, --help                   Print help
  -V, --version                Print version
```

### Bumping

`waffle bump -h`:

```
Bump the current project version to the next version. One of Major, Minor or Patch

Usage: waffle bump [OPTIONS]

Options:
  -M             Major
  -m             Minor
  -p             Patch
      --no-diff  Hide change diff
  -h, --help     Print help
```

### Getting the current version

To get the current version of a Rust project, either run `waffle` from your project directory or supply the location of a `Cargo.toml` file.

```
waffle get
```

or

```
waffle --toml-file <LOCATION_OF_TOML_FILE> get
```

Example output:

```
Using toml file: ./Cargo.toml
0.2.4
```


### Bump current version

You can bump the current version by major, minor or patch increments.

To perform a major version bump:
```
waffle bump -M
```

To perform a minor version bump:
```
waffle bump -m
```

To perform a patch version bump:
```
waffle bump -p
```

Example output:

```
Using toml file: ./Cargo.toml
  -version = "0.2.4"
  +version = "0.2.5"
Updated version from: 0.2.4 -> 0.2.5
```

### Git tag for current version

To get the Git tag command to tag the current version of your Rust project.

```
waffle tag
```

Example output would be:

```
git tag 'v<MAJOR.MINOR.PATCH>'
```
