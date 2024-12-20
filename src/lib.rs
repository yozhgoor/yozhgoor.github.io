//! I'm a Former emergency response driver, who always had a big interest in
//! programming and technology. I've been running Linux for 6 years, I tried
//! many distributions (Ubuntu, Tails, Manjaro,...). I use Arch Linux as my
//! daily driver because Arch gives me the opportunity to really discover what
//! it's going on under the hood.
//!
//! I had the chance to get insight knowledge on Rust in 2020 from a friend that
//! became my mentor. I've got the opportunity to learn Rust at RustMinded
//! including all the technical and non-technical skills to become a developer.
//!
//! You can find me on [GitHub](https://github.com/yozhgoor),
//! [Twitter](https://twitter.com/yozhgoor) and [Mastodon](https://hachyderm.io/@yozhgoor).
//! You can also send me an
//! [email](mailto:yohan.boogaert+recruit@protonmail.com).

// Jobs

/// Multiple jobs with hands on the wheel (emergency response driver, airport
/// shuttle, taxi,...).
pub mod exp04_2014_2020 {}

/// First job with hands on the keyboard at RustMinded. Discovering the OSS
/// world when learning how to become an Rustacean.
pub mod exp03_2020_2022 {}

/// Freelance mission as a Rust Developer at Cumul.io rewriting in Rust a query engine for data
/// analytics using faster and more efficient queries and improving the relation of multiple
/// microservices in an AWS cloud environment.
pub mod exp02_2022 {}

/// Freelance mission as a Rust Developer at Ystorian working on a cybersecurity platform that
/// gathers information from the OS, BIOS, kernel or network on different systems in parallel to
/// assess security vulnerabilities and compliance level.
pub mod exp01_2022_2024 {}

// Personal Projects

/// A CLI tool that allows you to create a temporary new Rust project using
/// cargo with already installed dependencies.
/// [\[Repository\]](<https://github.com/rustminded/cargo-temp>)
#[macro_export]
macro_rules! cargo_temp {
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
/// [\[Repository\]](<https://github.com/BigBoySystems/third-i-frontend>) |
/// [\[Site\]](<https://bigboysystems.com/>)
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

/// Port of [blueprintjs](<https://blueprintjs.com>) to Yew.
/// [\[Repository\]](<https://github.com/yewprint/yewprint>)
pub struct Yewprint {}

/// Library to get system information such as processes,
/// Cpus, disks, components and networks.
/// [\[Repository\]](<https://github.com/GuillaumeGomez/sysinfo>)
pub struct Sysinfo {}

// Non-technical skills

/// Release, update and maintenance of an OSS project.
pub enum OssProjectMaintenance {}

/// Project organisation using
/// [GitHub Project](<https://docs.github.com/en/issues/trying-out-the-new-projects-experience/about-projects>)
/// and [GitHub settings](<https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features>).
pub enum ProjectOrganisation {}

/// Continuous Integration via [GitHub Actions](<https://github.com/actions>).
pub enum SettingUpCI {}

/// Web application deployment using [Netlify](<https://www.netlify.com>).
pub enum WebDeployment {}

// Technical skills

/// Yew.
pub const FRAMEWORKS: () = ();

/// Arch Linux, Pop!_OS.
pub const OS: () = ();

/// Rust, Bash.
pub const PROGRAMMING_LANGUAGES: () = ();

/// French (5. Native), English (3. Minimum Professional Proficiency).
pub const SPOKEN_LANGUAGES: () = ();

/// Git, WASM, CLI, Terminal UI, Platform agnosticism.
pub const TECHNOLOGIES: () = ();

// Personal traits

pub trait Autodidact {}

pub trait Enthusiast {}

pub trait Flexible {}

pub trait Passionate {}

pub trait TeamPlayer {}

pub trait WantToBuild {}

pub trait WantToLearn {}
