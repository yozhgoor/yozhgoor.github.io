use anyhow::{Context, Result, bail};
use dom_query::Document;
use std::{fs, io::Write, path::Path, process::Command};

const INDEX: &str = "target/doc/yohan_boogaert_1995/index.html";
const DESCRIPTION: &str = "Rust Freelance Developer based in Belgium.";
const TITLE: &str = "Rust Developer - Yohan Boogaert";
const HEADING: &str = "Yohan Boogaert - Rust Developer";

fn main() -> Result<()> {
    let index = Path::new(INDEX);

    if !index.exists() {
        match Command::new("cargo").arg("doc").status() {
            Ok(status) if status.success() => (),
            _ => bail!("failed to generate document"),
        }
    }

    let document = Document::from(fs::read_to_string(index).context("failed to read index file")?);

    manipulate_document(&document).context("failed to manipulate document")?;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(index)
        .context("failed to open index file")?;

    file.write_all(document.html().as_bytes())?;

    println!("Generated successfully");

    Ok(())
}

fn manipulate_document(doc: &Document) -> Result<()> {
    // Change description
    let meta = doc.select_single("head meta[name=description]");
    meta.set_attr("content", DESCRIPTION);

    // Change title
    let title = doc.select_single("head title");
    title.set_html(TITLE);

    // Remove navigation bar
    let nav = doc.select("nav");
    nav.remove();

    // Remove sidebar resizer
    let side = doc.select("div.sidebar-resizer");
    side.remove();

    // Remove search bar
    let search = doc.select_single("rustdoc-search");
    search.remove();

    // Change main heading
    let heading = doc.select_single("div.main-heading");
    heading.set_html(format!("<h1>{}</h1>", HEADING));

    // Change section headers
    let headers = doc.select("h2.section-header");

    for header in headers.iter() {
        let a = header.select_single("a");
        let text = header.immediate_text().to_string();

        let txt = match text.as_ref() {
            "Modules" => "Experiences",
            "Macros" => "Personal Project",
            "Structs" => "OSS Contributions",
            "Enums" => "Non-technical Skills",
            "Constants" => "Technical Skills",
            _ => text.as_ref(),
        };

        header.set_html(format!("{}{}", txt, a.html()));
    }

    // Change list items
    let items = doc.select("li:has(div.item-name)");

    for item in items.iter() {
        let item_name = item.select("div.item-name");
        let name_a = item_name.select("a");
        let name_text = name_a.immediate_text().replace("<wbr>", "");

        // Modify the name depending on the class
        if let Some(class) = name_a.attr("class") {
            let name = match class.as_ref() {
                "mod" => {
                    let parts = name_text.split('_').collect::<Vec<_>>();

                    match parts.len() {
                        3 if parts[2] == "current" => format!("{} - Current", parts[1]),
                        3 => format!("{} - {}", parts[1], parts[2]),
                        2 => parts[1].to_string(),
                        _ => name_text.to_string(),
                    }
                }
                "macro" if name_text == "create_process_w" => "CreateProcessW".to_string(),
                "macro" => name_text.replace("_", "-"),
                "struct" => {
                    if name_text == "create_process_w" {
                        "CreateProcessW".to_string()
                    } else {
                        let mut result = String::new();
                        let mut first = true;
                        for ch in name_text.chars() {
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
                }
                "enum" => {
                    let mut result = String::new();
                    let mut first = true;
                    for ch in name_text.chars() {
                        if ch.is_uppercase() && !first {
                            result.push(' ');
                        }
                        result.push(ch);
                        first = false;
                    }
                    result
                }
                "constant" => name_text
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
                    for ch in name_text.chars() {
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
                _ => name_text,
            };

            name_a.set_attr("title", &name);
            name_a.set_text(&name);
        }

        let description = item.select("div.desc");
        let desc_a = description.select("a:has-text(\"[Repository]\")");

        if let Some(href) = desc_a.attr("href") {
            name_a.set_attr("href", &href);
        }

        // Remove `[Repository]` links
        desc_a.remove();
    }

    Ok(())
}
