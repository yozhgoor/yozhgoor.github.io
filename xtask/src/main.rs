use anyhow::{Context, Result, bail};
use dom_query::Document;
use std::{
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    process::Command,
};

const INDEX: &str = "target/doc/yohan_boogaert_1995/index.html";
const DOC: &str = "target/doc";
const DESCRIPTION: &str = "Rust Software Developer based in Belgium.";
const HEADING: &str = "Yohan Boogaert - Rust Software Developer";

fn main() -> Result<()> {
    let index = Path::new(INDEX);

    if !index.exists() {
        match Command::new("cargo").arg("doc").status() {
            Ok(status) if status.success() => (),
            _ => bail!("failed to generate document"),
        }
    }

    let document = Document::from(fs::read_to_string(index).context("failed to read index file")?);

    manage_document(&Path::new(DOC).join("index.html"), document.clone(), false)
        .context("failed to modify index file")?;
    manage_document(&Path::new(DOC).join("input.html"), document, true)
        .context("failed to create `input.html`")?;

    clean_target().context("failed to clean target directory")?;

    create_template().context("failed to create template")?;

    Ok(())
}

fn manage_document(path: &Path, document: Document, full: bool) -> Result<()> {
    let content = manipulate_document(document, full).context("failed to manipulate document")?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .context(format!("failed to open file at {}", path.display()))?;

    file.write_all(content.html().as_bytes())?;

    println!("File generated successfully: {}", path.display());

    Ok(())
}

fn clean_target() -> Result<()> {
    let doc_path = Path::new(DOC);

    for entry in fs::read_dir(doc_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if !path.ends_with("static.files") {
                fs::remove_dir_all(&path)?;
            } else {
                for e in fs::read_dir(path)? {
                    let e = e?;
                    let p = e.path();

                    if let Some(file_name) = p.file_name()
                        && let Some(s) = file_name.to_str()
                        && !s.contains("rustdoc")
                        && !s.contains("favicon")
                        && !s.contains("Fira")
                        && !s.contains("COPYRIGHT")
                    {
                        fs::remove_file(&p)?;
                    }
                }
            }
        } else if path.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if file_name != "input.html" && file_name != "index.html" {
                fs::remove_file(&path)?;
            }
        }
    }

    Ok(())
}

fn manipulate_document(doc: Document, full: bool) -> Result<Document> {
    apply_document_basics(&doc, full);
    rewrite_section_headers(&doc, full);
    rewrite_item_table(&doc, full);

    if full {
        finalize_full_output(&doc);
    } else {
        finalize_web_output(&doc);
    }

    Ok(doc)
}

fn apply_document_basics(doc: &Document, full: bool) {
    // Force dark theme
    let html = doc.select_single("html");
    html.set_attr("data-theme", "dark");

    // Remove unused meta elements
    let meta = doc.select("head meta[name=generator],head meta[name=rustdoc-vars]");
    meta.remove();

    // Adapt link elements `href`s
    for link in doc.select("link").iter() {
        let href = link
            .attr("href")
            .expect("selected link have href")
            .to_string();

        let path = Path::new(&href)
            .components()
            .filter(|c| *c != Component::ParentDir)
            .collect::<PathBuf>();

        let s = path.as_os_str().to_str().expect("can convert path");

        if s.contains("rustdoc") || s.contains("icon") {
            link.set_attr("href", s);
        } else {
            link.remove();
        }

        link.set_attr("href", s);
    }

    // Change description
    let meta = doc.select_single("head meta[name=description]");
    meta.set_attr("content", DESCRIPTION);

    // Change title
    let title = doc.select_single("head title");
    if full {
        title.remove();
    } else {
        title.set_html(HEADING);
    }

    // Remove unused elements
    let unused =
        doc.select("script,noscript,nav,rustdoc-search,div.sidebar-resizer,rustdoc-topbar");
    unused.remove();

    // Change main heading
    let heading = doc.select_single("div.main-heading");
    heading.set_html(format!("<h1>{}</h1>", HEADING));

    // Unwrap rustdoc intro block and remove toggle summary
    let summary = doc.select("details.top-doc > summary");
    summary.remove();

    let details = doc.select_single("details.top-doc");
    let intro = doc.select(
        "details.top-doc > div.docblock > p, details.top-doc > div.docblock > ul, details.top-doc > div.docblock > ol",
    );
    details.replace_with_selection(&intro);
}

