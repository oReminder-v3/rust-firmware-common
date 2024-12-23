#[cfg(debug_assertions)]
pub const SERVER_ADDRESS: &str = "http://localhost:8000";

#[cfg(not(debug_assertions))]
pub const SERVER_ADDRESS: &str = "https://oreminder.com/api";
