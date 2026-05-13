mod cleanup;
mod document;
mod pdf;
mod source;

fn main() -> anyhow::Result<()> {
    let source = source::generate_source()?;

    document::generate_index(source.as_ref())?;

    pdf::generate_pdf(source.as_ref())?;

    cleanup::clean_target_dir()?;

    Ok(())
}
