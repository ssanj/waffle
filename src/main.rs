
mod args;
mod error;
mod toml_tools;
mod workflow;
mod output;

fn main() {
  crate::workflow::perform_workflow()
}
