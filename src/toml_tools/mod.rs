mod toml_serde;
mod toml_funcs;

pub use toml_serde::{CargoToml, Package, ValidatedPackage};
pub use toml_funcs::{get_current_version, load_toml_file};

