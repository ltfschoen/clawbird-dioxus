use dioxus::prelude::*;

use std::path::{PathBuf, Path};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn read_file(path: PathBuf) -> Result<String, std::io::Error> {
  let mut file = File::open(path).await?;
  let mut contents = String::new();
  file.read_to_string(&mut contents).await?;
  Ok(contents)
}

pub fn append_file_to_parent_dir(p: &Path, filename: &str) -> PathBuf {
  let dirs = p.parent().unwrap();
  dirs.join(filename)
}

// Then you can define your server functions using shared utilities you defined for
// server only code.
#[server]
async fn get_file_contents(filename: &str) -> Result<String, ServerFnError> {
  let absolute_path = std::env::current_dir()?;
  let file = read_file(append_file_to_parent_dir(&absolute_path, filename)).await?;
  Ok(file)
}

