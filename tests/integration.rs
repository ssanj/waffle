use ansi_term::Colour;
use assert_cmd::Command;
use predicates::{function::FnPredicate, prelude::predicate};
use std::{format as s, println as p, fmt};
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

const SAMPLE_TOML_CONTENT: &str = r#"
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

#[derive(Debug, Clone)]
enum ComparisonType<'a> {
  Contains(&'a str),
  DoesNotContain(&'a str)
}

impl fmt::Display for ComparisonType<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}


fn std_out_comparison<'a>(expected: &'a [ComparisonType<'a>]) -> FnPredicate<impl Fn(&[u8]) -> bool + 'a, [u8]> {
    predicate::function(move |out: &[u8]| {

    let expected_values: Vec<_> =
      expected
        .iter()
        .cloned()
        .collect();

      let output = std::str::from_utf8(out).expect("Could not convert stdout to string");

      expected_values.into_iter().all(|comparison| {
        let error = s!("Could not validate stdout comparison: {}", comparison);
        p!("{}", Colour::Red.paint(&error));
        p!("{}", "-".repeat(error.len()));
        match comparison {
          ComparisonType::Contains(value) => output.contains(value),
          ComparisonType::DoesNotContain(value) => !output.contains(value),
        }
      })
    })
}


#[test]
fn get_current_package_version() {
  let working_dir = tempdir().unwrap();
  let sample_toml_file = working_dir.path().join("Sample.toml");
  std::fs::write(&sample_toml_file, SAMPLE_TOML_CONTENT).unwrap();
  println!("{}", &sample_toml_file.as_path().to_string_lossy());
  let mut cmd = Command::cargo_bin("waffle").unwrap();
  let expected_version_string = [ComparisonType::Contains("1.2.3")];

  cmd
    .arg("--toml-file")
    .arg(&sample_toml_file)
    .arg("get")
    .assert()
    .success()
    .stdout(std_out_comparison(&expected_version_string));
}


#[test]
fn tag_current_package_version() {
  let working_dir = tempdir().unwrap();
  let sample_toml_file = working_dir.path().join("Sample.toml");
  std::fs::write(&sample_toml_file, SAMPLE_TOML_CONTENT).unwrap();
  println!("{}", &sample_toml_file.as_path().to_string_lossy());
  let mut cmd = Command::cargo_bin("waffle").unwrap();
  let expected_version_string = [ComparisonType::Contains("git tag v1.2.3")];

  cmd
    .arg("--toml-file")
    .arg(&sample_toml_file)
    .arg("tag")
    .assert()
    .success()
    .stdout(std_out_comparison(&expected_version_string));
}


#[test]
fn bump_major_version() {
  let working_dir = tempdir().unwrap();
  let sample_toml_file = working_dir.path().join("Sample.toml");
  std::fs::write(&sample_toml_file, SAMPLE_TOML_CONTENT).unwrap();
  println!("{}", &sample_toml_file.as_path().to_string_lossy());
  let mut cmd = Command::cargo_bin("waffle").unwrap();
  let expected_version_string =
    [
      "Updated version from: 1.2.3 -> 2.0.0",
      &s!("{}version = \"1.2.3\"", Colour::Red.paint("-")),
      &s!("{}version = \"2.0.0\"", Colour::Green.paint("+")),
    ];

  let expected_comparisons: Vec<_> =
    expected_version_string
      .into_iter()
      .map(ComparisonType::Contains)
      .collect();

  cmd
    .arg("--toml-file")
    .arg(&sample_toml_file)
    .arg("bump")
    .arg("-M")
    .assert()
    .success()
    .stdout(std_out_comparison(&expected_comparisons));
}


#[test]
fn bump_current_package_version_without_diff() {
  let working_dir = tempdir().unwrap();
  let sample_toml_file = working_dir.path().join("Sample.toml");
  std::fs::write(&sample_toml_file, SAMPLE_TOML_CONTENT).unwrap();
  println!("{}", &sample_toml_file.as_path().to_string_lossy());
  let mut cmd = Command::cargo_bin("waffle").unwrap();

  let old_version_diff = s!("{}version = \"1.2.3\"", Colour::Red.paint("-"));
  let new_version_diff = s!("{}version = \"2.0.0\"", Colour::Green.paint("+"));
  let expected_version_string = "Updated version from: 1.2.3 -> 2.0.0";

  let expected_comparisons =
    [
      ComparisonType::Contains(expected_version_string),
      ComparisonType::DoesNotContain(&old_version_diff),
      ComparisonType::DoesNotContain(&new_version_diff),
    ];

  cmd
    .arg("--toml-file")
    .arg(&sample_toml_file)
    .arg("bump")
    .arg("-M")
    .arg("--no-diff")
    .assert()
    .success()
    .stdout(std_out_comparison(&expected_comparisons));
}
