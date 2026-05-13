use anyhow::Result;
use std::path::Path;
use std::{fs, io};

const DOC_DIR: &str = "target/doc";

pub(crate) fn clean_target_dir() -> Result<()> {
    for entry in fs::read_dir(DOC_DIR)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if !path.ends_with("static.files") {
                fs::remove_dir_all(&path)?;
            } else {
                clean_static_files_dir(&path)?;
            }
        } else if path.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if file_name != "index.html" && file_name != "resume.pdf" {
                fs::remove_file(&path)?;
            }
        }
    }

    Ok(())
}

fn clean_static_files_dir(static_files_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(static_files_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            let keep = file_name.contains("rustdoc")
                || file_name.contains("favicon")
                || file_name.contains("Fira")
                || file_name.contains("COPYRIGHT");

            if !keep {
                fs::remove_file(&path)?;
            }
        }
    }

    Ok(())
}
