use crate::config::Config;
use std::path::PathBuf;

#[test]
fn personal_config_dir_prefers_xdg_config_home() {
    let path = Config::resolve_personal_config_dir(
        Some(PathBuf::from("/xdg-home")),
        Some(PathBuf::from("/home/user")),
        Some(PathBuf::from("/platform-config")),
        false,
    )
    .expect("xdg config home should be accepted");

    assert_eq!(path, PathBuf::from("/xdg-home").join("git-iris"));
}

#[test]
fn personal_config_dir_ignores_empty_xdg_config_home() {
    let path = Config::resolve_personal_config_dir(
        Some(PathBuf::new()),
        Some(PathBuf::from("/home/user")),
        Some(PathBuf::from("/platform-config")),
        false,
    )
    .expect("empty xdg config home should fall through");

    assert_eq!(
        path,
        PathBuf::from("/home/user").join(".config").join("git-iris")
    );
}

#[test]
fn personal_config_dir_prefers_xdg_style_home_over_platform_default() {
    // Even when a platform-native config dir is available, we return the
    // XDG-style `~/.config/git-iris` location so dev-tool behavior is
    // consistent across macOS/Linux/etc.
    let path = Config::resolve_personal_config_dir(
        None,
        Some(PathBuf::from("/home/user")),
        Some(PathBuf::from("/Users/user/Library/Application Support")),
        false,
    )
    .expect("should prefer XDG-style over platform config dir");

    assert_eq!(
        path,
        PathBuf::from("/home/user").join(".config").join("git-iris")
    );
}

#[test]
fn personal_config_dir_honors_legacy_macos_location_when_marker_present() {
    // Existing macOS users whose config already lives under
    // `~/Library/Application Support/git-iris` keep using it so we don't
    // silently orphan their settings after the XDG switch.
    let legacy_base = PathBuf::from("/Users/user/Library/Application Support");
    let path = Config::resolve_personal_config_dir(
        None,
        Some(PathBuf::from("/Users/user")),
        Some(legacy_base.clone()),
        true,
    )
    .expect("should honor legacy macOS location when marker is present");

    assert_eq!(path, legacy_base.join("git-iris"));
}

#[test]
fn personal_config_dir_xdg_override_wins_over_legacy_macos_marker() {
    // Even with a legacy macOS config present, an explicit XDG_CONFIG_HOME
    // still wins — setting the env var is a deliberate override.
    let path = Config::resolve_personal_config_dir(
        Some(PathBuf::from("/xdg-home")),
        Some(PathBuf::from("/Users/user")),
        Some(PathBuf::from("/Users/user/Library/Application Support")),
        true,
    )
    .expect("explicit XDG override should win");

    assert_eq!(path, PathBuf::from("/xdg-home").join("git-iris"));
}

#[test]
fn personal_config_dir_falls_back_to_platform_when_home_missing() {
    // Exotic environments without a HOME dir fall through to the platform
    // config dir rather than erroring out.
    let path = Config::resolve_personal_config_dir(
        None,
        None,
        Some(PathBuf::from("/platform-config")),
        false,
    )
    .expect("platform config dir should be accepted as last resort");

    assert_eq!(path, PathBuf::from("/platform-config").join("git-iris"));
}

#[test]
fn personal_config_dir_errors_when_every_source_is_missing() {
    let result = Config::resolve_personal_config_dir(None, None, None, false);
    assert!(
        result.is_err(),
        "resolution with no inputs should surface an error"
    );
}
