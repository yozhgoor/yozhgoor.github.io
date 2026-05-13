use super::{Document, HEADING, parse_tags_from_description, remap_section_header};
use anyhow::Result;
use std::path::Path;

pub(crate) fn generate_index(source: &str) -> Result<()> {
    let document = Document::parse(source);

    document.apply_base();
    document.rewrite_item_names_and_links();

    let html = document.select_single("html");
    html.set_attr("data-theme", "dark");

    let title = document.select_single("head title");
    title.set_html(HEADING);

    rewrite_section_headers(&document);

    inject_tags(&document);
    inject_index_style(&document);
    append_pdf_hint(&document);

    document.write(Path::new("target/doc/index.html"))
}

fn rewrite_section_headers(document: &Document) {
    for header in document.select("h2.section-header").iter() {
        let a = header.select_single("a");
        let text = header.immediate_text().to_string();
        let mapped = remap_section_header(&text);
        header.set_html(format!("{}{}", mapped, a.html()));
    }
}

fn inject_tags(document: &Document) {
    for item in document
        .select("dl.item-table > dt:has(a[class=mod])")
        .iter()
    {
        let description = item.next_sibling();
        let original = description.inner_html().to_string();

        let (tags, cleaned) = parse_tags_from_description(&original);
        let tags_html = render_tags(&tags);

        description.set_html(format!("{}{}", tags_html, cleaned));
    }
}

fn render_tags(tags: &[String]) -> String {
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

fn inject_index_style(document: &Document) {
    let head = document.select_single("head");
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
}

fn append_pdf_hint(document: &Document) {
    let section = document.select_single("section.content");
    section.append_html(
        r#"<p style="text-align:center; margin-top:1.5rem;">
PDF version available <a href="resume.pdf">here</a>.
</p>"#,
    );
}
