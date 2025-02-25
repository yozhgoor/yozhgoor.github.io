//! I started working with Rust in 2020 at RustMinded, where I built a strong foundation in the
//! language and the open-source ecosystem. Since then, I’ve contributed to WebAssembly and Rust
//! tools, worked on rewriting a data query engine to improve performance, or helped on the
//! development of a cybersecurity platform for analyzing vulnerabilities on multiple systems.

//! Along the way, I’ve gained practical experience with JavaScript and TypeScript, working in team
//! environments to deliver effective solutions. I’m also an active contributor to open-source
//! projects and rely on Linux to streamline my development workflow.
//!
//! You can find me on [GitHub](https://github.com/yozhgoor) and
//! [Bluesky](https://bsky.app/profile/yozhgoor.bsky.social).
//! You can also send me an
//! [email](mailto:yohan.boogaert+recruit@protonmail.com).

// Jobs

/// Multiple jobs with hands on the wheel (emergency response driver, airport
/// shuttle, taxi,...).
pub mod exp04_2014_2020 {}

/// First job with hands on the keyboard at RustMinded. Discovering the OSS
/// world when learning how to become an Rustacean. Now working as Rust Developer Consultant.
pub mod exp03_2020_current {}

/// Freelance mission as a Rust Developer at Cumul.io rewriting in Rust a query engine for data
/// analytics using faster and more efficient queries and improving the relation of multiple
/// microservices in an AWS cloud environment.
pub mod exp02_2022 {}

/// Freelance mission as a Rust Developer at Ystorian working on a cybersecurity platform that
/// gathers information from the OS, BIOS, kernel or network on different systems in parallel to
/// assess security vulnerabilities and compliance level.
pub mod exp01_2022_2025 {}

// Personal Projects

/// A CLI tool that allows you to create a temporary new Rust project using
/// cargo with already installed dependencies.
/// [\[Repository\]](<https://github.com/yozhgoor/cargo-temp>)
#[macro_export]
macro_rules! cargo_temp {
    () => {};
}

/// A library that provides an API similar to `std::process` to create and handle processes on Windows
/// using the Win32 API.
/// [\[Repository\]](<https://github.com/yozhgoor/CreateProcessW>)
#[macro_export]
macro_rules! create_process_w {
    () => {};
}

/// A Yew and Yewprint playground using wasm-run.
/// [\[Repository\]](<https://github.com/yozhgoor/yewprint-playground>)
#[macro_export]
macro_rules! yewprint_playground {
    () => {};
}

// OSS Contributions

/// A developer tool to help you get up and running quickly with a new Rust project by leveraging a
/// pre-existing git repository as a template.
/// [\[Repository\]](<https://github.com/cargo-generate/cargo-generate>)
pub struct CargoGenerate {}

/// Date and Time for Rust.
/// [\[Repository\]](<https://github.com/chronotope/chrono>)
pub struct Chrono {}

/// French translation of the book "The Rust Programming Language".
/// [\[Repository\]](<https://github.com/Jimskapt/rust-book-fr>)
pub struct RustBookFr {}

/// Third-I, the only camera that replicates your hearing and your point of
/// view.
/// [\[Repository\]](<https://github.com/BigBoySystems/third-i-frontend>)
pub struct ThirdIFrontend {}

/// TopoJSON bindings and utilities for Rust.
/// [\[Repository\]](<https://github.com/georust/topojson>)
pub struct TopoJson {}

/// Customizable commands based on [xtask](https://github.com/matklad/cargo-xtask)
/// for your WASM project.
/// [\[Repository\]](<https://github.com/rustminded/xtask-wasm>)
pub struct XtaskWasm {}

/// A customizable helper to watch for changes in your xtask projects.
/// [\[Repository\]](<https://github.com/rustminded/xtask-watch>)
pub struct XtaskWatch {}

/// A port of [blueprintjs](<https://blueprintjs.com>) to Yew.
/// [\[Repository\]](<https://github.com/yewprint/yewprint>)
pub struct Yewprint {}

/// A library to get system information such as processes,
/// Cpus, disks, components and networks.
/// [\[Repository\]](<https://github.com/GuillaumeGomez/sysinfo>)
pub struct Sysinfo {}

/// Immutable types and ImplicitClone trait similar to Copy.
/// [\[Repository\]](<https://github.com/yewstack/implicit-clone>)
pub struct ImplicitClone {}

/// A GPT manager that allows you to copy partitions from one disk to another.
/// [\[Repository\]](<https://github.com/rust-disk-partition-management/gptman>)
pub struct Gptman {}

// Non-technical skills

/// Release, update and maintenance of an OSS project.
pub enum OssProjectMaintenance {}

/// Project organisation using
/// [GitHub Project](<https://docs.github.com/en/issues/trying-out-the-new-projects-experience/about-projects>)
/// and [GitHub settings](<https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features>).
pub enum ProjectOrganisation {}

/// Continuous Integration via [GitHub Actions](<https://github.com/actions>).
pub enum ContinuousIntegration {}

/// Web application deployment using [Netlify](<https://www.netlify.com>).
pub enum WebDeployment {}

// Technical skills

/// Yew, Rocket, Axum, Actix, Tokio, sqlx.
pub const FRAMEWORKS: () = ();

/// Nixos, Arch Linux, Pop!_OS, macOS, Windows, Debian, Tails.
pub const OS: () = ();

/// Rust, Bash, HTML, CSS, JavaScript.
pub const PROGRAMMING_LANGUAGES: () = ();

/// French (5. Native), English (3. Minimum Professional Proficiency), Dutch (1. Basic).
pub const SPOKEN_LANGUAGES: () = ();

/// Git, WebAssembly, CLI, Terminal UI, Platform agnosticism, System, Web, Backend.
pub const TECHNOLOGIES: () = ();

// Personal traits

pub trait Autodidact {}

pub trait Enthusiast {}

pub trait Flexible {}

pub trait Passionate {}

pub trait TeamPlayer {}

pub trait WantToBuild {}

pub trait WantToLearn {}
