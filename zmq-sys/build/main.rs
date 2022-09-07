#[cfg_attr(feature = "vendored", target_os = "macos", path = "pkg_config.rs")]
#[cfg_attr(feature = "vendored", path = "vendored.rs")]
#[cfg_attr(not(feature = "vendored"), path = "pkg_config.rs")]
mod find;

fn main() {
    find::configure()
}
