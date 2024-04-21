pub type ResultW<T> = Result<T, WaffleError>;

pub enum WaffleError {
  CouldNotReadTomlFile(String),
  CouldParseTomlFile(String),
}
