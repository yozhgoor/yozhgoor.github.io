use anyhow::{Context, Result, bail};
use regex::{Captures, Regex};
use std::{fs, io::Write, path::Path, process::Command, sync::LazyLock};

const INDEX: &str = "target/doc/yohan_boogaert_1995/index.html";

fn main() -> Result<()> {
    let index = Path::new(INDEX);

    if !index.exists() {
        match Command::new("cargo").arg("doc").status() {
            Ok(status) if status.success() => (),
            _ => bail!("failed to generate documentation"),
        }
    }

    let content = parse_content(fs::read_to_string(index).context("failed to read index file")?);

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(index)
        .context("failed to open index file")?;

    file.write_all(content.as_bytes())?;

    println!("Generated successfully");

    Ok(())
}

fn parse_content(s: impl Into<String>) -> String {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"(<nav.*?</nav>)"#).expect("regex must be compiled"));

    let mut s = s.into();

    s = RE
        .replace_all(&s, |caps: &Captures| {
            if caps.get(1).is_some() {
                "".to_string()
            } else {
                caps[0].to_string()
            }
        })
        .to_string();

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_nav() {
        let res = "beforeafter";

        assert_eq!(parse_content("before<nav>inside</nav>after"), res);

        assert_eq!(
            parse_content("before<nav class=\"inside\">inside</nav>after"),
            res
        );
    }
}
