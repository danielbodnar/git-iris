use crate::config::Config;
use std::path::PathBuf;

#[test]
fn personal_config_dir_prefers_xdg_config_home() {
    let path = Config::resolve_personal_config_dir(
        Some(PathBuf::from("xdg-home")),
        Some(PathBuf::from("platform-config")),
    )
    .expect("xdg config home should be accepted");

    assert_eq!(path, PathBuf::from("xdg-home").join("git-iris"));
}

#[test]
fn personal_config_dir_falls_back_to_platform_config_dir() {
    let path = Config::resolve_personal_config_dir(None, Some(PathBuf::from("platform-config")))
        .expect("platform config dir should be accepted");

    assert_eq!(path, PathBuf::from("platform-config").join("git-iris"));
}

#[test]
fn personal_config_dir_ignores_empty_xdg_config_home() {
    let path = Config::resolve_personal_config_dir(
        Some(PathBuf::new()),
        Some(PathBuf::from("platform-config")),
    )
    .expect("empty xdg config home should fall back to platform config dir");

    assert_eq!(path, PathBuf::from("platform-config").join("git-iris"));
}