fn rewrite_section_headers(doc: &Document, full: bool) {
    for header in doc.select("h2.section-header").iter() {
        let a = header.select_single("a");
        let text = header.immediate_text().to_string();
        let mapped = remap_section_header(&text);

        if full {
            header.set_html(mapped);
        } else {
            header.set_html(format!("{}{}", mapped, a.html()));
        }
    }
}

fn rewrite_item_table(doc: &Document, full: bool) {
    for item in doc.select("dl.item-table > dt:has(a[class])").iter() {
        let name_a = item.select_single("a");
        let name_text = name_a.immediate_text().replace("<wbr>", "");
        let description = item.next_sibling();

        let class = name_a.attr("class").map(|c| c.to_string());

        let mut tags: Vec<String> = Vec::new();
        let mut cleaned_description = description.inner_html().to_string();
        if class.as_deref() == Some("mod") {
            let (parsed_tags, cleaned) = parse_tags_from_description(&cleaned_description);
            tags = parsed_tags;
            cleaned_description = cleaned;
        }

        if let Some(class) = class.as_deref() {
            let name = match class {
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

        if class.as_deref() == Some("mod") {
            if full {
                description.set_html(cleaned_description);
            } else {
                let tags_html = render_tags_html(&tags);
                description.set_html(format!("{}{}", tags_html, cleaned_description));
            }
        }
    }
}

fn finalize_full_output(doc: &Document) {
    for item in doc.select("dl.item-table > dt:has(a)").iter() {
        let a = item.select_single("a");
        let desc = item.next_sibling();
        let line = format!("{} - {}", a.html(), desc.inner_html());

        item.set_html(line);
    }

    doc.select("dl.item-table > dd").remove();
}

fn finalize_web_output(doc: &Document) {
    let head = doc.select_single("head");
    head.append_html(
        r#"<style>
.tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.35rem;
  margin: 0.35rem 0 0.5rem;
}
.tag {
  border: 1px solid currentColor;
  border-radius: 999px;
  padding: 0.1rem 0.45rem;
  font-size: 0.78rem;
  line-height: 1.2;
  opacity: 0.9;
}
dl.item-table > dt:has(a.mod) {
  padding-top: 2rem;
}
</style>"#,
    );

    let section = doc.select_single("section.content");
    section.append_html(
        r#"<p style="text-align:center; margin-top:1.5rem;">
PDF version available <a href="resume.pdf">here</a>.
</p>"#,
    );
}

fn create_template() -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(Path::new(DOC).join("template.tex"))
        .context("failed to open template")?;

    let content = r#"\documentclass{article}

\usepackage{geometry}
\usepackage{hyperref}

\geometry{margin=0.5in}

\hypersetup{
  colorlinks=true,
  urlcolor=blue,
}

\pagestyle{empty}

\providecommand{\tightlist}{\setlength{\itemsep}{0pt}\setlength{\parskip}{0pt}}

\setcounter{secnumdepth}{0}
\setlength{\parindent}{0pt}

\begin{document}

$body$

\vspace{0.3in}
\begin{center}
Full version available at \url{https://yozhgoor.github.io}
\end{center}

\end{document}"#;

    file.write_all(content.as_bytes())?;

    Ok(())
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
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
                    first + &rest
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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

fn render_tags_html(tags: &[String]) -> String {
    if tags.is_empty() {
        return String::new();
    }

    let chips = tags
        .iter()
        .map(|t| format!(r#"<span class="tag">{}</span>"#, t))
        .collect::<Vec<_>>()
        .join("");

    format!(r#"<div class="tags">{}</div>"#, chips)
}
