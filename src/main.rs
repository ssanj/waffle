
mod args;
mod error;
mod wtoml;
mod workflow;
mod output;
mod diff;

fn main() {
  crate::workflow::perform_workflow()
}
