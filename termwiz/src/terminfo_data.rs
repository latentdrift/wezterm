//! Embedded terminfo database files.
//!
//! This module exposes compiled terminfo data as const byte slices,
//! allowing other crates to access them without relying on relative
//! file paths that break when dependencies are vendored.

/// Compiled terminfo data for the `wezterm` terminal type.
pub const WEZTERM: &[u8] = include_bytes!("../data/wezterm");

/// Compiled terminfo data for `xterm-256color`.
pub const XTERM_256COLOR: &[u8] = include_bytes!("../data/xterm-256color");
