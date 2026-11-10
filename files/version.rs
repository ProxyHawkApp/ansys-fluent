//! Static build info for Ansys Fluent.

pub const BUILD_VERSION: &str  = "v3.8.9";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "5a9e9f251d3e";
pub const BUILD_TIMESTAMP: u64 = 1758097508;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Ansys Fluent", BUILD_VERSION, BUILD_CHANNEL)
}
