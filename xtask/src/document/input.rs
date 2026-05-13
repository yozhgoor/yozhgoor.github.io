use super::{Document, parse_tags_from_description, remap_section_header};
use anyhow::Result;
use std::path::Path;

pub(crate) const INPUT_HTML_PATH: &str = "target/doc/input.html";

pub(crate) fn generate_input(source: &str) -> Result<()> {
    let document = Document::parse(source);

    document.apply_base();
    document.rewrite_item_names_and_links();

    let title = document.select_single("head title");
    title.remove();

    rewrite_section_headers(&document);
    strip_tags(&document);
    add_spacing_before_links(&document);
    flatten_item_table(&document);

    document.write(Path::new("target/doc/input.html"))
}

fn rewrite_section_headers(document: &Document) {
    for header in document.select("h2.section-header").iter() {
        let text = header.immediate_text().to_string();
        let mapped = remap_section_header(&text);
        header.set_html(mapped);
    }
}

fn add_spacing_before_links(document: &Document) {
    for p in document.select("section.content > p").iter() {
        let text = p.text().to_string();
        if text.contains("LinkedIn") && text.contains("GitHub") {
            let current = p.inner_html().to_string();
            p.set_html(format!("<br/>{}", current));
            break;
        }
    }
}

fn strip_tags(document: &Document) {
    for item in document
        .select("dl.item-table > dt:has(a[class=mod])")
        .iter()
    {
        let description = item.next_sibling();
        let original = description.inner_html().to_string();

        let (_tags, cleaned) = parse_tags_from_description(&original);
        description.set_html(cleaned);
    }
}

fn flatten_item_table(document: &Document) {
    for item in document.select("dl.item-table > dt:has(a)").iter() {
        let a = item.select_single("a");
        let desc = item.next_sibling();
        let line = format!("{} - {}", a.html(), desc.inner_html());

        item.set_html(line);
    }

    document.select("dl.item-table > dd").remove();
}
