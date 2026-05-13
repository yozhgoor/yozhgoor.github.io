use anyhow::{Context, Result};
use std::{
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
};

mod index;
mod input;

pub(crate) use index::generate_index;
pub(crate) use input::{INPUT_HTML_PATH, generate_input};

const HEADING: &str = "Yohan Boogaert - Rust Software Developer";
const DESCRIPTION: &str = "Rust Software Developer based in Belgium.";

struct Document(dom_query::Document);

impl Document {
    fn parse(source: &str) -> Self {
        Self(dom_query::Document::from(source))
    }

    fn write(&self, path: &Path) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .context(format!("failed to open file at {}", path.display()))?;

        file.write_all(self.0.html().as_bytes())
            .context(format!("failed to write file at {}", path.display()))?;

        println!("File generated successfully: {}", path.display());

        Ok(())
    }

    fn apply_base(&self) {
        let meta = self.select("head meta[name=generator], head meta[name=rustdoc-vars]");
        meta.remove();

        for link in self.select("link").iter() {
            let href = link
                .attr("href")
                .expect("selected link should have href")
                .to_string();

            let path = Path::new(&href)
                .components()
                .filter(|c| *c != Component::ParentDir)
                .collect::<PathBuf>();

            let normalized = path.as_os_str().to_str().expect("can convert href path");

            if normalized.contains("rustdoc") || normalized.contains("icon") {
                link.set_attr("href", normalized);
            } else {
                link.remove();
            }

            link.set_attr("href", normalized);
        }

        let heading = self.select_single("div.main-heading");
        heading.set_html(format!("<h1>{}</h1>", HEADING));

        let desc = self.select_single("head meta[name=description]");
        desc.set_attr("content", DESCRIPTION);

        let unused =
            self.select("script,noscript,nav,rustdoc-search,div.sidebar-resizer,rustdoc-topbar");
        unused.remove();

        let summary = self.select("details.top-doc > summary");
        summary.remove();

        let details = self.select_single("details.top-doc");
        let intro = self.select(
            format!(
                "{0} > {1} > p, {0} > {1} > ul, {0} > {1} > ol",
                "details.top-doc", "div.docblock",
            )
            .as_ref(),
        );
        details.replace_with_selection(&intro);
    }

    fn rewrite_item_names_and_links(&self) {
        for item in self.select("dl.item-table > dt:has(a[class])").iter() {
            let name_a = item.select_single("a");
            let name_text = name_a.immediate_text().replace("<wbr>", "");
            let description = item.next_sibling();

            if let Some(class) = name_a.attr("class") {
                let name = match class.as_ref() {
                    "mod" => format_mod_name(&name_text),
                    "macro" if name_text == "create_process_w" => "CreateProcessW".to_string(),
                    "macro" => name_text.replace('_', "-"),
                    "struct" => format_struct_name(&name_text),
                    "enum" => format_enum_name(&name_text),
                    "constant" => format_constant_name(&name_text),
                    _ => name_text.clone(),
                };

                name_a.set_attr("title", &name);
                name_a.set_text(&name);
            }

            let desc_a = description.select("a:has-text(\"[Repository]\")");
            if let Some(href) = desc_a.attr("href") {
                name_a.set_attr("href", &href);
            } else {
                name_a.set_attr("href", "javascript:void(0)");
            }
            desc_a.remove();
        }
    }

    fn select(&self, sel: &str) -> dom_query::Selection<'_> {
        self.0.select(sel)
    }

    fn select_single(&self, sel: &str) -> dom_query::Selection<'_> {
        self.0.select_single(sel)
    }
}

fn parse_tags_from_description(desc: &str) -> (Vec<String>, String) {
    fn parse_line(input: &str) -> Option<(Vec<String>, String)> {
        let normalized = input.replace("&nbsp;", " ");
        let trimmed = normalized.trim_start();

        let rest = trimmed.strip_prefix("Tags:")?;
        let (tags_part, desc_part) = rest.split_once('|')?;

        let tags = tags_part
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        Some((tags, desc_part.trim().to_string()))
    }

    if let Some(start_p) = desc.find("<p>")
        && let Some(end_p_rel) = desc[start_p..].find("</p>")
    {
        let end_p = start_p + end_p_rel + "</p>".len();
        let first_p = &desc[start_p..end_p];
        let first_p_text = first_p.replace("<p>", "").replace("</p>", "");

        if let Some((tags, cleaned_desc_text)) = parse_line(&first_p_text) {
            let rebuilt_first_p = format!("<p>{}</p>", cleaned_desc_text);
            let mut cleaned_html = desc.to_string();
            cleaned_html.replace_range(start_p..end_p, &rebuilt_first_p);
            return (tags, cleaned_html);
        }
    }

    if let Some((tags, cleaned_desc_text)) = parse_line(desc) {
        return (tags, cleaned_desc_text);
    }

    (Vec::new(), desc.to_string())
}

fn remap_section_header(text: &str) -> &str {
    match text {
        "Modules" => "Experiences",
        "Macros" => "Personal Projects",
        "Structs" => "Open-source Contributions",
        "Enums" => "Professional Skills",
        "Constants" => "Technical Skills",
        _ => text,
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn format_mod_name(name_text: &str) -> String {
    let parts = name_text.split('_').collect::<Vec<_>>();

    match parts.len() {
        5 => format!(
            "{} {} - {} {}",
            capitalize_first(parts[1]),
            parts[2],
            capitalize_first(parts[3]),
            parts[4]
        ),
        4 if parts[3] == "current" => {
            format!("{} {} - Current", capitalize_first(parts[1]), parts[2])
        }
        3 if parts[2] == "current" => format!("{} - Current", parts[1]),
        3 => format!("{} - {}", parts[1], parts[2]),
        2 => parts[1].to_string(),
        _ => name_text.to_string(),
    }
}

fn format_struct_name(name_text: &str) -> String {
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

fn format_enum_name(name_text: &str) -> String {
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

fn format_constant_name(name_text: &str) -> String {
    name_text
        .replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => {
                    let first = f.to_uppercase().to_string();
                    let rest = chars.collect::<String>().to_lowercase();
                    first + rest.as_ref()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
