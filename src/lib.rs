//! A former emergency response driver who always been interested in programming
//! and technology. I've been running Linux for 6 years, I tried many
//! distributions (Ubuntu, Tails, Manjaro,...). I use Arch Linux as my daily
//! driver because Arch gives me the opportunity to really discover what it's
//! going under the hood and nearly build my desktop from scratch.
//!
//! I heard about Rust in 2020 from a friend who shared her knowledge with me
//! about programming and helped me with Linux. Recently that friend (who is my
//! mentor now) gave me the opportunity to realize my dreams and become a
//! developer! I now learn how to write Rust code and how to work like a
//! developer at RustMinded.
//!
//! You can find me on [Github](https://github.com/Yozhgoor) and
//! [Twitter](https://twitter.com/yozhgoor).
//! You can also send me a [mail](mailto:yohan.boogaert@protonmail.com).

// experiences

/// Volunteer at Centre de Secours Monceau - 2020
pub mod emergency_medical_technician {}

/// Junior at RustMinded - Current
pub mod software_developer {}

/// Ambulance, Shuttle, Taxi,... - 2014 to 2019
pub mod professional_driver {}

// Personal projects

/// Project - A CLI tool that allows you to create a temporary new Rust project
/// using cargo with already installed dependencies.
/// [site](https://github.com/rustminded/cargo-temp)
pub struct CargoTemp {}

/// Project - My vision of the Rust Programming Language as an introduction to
/// Rust. [site](https://github.com/yozhgoor/intro-to-rust)
pub struct IntroToRust {}

/// Project - A Yew and Yewprint playground using wasmbl.
/// [site](https://github.com/yozhgoor/yewprint-playground)
pub struct YewprintPlayground {}

// Projects I've been active lately

/// Contributions - A CLI tool that compares a crate's public API between two
/// different branches, shows what changed, and suggests the next version
/// according to semver. [site](https://github.com/iomentum/cargo-breaking)
pub enum CargoBreaking {}

/// Contributions - French translation of the book "The Rust Programming
/// Language". [site](https://github.com/Jimskapt/rust-book-fr)
pub enum RustBookFr {}

/// Contributions - "Third-I", the only camera that replicates your hearing and
/// your point ofview.
/// [site](https://github.com/BigBoySystems/third-i-frontend)
pub enum ThirdIFrontend {}

/// Contributions - Bundles and tooling for Rust WASM frontend application.
/// [site](https://github.com/wasmbl/wasmbl)
pub enum Wasmbl {}

/// Contributions - Port of blueprintjs.com to Yew.
/// [site](https://github.com/yewprint/yewprint)
pub enum Yewprint {}

// Technical skills

/// Yew.
pub const FRAMEWORKS: &str = "Yew";

/// Arch Linux, Pop!_OS.
pub const OS: (&str, &str) = ("Arch Linux", "Pop!_OS");

/// Rust, Bash.
pub const PROGRAMMING_LANGUAGES: (&str, &str) = ("Rust", "Bash");

/// French (5. Native), English (3. Minimum Professional Proficiency).
pub const SPOKEN_LANGUAGES: (&str, &str) = ("French", "English");

/// Git, WASM, CLI.
pub const TECHNOLOGIES: (&str, &str, &str) = ("Git", "WASM", "CLI");

// Soft Skills

pub trait Autodidact {}

pub trait Enthusiast {}

pub trait Flexible {}

pub trait Passionnate {}

pub trait TeamPlayer {}

pub trait WantToLearn {}

pub trait WantToBuild {}
