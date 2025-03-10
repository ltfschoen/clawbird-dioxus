//! This crate contains all shared fullstack server functions.
use dioxus::{html::input, prelude::*};
pub mod file;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
  Ok(input)
}

/// Get file contents based on client-side user input on the server.
#[server(File)]
pub async fn get_file_contents(input: String) -> Result<String, ServerFnError> {
  let contents = file::file::get_file_contents(input).await?;
  Ok(contents)
}
