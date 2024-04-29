
mod args;
mod error;
mod toml_tools;
mod workflow;
mod output;
mod diff;

fn main() {
  crate::workflow::perform_workflow()
}
