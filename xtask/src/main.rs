use anyhow::{Context, Result, bail};
use regex::{Captures, Regex};
use std::{fs, io::Write, path::Path, process::Command, sync::LazyLock};

const INDEX: &str = "target/doc/yohan_boogaert_1995/index.html";
const DESCRIPTION: &str = "Description";
const TITLE: &str = "Title";
const HEADING: &str = "Heading";

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
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r#"(<nav.*?</nav>)|(<rustdoc-search></rustdoc-search>)|(<meta name="description" content=".*?">)|(<title>.*?</title>)|(<div class="main-heading".*?</div>)"#
        ).expect("regex must be compiled")
    });

    let mut s = s.into();

    // To modify:
    // <h2 class="section-header"...</h2>
    // <li><div class="item-name"...</li>

    s = RE
        .replace_all(&s, |caps: &Captures| {
            if caps.get(1).is_some() {
                // Remove the navigation bar

                "".to_string()
            } else if caps.get(2).is_some() {
                // Remove the search bar

                "".to_string()
            } else if caps.get(3).is_some() {
                // Modify the description
                format!("<meta name=\"description\" content=\"{}\">", DESCRIPTION)
            } else if caps.get(4).is_some() {
                // Modify the title
                format!("<title>{}</title>", TITLE)
            } else if caps.get(5).is_some() {
                // Modify main heading
                format!("<div class=\"main-heading\"><h1>{}</h1></div>", HEADING)
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

    const RES: &str = "beforeafter";

    #[test]
    fn remove_nav() {
        assert_eq!(parse_content("before<nav>inside</nav>after"), RES);

        assert_eq!(
            parse_content("before<nav class=\"inside\">inside</nav>after"),
            RES
        );
    }

    #[test]
    fn remove_search() {
        assert_eq!(
            parse_content("before<rustdoc-search></rustdoc-search>after"),
            RES
        );
    }

    #[test]
    fn modify_description() {
        assert_eq!(
            parse_content("before<meta name=\"description\" content=\"old content\">after"),
            format!(
                "before<meta name=\"description\" content=\"{}\">after",
                DESCRIPTION
            )
        )
    }

    #[test]
    fn modify_title() {
        assert_eq!(
            parse_content("before<title>old title</title>after"),
            format!("before<title>{}</title>after", TITLE)
        )
    }

    #[test]
    fn modify_heading() {
        assert_eq!(
            parse_content(
                "before<div class=\"main-heading\"><h1>Old<span>content</span></h1></div>"
            ),
            format!(
                "before<div class=\"main-heading\"><h1>{}</h1></div>",
                HEADING
            )
        );
    }
}
