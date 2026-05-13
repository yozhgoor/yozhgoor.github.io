use crate::document::{INPUT_HTML_PATH, generate_input};
use anyhow::{Context, Result, bail};
use std::{fs, io::Write, path::Path, process::Command};

const PDF_OUTPUT: &str = "target/doc/resume.pdf";
const TEMPLATE_PATH: &str = "target/doc/template.tex";

pub(crate) fn generate_pdf(source: &str) -> Result<()> {
    generate_input(source)?;
    create_pdf_template()?;
    run_pandoc()?;

    println!("File generated successfully: {}", PDF_OUTPUT);
    Ok(())
}

fn create_pdf_template() -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(Path::new(TEMPLATE_PATH))
        .context("failed to open template.tex")?;

    let content = r#"\documentclass{article}

\usepackage{geometry}
\usepackage{hyperref}
\usepackage{fancyhdr}

\geometry{top=0.3in,bottom=0.5in,left=0.5in,right=0.5in}

\hypersetup{
  colorlinks=true,
  urlcolor=blue,
}

\pagestyle{fancy}
\fancyhf{}
\fancyfoot[C]{Full version available at \url{https://yozhgoor.github.io}}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{0pt}

\providecommand{\tightlist}{\setlength{\itemsep}{0pt}\setlength{\parskip}{0pt}}

\setcounter{secnumdepth}{0}
\setlength{\parindent}{0pt}

\begin{document}

$body$

\end{document}"#;

    file.write_all(content.as_bytes())
        .context("failed to write template.tex")?;

    println!("File generated successfully: {}", TEMPLATE_PATH);

    Ok(())
}

fn run_pandoc() -> Result<()> {
    match Command::new("pandoc")
        .args([
            INPUT_HTML_PATH,
            "--from",
            "html",
            "--template",
            TEMPLATE_PATH,
            "--output",
            PDF_OUTPUT,
        ])
        .status()
    {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => bail!("pandoc command failed"),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            bail!("pandoc is not installed or not available in PATH")
        }
        Err(err) => Err(err).context("failed to execute pandoc"),
    }
}
