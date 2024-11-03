mod plugin;

#[cfg(target_family = "wasm")]
mod channel;
#[cfg(target_family = "wasm")]
mod web;

pub use plugin::{WebEvent, WebPlugin};
