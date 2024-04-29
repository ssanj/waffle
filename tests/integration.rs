use ansi_term::Colour;
use assert_cmd::Command;
use predicates::{function::FnPredicate, prelude::predicate};
use std::{format as s, println as p};
use tempfile::tempdir;

#[test]
fn returns_version() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("waffle").unwrap();

  let version = env!("CARGO_PKG_VERSION");
  let expected_version_string = s!("waffle {}\n", version);

  cmd
    .arg("-V")
    .assert()
    .success()
    .stdout(expected_version_string);

  Ok(())
}

const sample_toml_content: &str = r#"
[package]
name = "Sample"
version = "1.2.3"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
serde = { version = "1", features = ["derive"]}
serde_json = "1"
clap = { version = "4", features = ["derive", "cargo", "env", "unicode"] }
ansi_term = "0.12"

[dev-dependencies]
pretty_assertions = "1"
assert_cmd = "2"
"#;


fn std_out_contains(expected: &str) -> FnPredicate<impl Fn(&[u8]) -> bool, [u8]> {
    let owned_expected = expected.to_owned();
    predicate::function(move |out: &[u8]| {
      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");
      let error = s!("Could not validate stdout contains: {}", &owned_expected);
      p!("{}", Colour::Red.paint(&error));
      p!("{}", "-".repeat(error.len()));
      output.contains(&owned_expected)
    })
  }


#[test]
fn gets_current_package_version() {
  let working_dir = tempdir().unwrap();
  let sample_toml_file = working_dir.path().join("Sample.toml");
  std::fs::write(&sample_toml_file, sample_toml_content).unwrap();
  println!("{}", &sample_toml_file.as_path().to_string_lossy());
  let mut cmd = Command::cargo_bin("waffle").unwrap();
  let expected_version_string = "1.2.3";

  cmd
    .arg("--toml-file")
    .arg(&sample_toml_file)
    .arg("get")
    .assert()
    .success()
    .stdout(std_out_contains(expected_version_string));
}
