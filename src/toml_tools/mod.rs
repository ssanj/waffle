mod toml_serde;
mod toml_funcs;

pub use toml_serde::{CargoToml, Package, ValidatedPackage, TomlData};
pub use toml_funcs::{get_current_version, get_toml_file, write_updated_version};

