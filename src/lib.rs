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
//! You can find me on [Github](https://github.com/yozhgoor) and
//! [Twitter](https://twitter.com/yozhgoor).
//! You can also send me an
//! [email](mailto:yohan.boogaert+recruit@protonmail.com).

// Jobs

/// Junior at RustMinded - Current
pub mod software_developer {}

/// Volunteer at Centre de Secours Monceau - 2020
pub mod emergency_medical_technician {}

/// Ambulance, Shuttle, Taxi,... - 2014 to 2019
pub mod professional_driver {}

// Personal Projects

/// A CLI tool that allows you to create a temporary new Rust project using
/// cargo with already installed dependencies.
/// [\[Repository\]](<https://github.com/rustminded/cargo-temp>)
#[macro_export]
macro_rules! cargo_temp {
    () => {};
}

/// My vision of the Rust Programming Language as an introduction to Rust.
/// [\[Repository\]](<https://github.com/yozhgoor/intro-to-rust>)
#[macro_export]
macro_rules! intro_to_rust {
    () => {};
}

/// A Yew and Yewprint playground using wasm-run.
/// [\[Repository\]](<https://github.com/yozhgoor/yewprint-playground>)
#[macro_export]
macro_rules! yewprint_playground {
    () => {};
}

/// A crate to create and handle processes on Windows (similar too `std::process`).
/// [\[Repository\]](<https://github.com/yozhgoor/CreateProcessW>)
#[macro_export]
macro_rules! CreateProcessW {
    () => {};
}

/// Yet Another To-Do App in your Terminal. (Under development)
/// [\[Repository\]](<https://github.com/yozhgoor/tui-do>)
#[macro_export]
macro_rules! tui_do {
    () => {};
}

// OSS Contributions

/// Port of [blueprintjs](<https://blueprintjs.com>) to Yew.
/// [\[Repository\]](<https://github.com/yewprint/yewprint>)
pub struct Yewprint {}

/// Bundles and tooling for Rust WASM frontend application.
/// [\[Repository\]](<https://github.com/wasmbl/wasmbl>)
pub struct WasmRun {}

/// A CLI tool that compares a crate's public API between two different
/// branches, shows what changed, and suggests the next version according to
/// semver.
/// [\[Repository\]](<https://github.com/iomentum/cargo-breaking>)
pub struct CargoBreaking {}

/// French translation of the book "The Rust Programming Language".
/// [\[Repository\]](<https://github.com/Jimskapt/rust-book-fr>)
pub struct RustBookFr {}

/// Third-I, the only camera that replicates your hearing and your point of
/// view.
/// [\[Repository\]](<https://github.com/BigBoySystems/third-i-frontend>) |
/// [\[Site\]](<https://bigboysystems.com/>)
pub struct ThirdIFrontend {}

/// The place to be hired as an awesome Rustacean. (Under development)
/// [\[Repository\]](<https://github.com/rustminded/recruit-rust>)
pub struct RecruitRust {}

/// Customizable commands based on [xtask](https://github.com/matklad/cargo-xtask)
/// for your WASM project.
/// [\[Repository\]](<https://github.com/rustminded/xtask-wasm>)
pub struct XtaskWasm {}

// Non-technical skills

/// Web application deployment using [Netlify](<https://www.netlify.com>).
pub enum WebDeployment {}

/// Continuous Integration via [GitHub Actions](<https://github.com/actions>).
pub enum SettingUpCI {}

/// Release, update and maintenance of an OSS project.
pub enum OssProjectMaintenance {}

// Technical skills

/// French (5. Native), English (3. Minimum Professional Proficiency).
pub const SPOKEN_LANGUAGES: () = ();

/// Rust, Bash.
pub const PROGRAMMING_LANGUAGES: () = ();

/// Yew.
pub const FRAMEWORKS: () = ();

/// Git, WASM, CLI, Terminal UI.
pub const TECHNOLOGIES: () = ();

/// Arch Linux, Pop!_OS.
pub const OS: () = ();

// Personal traits

pub trait TeamPlayer {}

pub trait Autodidact {}

pub trait Passionate {}

pub trait Flexible {}

pub trait Enthusiast {}

pub trait WantToLearn {}

pub trait WantToBuild {}
