use dioxus::prelude::*;

#[cfg(feature = "server")]
mod tokio_utilities {
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
}

// Then you can define your server functions using shared utilities you defined for
// server only code.
#[server]
pub async fn get_file_contents(filename: String) -> Result<String, ServerFnError> {
  let absolute_path = std::env::current_dir()?;
  let path = append_file_to_parent_dir(&absolute_path, &filename);
  let file = read_file(path).await?;
  Ok(file)
}

