pub mod app;
pub mod doctor;
pub mod completion;

pub use doctor::run as doctor;
pub use completion::run as completion;
pub use app::{Cli, Commands};