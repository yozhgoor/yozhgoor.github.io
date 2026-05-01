//! I started working with Rust in 2020 at [RustMinded](https://github.com/rustminded), where I
//! built a strong foundation in the language and the open-source ecosystem. Since then, I've built
//! Rust software across backend, systems, and developer tooling in both product and consulting
//! environments.

//! Along the way, I've gained practical experience with JavaScript and TypeScript in
//! collaborative teams, and I continue to contribute to open-source projects while using
//! Linux as my primary development platform.
//!
//! You can find me on [GitHub](https://github.com/yozhgoor) and
//! [Mastodon](https://mastodon.social/@yoz@hachyderm.io).
//! You can also send me an
//! [email](mailto:yohan.boogaert+recruit@protonmail.com).

// Jobs

/// Rust Developer role at RustMinded, evolving into consulting through open-source collaboration
/// and production-focused engineering on Rust software.
pub mod exp01_2020_current {}

/// Freelance mission as a Rust Developer at Cumul.io (now Luzmo), contributing to a Rust rewrite
/// of a data analytics query engine to improve performance and service interoperability in an AWS
/// microservices environment.
pub mod exp02_2022 {}

/// Freelance mission as a Rust Developer at Ystorian, working on a cybersecurity platform that
/// gathers BIOS, kernel, OS, and network information in parallel to assess vulnerabilities and
/// compliance across multiple systems.
pub mod exp03_2022_2025 {}

/// Multiple jobs with hands on the wheel (emergency response driver, airport shuttle, taxi,...).
pub mod exp04_2014_2020 {}

// Personal Projects

/// A CLI tool to create temporary Rust projects with pre-installed dependencies for faster
/// prototyping and experiments.
/// [\[Repository\]](<https://github.com/yozhgoor/cargo-temp>)
#[macro_export]
macro_rules! cargo_temp {
    () => {};
}

/// A Windows process-management library with an API inspired by `std::process`, built on top of
/// the Win32 API.
/// [\[Repository\]](<https://github.com/yozhgoor/CreateProcessW>)
#[macro_export]
macro_rules! create_process_w {
    () => {};
}

/// A CLI tool to help manage Rust project workflows efficiently.
/// [\[Repository\]](<https://github.com/yozhgoor/cargo-flow>)
#[macro_export]
macro_rules! cargo_flow {
    () => {};
}

/// Simple auto-grid layout utility for ratatui TUI applications.
/// [\[Repository\]](<https://github.com/yozhgoor/ratatui-auto-grid>)
#[macro_export]
macro_rules! ratatui_auto_grid {
    () => {};
}

/// Silence warnings in xtask-based builds without invalidating the dependency cache.
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

// Non-technical skills

/// Open-source project maintenance, including releases and iterative updates.
pub enum OssProjectMaintenance {}

/// Project organisation using
/// [GitHub Project](<https://docs.github.com/en/issues/trying-out-the-new-projects-experience/about-projects>)
/// and repository [settings](<https://docs.github.com/en/repositories/managing-your-repositories-settings-and-features>).
pub enum ProjectOrganisation {}

/// Continuous Integration workflows with [GitHub Actions](<https://github.com/actions>).
pub enum ContinuousIntegration {}

/// Web application deployment.
pub enum WebDeployment {}

// Technical skills

/// Yew, Rocket, Axum, Actix, Tokio, sqlx, Ratatui.
pub const FRAMEWORKS: () = ();

/// NixOS, Arch Linux, macOS, Windows.
pub const OS: () = ();

/// Rust, Bash, HTML, CSS, JavaScript, TypeScript, Python, C.
pub const PROGRAMMING_LANGUAGES: () = ();

/// French (Native), English (Professional Proficiency), Dutch (Basic).
pub const SPOKEN_LANGUAGES: () = ();

/// Git, WebAssembly, CLI, terminal UI, cross-platform development, systems, web, backend.
pub const TECHNOLOGIES: () = ();

// Personal traits

pub trait Autodidact {}

pub trait Enthusiast {}

pub trait Flexible {}

pub trait Passionate {}

pub trait TeamPlayer {}

pub trait WantToBuild {}

pub trait WantToLearn {}
