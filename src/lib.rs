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

/// Junior at RustMinded
pub mod software_developer {}

/// Volunteer at Centre de Secours Monceau
pub mod emergency_medical_technician {}

/// Ambulance, Shuttle, Taxi,...
pub mod professional_driver {}

/// A CLI tool that allows you to create a temporary new Rust project using
/// cargo with already installed dependencies.
/// [site](https://github.com/rustminded/cargo-temp)
pub struct CargoTemp {}

/// My vision of the Rust Programming Language as an introduction to Rust.
/// [site](https://github.com/yozhgoor/intro-to-rust)
pub struct IntroToRust {}

/// A Yew and Yewprint playground using wasmbl.
/// [site](https://github.com/yozhgoor/yewprint-playground)
pub struct YewprintPlayground {}

/// Port of blueprintjs.com to Yew.
/// [site](https://github.com/yewprint/yewprint)
pub enum Yewprint {}

/// Bundles and tooling for Rust WASM frontend application.
/// [site](https://github.com/wasmbl/wasmbl)
pub enum Wasmbl {}

/// A CLI tool that compares a crate's public API between two different
/// branches, shows what changed, and suggests the next version according to
/// semver.
/// [site](https://github.com/iomentum/cargo-breaking)
pub enum CargoBreaking {}

/// French translation of the book "The Rust Programming Language".
/// [site](https://github.com/Jimskapt/rust-book-fr)
pub enum RustBookFr {}

/// "Third-I", the only camera that replicates your hearing and your point of
/// view.
/// [site](https://github.com/BigBoySystems/third-i-frontend)
pub enum ThirdIFrontend {}

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
