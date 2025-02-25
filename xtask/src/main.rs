use anyhow::{Context, Result, bail};
use regex::{Captures, Regex};
use std::{fs, io::Write, path::Path, process::Command, sync::LazyLock};

const INDEX: &str = "target/doc/yohan_boogaert_1995/index.html";
const DESCRIPTION: &str = "Description";
const TITLE: &str = "Title";
const HEADING: &str = "Heading";
const SECTION: &str = "Section";

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
            r#"(<nav.*?</nav>)|(<rustdoc-search></rustdoc-search>)|(<meta name="description" content=".*?">)|(<title>.*?</title>)|(<div class="main-heading".*?</div>)|(<h2[^>]*class="section-header"[^>]*>)([^<]*)(<a[^>]*>.*?</a></h2>)|(<li>\s*<div class="item-name">\s*<a class=")([^"]*)(" href=")([^"]*)(" title=")([^"]*)(">\s*)(.*?)(\s*</a>\s*</div>\s*<div class="desc docblock-short">\s*)(.*?)(?:(<a href="([^"]*)">\[Repository\]</a>)\s*)?(</div>\s*</li>)"#
        ).expect("regex must be compiled")
    });

    let mut s = s.into();

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
            } else if caps.get(6).is_some() {
                // Modify section headers
                format!("{}{}{}", &caps[6], SECTION, &caps[8])
            } else if caps.get(9).is_some() {
                let element = &caps[16].replace("<wbr>", "").to_string();

                let name = if caps.get(10).is_some() {
                    match &caps[10] {
                        "element_class" => element.to_lowercase(),
                        "any_class" => element.to_uppercase(),
                        /*
                        "mod" => todo!("modules"),
                        "macro" => todo!("macro"),
                        "struct" => todo!("struct"),
                        "enum" => todo!("enum"),
                        "constant" => todo!("constant"),
                        "trait" => todo!("trait"),
                        */
                        _ => element.to_string(),
                    }
                } else {
                    element.to_string()
                };

                format!(
                    "{}{}{}{}{}{}{}{}{}{}{}",
                    &caps[9],
                    &caps[10],
                    &caps[11],
                    caps.get(20).map_or("", |m| m.as_str()),
                    &caps[13],
                    name,
                    &caps[15],
                    name,
                    &caps[17],
                    caps.get(18).map_or("", |m| m.as_str()),
                    &caps[21],
                )
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
        assert_eq!(parse_content("<nav>inside</nav>"), "");

        assert_eq!(parse_content("<nav class=\"inside\">inside</nav>"), "");
    }

    #[test]
    fn remove_search() {
        assert_eq!(parse_content("<rustdoc-search></rustdoc-search>"), "");
    }

    #[test]
    fn modify_description() {
        assert_eq!(
            parse_content("<meta name=\"description\" content=\"Content\">"),
            format!("<meta name=\"description\" content=\"{}\">", DESCRIPTION)
        )
    }

    #[test]
    fn modify_title() {
        assert_eq!(
            parse_content("<title>Content</title>"),
            format!("<title>{}</title>", TITLE)
        )
    }

    #[test]
    fn modify_heading() {
        assert_eq!(
            parse_content("<div class=\"main-heading\"><h1>Content<span>content</span></h1></div>"),
            format!("<div class=\"main-heading\"><h1>{}</h1></div>", HEADING)
        );
    }

    #[test]
    fn modify_subtitle() {
        assert_eq!(
            parse_content("<h2 id=\"element\" class=\"section-header\">Element</h2>"),
            format!("<h2 id=\"element\" class=\"section-header\">Element</h2>")
        );

        assert_eq!(
            parse_content(
                "<h2 id=\"element\" class=\"section-header\">Element<a href=\"#element\" class=\"anchor\">§</a></h2>"
            ),
            format!(
                "<h2 id=\"element\" class=\"section-header\">{}<a href=\"#element\" class=\"anchor\">§</a></h2>",
                SECTION
            ),
        );
    }

    #[test]
    fn modify_elements() {
        assert_eq!(
            parse_content(
                "<li><div class=\"item-name\"><a class=\"element_class\" href=\"element_link\" title=\"element_title\">element</a></div><div class=\"desc docblock-short\">description with <a href=\"link\">a link</a>.</div></li>"
            ),
            "<li><div class=\"item-name\"><a class=\"element_class\" href=\"\" title=\"element\">element</a></div><div class=\"desc docblock-short\">description with <a href=\"link\">a link</a>.</div></li>"
        );

        assert_eq!(
            parse_content(
                "<li><div class=\"item-name\"><a class=\"any_class\" href=\"macro.any_thing.html\" title=\"title\">any_<wbr>thing</a></div><div class=\"desc docblock-short\">description.<a href=\"https://repo.com/link\">[Repository]</a></div></li>"
            ),
            "<li><div class=\"item-name\"><a class=\"any_class\" href=\"https://repo.com/link\" title=\"ANY_THING\">ANY_THING</a></div><div class=\"desc docblock-short\">description.</div></li>"
        );
    }
}
