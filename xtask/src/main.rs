use anyhow::{Context, Result, bail};
use regex::{Captures, Regex};
use std::{fs, io::Write, path::Path, process::Command, sync::LazyLock};

const INDEX: &str = "target/doc/yohan_boogaert_1995/index.html";
const DESCRIPTION: &str = "Yohan Boogaert - Rust Freelance Developer based in Belgium";
const TITLE: &str = "Yohan Boogaert - Rust Developer";
const HEADING: &str = "Yohan Boogaert - Rust Developer";

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
                let section = match &caps[7] {
                    "Modules" => "Experiences",
                    "Macros" => "Personal Projects",
                    "Structs" => "OSS Contributions",
                    "Enums" => "Non-technical Skills",
                    "Constants" => "Technical Skills",
                    _ => &caps[7],
                };

                // Modify section headers
                format!("{}{}{}", &caps[6], section, &caps[8])
            } else if caps.get(9).is_some() {
                let element = &caps[16].replace("<wbr>", "").to_string();

                let name = if caps.get(10).is_some() {
                    match &caps[10] {
                        "mod" => {
                            let parts = element.split('_').collect::<Vec<_>>();

                            match parts.len() {
                                3 if parts[2] == "current" => format!("{} - Current", parts[1]),
                                3 => format!("{} - {}", parts[1], parts[2]),
                                2 => parts[1].to_string(),
                                _ => element.to_string(),
                            }
                        }
                        "macro" => element.replace('_', "-"),
                        "struct" => {
                            let mut result = String::new();
                            let mut first = true;
                            for ch in element.chars() {
                                if ch.is_uppercase() {
                                    if !first {
                                        result.push('-');
                                    }
                                    result.push(ch.to_ascii_lowercase());
                                } else {
                                    result.push(ch);
                                }
                                first = false;
                            }
                            result
                        }
                        "enum" => {
                            let mut result = String::new();
                            let mut first = true;
                            for ch in element.chars() {
                                if ch.is_uppercase() && !first {
                                    result.push(' ');
                                }
                                result.push(ch);
                                first = false;
                            }
                            result
                        }
                        "constant" => element
                            .replace('_', " ")
                            .split_whitespace()
                            .map(|word| {
                                let mut chars = word.chars();
                                match chars.next() {
                                    None => String::new(),
                                    Some(f) => {
                                        let first = f.to_uppercase().to_string();
                                        let chars = chars.collect::<String>().to_lowercase();
                                        first + &chars
                                    }
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                        "trait" => {
                            let mut result = String::new();
                            let mut first = true;
                            for ch in element.chars() {
                                if ch.is_uppercase() && !first {
                                    result.push(' ');
                                    result.push(ch.to_ascii_lowercase());
                                } else {
                                    result.push(ch);
                                }
                                first = false;
                            }
                            result
                        }
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
    use std::collections::HashMap;

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
            parse_content(
                "<h2 id=\"element\" class=\"section-header\">Modules<a href=\"#element\" class=\"anchor\">§</a></h2>"
            ),
            "<h2 id=\"element\" class=\"section-header\">Experiences<a href=\"#element\" class=\"anchor\">§</a></h2>"
        );

        assert_eq!(
            parse_content(
                "<h2 id=\"element\" class=\"section-header\">Macros<a href=\"#element\" class=\"anchor\">§</a></h2>"
            ),
            "<h2 id=\"element\" class=\"section-header\">Personal Projects<a href=\"#element\" class=\"anchor\">§</a></h2>"
        );

        assert_eq!(
            parse_content(
                "<h2 id=\"element\" class=\"section-header\">Structs<a href=\"#element\" class=\"anchor\">§</a></h2>"
            ),
            "<h2 id=\"element\" class=\"section-header\">OSS Contributions<a href=\"#element\" class=\"anchor\">§</a></h2>"
        );

        assert_eq!(
            parse_content(
                "<h2 id=\"element\" class=\"section-header\">Enums<a href=\"#element\" class=\"anchor\">§</a></h2>"
            ),
            "<h2 id=\"element\" class=\"section-header\">Non-technical Skills<a href=\"#element\" class=\"anchor\">§</a></h2>"
        );

        assert_eq!(
            parse_content(
                "<h2 id=\"element\" class=\"section-header\">Constants<a href=\"#element\" class=\"anchor\">§</a></h2>"
            ),
            "<h2 id=\"element\" class=\"section-header\">Technical Skills<a href=\"#element\" class=\"anchor\">§</a></h2>"
        );

        assert_eq!(
            parse_content(
                "<h2 id=\"element\" class=\"section-header\">Traits<a href=\"#element\" class=\"anchor\">§</a></h2>"
            ),
            "<h2 id=\"element\" class=\"section-header\">Traits<a href=\"#element\" class=\"anchor\">§</a></h2>"
        );
    }

    #[test]
    fn modify_elements() {
        let mut h = HashMap::new();

        // Modules
        h.insert("<li><div class=\"item-name\"><a class=\"mod\" href=\"element_link\" title=\"element_title\">exp01_2020_current</a></div><div class=\"desc docblock-short\">description with <a href=\"link\">a link</a>.</div></li>".to_string(),"<li><div class=\"item-name\"><a class=\"mod\" href=\"\" title=\"2020 - Current\">2020 - Current</a></div><div class=\"desc docblock-short\">description with <a href=\"link\">a link</a>.</div></li>".to_string());

        // Macros
        h.insert("<li><div class=\"item-name\"><a class=\"macro\" href=\"macro.any_thing.html\" title=\"another_macro\">super_project</a></div><div class=\"desc docblock-short\">description.<a href=\"https://repo.com/link\">[Repository]</a></div></li>".to_string(), "<li><div class=\"item-name\"><a class=\"macro\" href=\"https://repo.com/link\" title=\"super-project\">super-project</a></div><div class=\"desc docblock-short\">description.</div></li>".to_string());

        // Structs
        h.insert("<li><div class=\"item-name\"><a class=\"struct\" href=\"struct.another_thing.html\" title=\"another_struct\">SuperProject</a></div><div class=\"desc docblock-short\">description.</div></li>".to_string(), "<li><div class=\"item-name\"><a class=\"struct\" href=\"\" title=\"super-project\">super-project</a></div><div class=\"desc docblock-short\">description.</div></li>".to_string());

        // Enums
        h.insert("<li><div class=\"item-name\"><a class=\"enum\" href=\"enum.another_enum.html\" title=\"another_enum\">SuperProject</a></div><div class=\"desc docblock-short\">description.</div></li>".to_string(), "<li><div class=\"item-name\"><a class=\"enum\" href=\"\" title=\"Super Project\">Super Project</a></div><div class=\"desc docblock-short\">description.</div></li>".to_string());

        // Constants
        h.insert("<li><div class=\"item-name\"><a class=\"constant\" href=\"constant.another_constant.html\" title=\"another_constant\">SUPER_PROJECT</a></div><div class=\"desc docblock-short\">description.</div></li>".to_string(), "<li><div class=\"item-name\"><a class=\"constant\" href=\"\" title=\"Super Project\">Super Project</a></div><div class=\"desc docblock-short\">description.</div></li>".to_string());

        // Traits
        h.insert("<li><div class=\"item-name\"><a class=\"trait\" href=\"trait.another_trait.html\" title=\"another_trait\">SuperProject</a></div><div class=\"desc docblock-short\">description.</div></li>".to_string(), "<li><div class=\"item-name\"><a class=\"trait\" href=\"\" title=\"Super project\">Super project</a></div><div class=\"desc docblock-short\">description.</div></li>".to_string());

        for (key, value) in h.into_iter() {
            assert_eq!(parse_content(key), value)
        }
    }
}
