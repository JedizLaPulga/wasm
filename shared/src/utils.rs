//! Shared utility functions

pub fn format_version() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

pub fn get_author() -> &'static str {
    env!("CARGO_PKG_AUTHORS")
}
