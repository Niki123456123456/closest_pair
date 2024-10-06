#![warn(clippy::all, rust_2018_idioms)]
mod twod;
mod app;
mod nd;
mod bench;
pub use app::App;
pub use twod::*;