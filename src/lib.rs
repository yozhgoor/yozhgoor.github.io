//! A former emergency response driver who has always been interested in
//! programming and technology. I've been running Linux for 6 years, I tried
//! many distributions (Ubuntu, Tails, Manjaro,...). I use Arch Linux as my
//! daily driver because Arch gives me the opportunity to really discover what
//! it's going under the hood.
//!
//! I heard about Rust in 2020 from a friend who shared her knowledge with me
//! about programming and helped me with Linux. Recently that friend (who is my
//! mentor now) gave me the opportunity to realize my dream to become a software
//! developer! I'm currently learning Rust at RustMinded, including all the
//! technical and non-technical skills.
//!
//! You can find me on [Github](https://github.com/yozhgoor) and
//! [Twitter](https://twitter.com/yozhgoor).
//! You can also send me a [mail](mailto:recruit-yozhgoor@protonmail.com).

/// Junior at RustMinded - Current
pub mod software_developer {}

/// Volunteer at Centre de Secours Monceau - 2020
pub mod emergency_medical_technician {}

/// Ambulance, Shuttle, Taxi,... - 2014 to 2019
pub mod professional_driver {}

/// A CLI tool that allows you to create a temporary new Rust project using
/// cargo with already installed dependencies.
/// [\[Repository\]](<https://github.com/rustminded/cargo-temp>)
pub struct CargoTemp {}

/// My vision of the Rust Programming Language as an introduction to Rust.
/// [\[Repository\]](<https://github.com/yozhgoor/intro-to-rust>)
pub struct IntroToRust {}

/// A Yew and Yewprint playground using wasmbl.
/// [\[Repository\]](<https://github.com/yozhgoor/yewprint-playground>)
pub struct YewprintPlayground {}

/// Port of blueprintjs.com to Yew.
/// [\[Repository\]](<https://github.com/yewprint/yewprint>)
pub enum Yewprint {}

/// Bundles and tooling for Rust WASM frontend application.
/// [\[Repository\]](<https://github.com/wasmbl/wasmbl>)
pub enum Wasmbl {}

/// A CLI tool that compares a crate's public API between two different
/// branches, shows what changed, and suggests the next version according to
/// semver.
/// [\[Repository\]](<https://github.com/iomentum/cargo-breaking>)
pub enum CargoBreaking {}

/// French translation of the book "The Rust Programming Language".
/// [\[Repository\]](<https://github.com/Jimskapt/rust-book-fr>)
pub enum RustBookFr {}

/// Third-I, the only camera that replicates your hearing and your point of
/// view.
/// [\[Repository\]](<https://github.com/BigBoySystems/third-i-frontend>) |
/// [\[Site\]](<https://bigboysystems.com/>)
pub enum ThirdiFrontend {}

/// French (5. Native), English (3. Minimum Professional Proficiency).
pub const SPOKEN_LANGUAGES: (&str, &str) = ("French", "English");

/// Rust, Bash.
pub const PROGRAMMING_LANGUAGES: (&str, &str) = ("Rust", "Bash");

/// Yew.
pub const FRAMEWORKS: &str = "Yew";

/// Git, WASM, CLI.
pub const TECHNOLOGIES: (&str, &str, &str) = ("Git", "WASM", "CLI");

/// Arch Linux, Pop!_OS.
pub const OS: (&str, &str) = ("Arch Linux", "Pop!_OS");

pub trait TeamPlayer {}

pub trait Autodidact {}

pub trait Passionnate {}

pub trait Flexible {}

pub trait Enthusiast {}

pub trait WantToLearn {}

pub trait WantToBuild {}
