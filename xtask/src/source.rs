use anyhow::{Context, Result, bail};
use std::{fs, process::Command};

pub(crate) fn generate_source() -> Result<String> {
    match Command::new("cargo").arg("doc").status() {
        Ok(status) if status.success() => {}
        Ok(_) => bail!("failed to generate source with `cargo doc`"),
        Err(err) => return Err(err).context("failed to execute `cargo doc`"),
    }

    fs::read_to_string("target/doc/yohan_boogaert_1995/index.html")
        .context("failed to read rustdoc index")
}
