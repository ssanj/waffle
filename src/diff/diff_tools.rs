use std::fmt::Display;
use ansi_term::Colour;
use similar::{ChangeTag, TextDiff};


pub fn show_diff(content: &str, new_content: &str) {
    let text_diff = TextDiff::from_lines(content, new_content);
    for change in text_diff.iter_all_changes() {
      match change.tag() {
        ChangeTag::Delete => print_diff("-", Colour::Red, change),
        ChangeTag::Insert => print_diff("+", Colour::Green, change),
        ChangeTag::Equal => (),
      }
    }
}

fn print_diff<T: Display>(prefix: &str, colour: Colour, message: T) {
  print!("  {}{}", colour.paint(prefix), message.to_string())
}
