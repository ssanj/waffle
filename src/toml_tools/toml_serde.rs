#[derive(serde::Deserialize)]
pub struct CargoToml {
  pub package: Package
}

#[derive(serde::Deserialize)]
pub struct Package {
  pub version: String
}
