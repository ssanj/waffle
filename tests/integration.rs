use ansi_term::Colour;
use assert_cmd::Command;
use predicates::{function::FnPredicate, prelude::predicate};
use std::{fmt, format as s, path::PathBuf, println as p};
use tempfile::{tempdir, TempDir};

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


#[test]
fn get_current_package_version() {
  let working_dir = tempdir().unwrap();
  let (sample_toml_file, mut cmd) = setup_test(&working_dir);

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
  let (sample_toml_file, mut cmd) = setup_test(&working_dir);

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
  assert_version_bump("2.0.0", BumpType::Major)
}


#[test]
fn bump_minor_version() {
  assert_version_bump("1.3.0", BumpType::Minor)
}


#[test]
fn bump_patch_version() {
  assert_version_bump("1.2.4", BumpType::Patch)
}


#[test]
fn bump_current_package_version_without_diff() {
  let working_dir = tempdir().unwrap();
  let (sample_toml_file, mut cmd) = setup_test(&working_dir);

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

// ---------------------------------------------------------------------------------------------------------------------
// Test Helpers
// ---------------------------------------------------------------------------------------------------------------------

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


enum BumpType {
  Major,
  Minor,
  Patch
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


fn setup_test<'a>(working_dir: &TempDir) -> (PathBuf, Command) {
  let sample_toml_file = working_dir.path().join("Sample.toml");
  std::fs::write(&sample_toml_file, SAMPLE_TOML_CONTENT).unwrap();
  println!("{}", &sample_toml_file.as_path().to_string_lossy());
  let cmd = Command::cargo_bin("waffle").unwrap();
  (sample_toml_file, cmd)
}


fn assert_version_bump(new_version: &str, bump_type: BumpType) {
  let working_dir = tempdir().unwrap();
  let (sample_toml_file, mut cmd) = setup_test(&working_dir);

  let expected_version_string =
    [
      s!("Updated version from: 1.2.3 -> {new_version}"),
      s!("{}version = \"1.2.3\"", Colour::Red.paint("-")),
      s!("{}version = \"{new_version}\"", Colour::Green.paint("+")),
    ];

  let expected_comparisons: Vec<_> =
    expected_version_string
      .iter()
      .map(|v| ComparisonType::Contains(v))
      .collect();

  cmd
    .arg("--toml-file")
    .arg(&sample_toml_file)
    .arg("bump");

  match bump_type {
    BumpType::Major => cmd.arg("-M"),
    BumpType::Minor => cmd.arg("-m"),
    BumpType::Patch => cmd.arg("-p"),
  };

  cmd
    .assert()
    .success()
    .stdout(std_out_comparison(&expected_comparisons));
}
