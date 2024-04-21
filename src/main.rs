mod args;
mod error;
mod toml_tools;
mod workflow;

fn main() {
  crate::workflow::perform_workflow();
}
