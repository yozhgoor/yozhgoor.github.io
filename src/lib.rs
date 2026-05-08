//! Rust software developer based in Belgium, available for freelance missions and full-time
//! opportunities in remote or hybrid settings.
//!
//! I design and build production Rust software end-to-end across consulting and product
//! environments, from systems programming and backend services to developer tooling and team
//! training, with a strong emphasis on architecture, reliability, and long-term maintainability.
//!
//! I am an active contributor and maintainer in the Rust open-source ecosystem, and I regularly
//! participate in community events such as meetups and conferences.
//!
//! [LinkedIn][linkedin] · [GitHub][github] · [Mastodon][mastodon] · [Email][email]
//!
//! [linkedin]: https://www.linkedin.com/in/yohan-boogaert
//! [github]: https://github.com/yozhgoor
//! [mastodon]: https://mastodon.social/@yoz@hachyderm.io
//! [email]: mailto:yohan.boogaert+recruit@protonmail.com

// Jobs

/// Tags: Rust, Systems, Concurrency, Data Collection, Compliance, CI/CD, Git, Cross-platform |
/// Rust Developer (freelance) at Ystorian, building a cybersecurity platform that collects BIOS,
/// kernel, OS, and network data in parallel across multiple systems to assess vulnerabilities and
/// compliance, enabling reliable analysis at scale.
pub mod exp01_jul_2022_jan_2025 {}

/// Tags: Rust, Backend, Query Engine, Microservices, AWS, Data Analytics, Technical Training |
/// Rust Developer (freelance) at Cumul.io (now Luzmo), collaborating with the CTO and backend team
/// on a Rust rewrite of a data analytics query engine to increase performance and microservices
/// interoperability in an AWS environment, while also leading Rust training for engineers from
/// diverse backgrounds.
pub mod exp02_mar_2022_jul_2022 {}

/// Tags: Rust, Consulting, Open-Source, Backend, Developer Tooling, Systems, CI/CD, Git,
/// WebAssembly |
/// Rust Developer at RustMinded, evolving into consulting through open-source
/// collaboration and production-focused Rust engineering across client missions.
pub mod exp03_mar_2021_current {}

// Personal Projects

/// A CLI tool to spin up temporary Rust projects with pre-installed dependencies for quick
/// prototyping and experimentation.
/// [\[Repository\]](<https://github.com/yozhgoor/cargo-temp>)
#[macro_export]
macro_rules! cargo_temp {
    () => {};
}

/// A Windows process-management library built on the Win32 API, offering an interface consistent
/// with `std::process`.
/// [\[Repository\]](<https://github.com/yozhgoor/CreateProcessW>)
#[macro_export]
macro_rules! create_process_w {
    () => {};
}

/// A CLI tool to manage and streamline Rust project workflows.
/// [\[Repository\]](<https://github.com/yozhgoor/cargo-flow>)
#[macro_export]
macro_rules! cargo_flow {
    () => {};
}

/// Automatic grid layout utility for ratatui TUI applications.
/// [\[Repository\]](<https://github.com/yozhgoor/ratatui-auto-grid>)
#[macro_export]
macro_rules! ratatui_auto_grid {
    () => {};
}

/// A utility to suppress warnings in xtask-based builds without invalidating the dependency cache.
/// [\[Repository\]](<https://github.com/yozhgoor/xtask-no-warnings>)
#[macro_export]
macro_rules! xtask_no_warnings {
    () => {};
}

// OSS Contributions

/// A developer tool for bootstrapping new Rust projects from existing git templates.
/// [\[Repository\]](<https://github.com/cargo-generate/cargo-generate>)
pub struct CargoGenerate {}

/// Date and time library for Rust.
/// [\[Repository\]](<https://github.com/chronotope/chrono>)
pub struct Chrono {}

/// French translation of "The Rust Programming Language".
/// [\[Repository\]](<https://github.com/Jimskapt/rust-book-fr>)
pub struct RustBookFr {}

/// Third-I, a camera project designed to replicate hearing and point of view.
/// [\[Repository\]](<https://github.com/BigBoySystems/third-i-frontend>)
pub struct ThirdIFrontend {}

/// TopoJSON bindings and utilities for Rust.
/// [\[Repository\]](<https://github.com/georust/topojson>)
pub struct Topojson {}

/// Customizable xtask-based commands for WASM projects.
/// [\[Repository\]](<https://github.com/rustminded/xtask-wasm>)
pub struct XtaskWasm {}

/// Customizable xtask-based watcher helper.
/// [\[Repository\]](<https://github.com/rustminded/xtask-watch>)
pub struct XtaskWatch {}

/// A port of [blueprintjs](<https://blueprintjs.com>) to Yew.
/// [\[Repository\]](<https://github.com/yewprint/yewprint>)
pub struct Yewprint {}

/// A system information library covering processes, CPUs, disks, components and networks.
/// [\[Repository\]](<https://github.com/GuillaumeGomez/sysinfo>)
pub struct Sysinfo {}

/// Immutable types and an `ImplicitClone` trait similar to `Copy`.
/// [\[Repository\]](<https://github.com/yewstack/implicit-clone>)
pub struct ImplicitClone {}

/// A GPT manager that can copy partitions between disks.
/// [\[Repository\]](<https://github.com/rust-disk-partition-management/gptman>)
pub struct Gptman {}

/// A MBR partition management in Rust.
/// [\[Repository\]](<https://github.com/rust-disk-partition-management/mbrman>)
pub struct Mbrman {}

// Professional Skills

/// French (Native), English (Professional Proficiency), Dutch (Basic).
pub enum SpokenLanguages {}

/// Managing project boards and repository settings.
pub enum ProjectOrganisation {}

/// Managing releases and iterative updates for open-source projects.
pub enum OpenSourceMaintenance {}

// Technical Skills

/// Rust, Bash, JavaScript, TypeScript, Python, C, SQL, HTML, CSS, Nix, Lua.
pub const LANGUAGES: () = ();

/// Yew, Rocket, Axum, Actix, Tokio, sqlx, Ratatui.
pub const FRAMEWORKS_AND_LIBRARIES: () = ();

/// Git, GitHub Actions (CI/CD), GitHub Projects, Neovim.
pub const TOOLS: () = ();

/// WebAssembly, CLI, TUI, cross-platform, Windows API, Web, Backend, Systems.
pub const PLATFORMS: () = ();

/// NixOS, Arch Linux, macOS, Windows.
pub const OPERATING_SYSTEMS: () = ();
