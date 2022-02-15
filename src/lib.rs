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
//! You can find me on [GitHub](https://github.com/yozhgoor) and
//! [Twitter](https://twitter.com/yozhgoor).
//! You can also send me an
//! [email](mailto:yohan.boogaert+recruit@protonmail.com).

// Jobs

/// Multiple jobs with hands on the wheel (emergency response driver, airport
/// shuttle, taxi,...).
pub mod between_2014_and_2020 {}

/// First job with hands on the keyboard at RustMinded. Discovering the OSS
/// world when learning how to become an Rustacean and a developer.
pub mod current {}

// Personal Projects

/// A CLI tool that allows you to create a temporary new Rust project using
/// cargo with already installed dependencies.
/// [\[Repository\]](<https://github.com/rustminded/cargo-temp>)
#[macro_export]
macro_rules! cargo_temp {
    () => {};
}

/// A crate to create and handle processes on Windows (similar too `std::process`).
/// [\[Repository\]](<https://github.com/yozhgoor/CreateProcessW>)
#[macro_export]
macro_rules! create_process_w {
    () => {};
}

/// My vision of the Rust Programming Language as an introduction to Rust.
/// [\[Repository\]](<https://github.com/yozhgoor/intro-to-rust>)
#[macro_export]
macro_rules! intro_to_rust {
    () => {};
}

/// Yet Another To-Do App in your Terminal. (Under development)
/// [\[Repository\]](<https://github.com/yozhgoor/tui-do>)
#[macro_export]
macro_rules! tui_do {
    () => {};
}

/// A Yew and Yewprint playground using wasm-run.
/// [\[Repository\]](<https://github.com/yozhgoor/yewprint-playground>)
#[macro_export]
macro_rules! yewprint_playground {
    () => {};
}

// OSS Contributions

/// A CLI tool that compares a crate's public API between two different
/// branches, shows what changed, and suggests the next version according to
/// semver.
/// [\[Repository\]](<https://github.com/iomentum/cargo-breaking>)
pub struct CargoBreaking {}

/// The place to be hired as an awesome Rustacean. (Under development)
/// [\[Repository\]](<https://github.com/rustminded/recruit-rust>)
pub struct RecruitRust {}

/// French translation of the book "The Rust Programming Language".
/// [\[Repository\]](<https://github.com/Jimskapt/rust-book-fr>)
pub struct RustBookFr {}

/// Third-I, the only camera that replicates your hearing and your point of
/// view.
/// [\[Repository\]](<https://github.com/BigBoySystems/third-i-frontend>) |
/// [\[Site\]](<https://bigboysystems.com/>)
pub struct ThirdIFrontend {}

/// Bundles and tooling for Rust WASM frontend application.
/// [\[Repository\]](<https://github.com/wasmbl/wasmbl>)
pub struct WasmRun {}

/// Customizable commands based on [xtask](https://github.com/matklad/cargo-xtask)
/// for your WASM project.
/// [\[Repository\]](<https://github.com/rustminded/xtask-wasm>)
pub struct XtaskWasm {}

/// A customizable helper to watch for changes in your xtask projects.
/// [\Repository\]](<https://github.com/rustminded/xtask-watch>)
pub struct XtaskWatch {}

/// Port of [blueprintjs](<https://blueprintjs.com>) to Yew.
/// [\[Repository\]](<https://github.com/yewprint/yewprint>)
pub struct Yewprint {}

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

/// Git, WASM, CLI, Terminal UI.
pub const TECHNOLOGIES: () = ();

// Personal traits

pub trait Autodidact {}

pub trait Enthusiast {}

pub trait Flexible {}

pub trait Passionate {}

pub trait TeamPlayer {}

pub trait WantToBuild {}

pub trait WantToLearn {}
