use crate::config::Config;

#[test]
fn explicit_project_defaults_override_personal_config() {
    let mut personal_config = Config::default();
    personal_config.default_provider = "anthropic".to_string();
    personal_config.use_gitmoji = false;
    personal_config.instructions = "personal instructions".to_string();
    personal_config.instruction_preset = "conventional".to_string();
    personal_config.theme = "midnight".to_string();
    personal_config.subagent_timeout_secs = 300;

    let project_toml = r#"
default_provider = "openai"
use_gitmoji = true
instructions = ""
instruction_preset = "default"
theme = ""
subagent_timeout_secs = 120
"#;
    let mut project_config: Config = toml::from_str(project_toml).expect("valid project config");
    project_config.is_project_config = true;
    let project_source: toml::Value =
        toml::from_str(project_toml).expect("valid project config source");

    personal_config.merge_loaded_project_config(project_config, &project_source);

    assert_eq!(personal_config.default_provider, "openai");
    assert!(personal_config.use_gitmoji);
    assert_eq!(personal_config.instructions, "");
    assert_eq!(personal_config.instruction_preset, "default");
    assert_eq!(personal_config.theme, "");
    assert_eq!(personal_config.subagent_timeout_secs, 120);
}

#[test]
fn migrate_legacy_gemini_provider_to_google() {
    let config_toml = r#"
default_provider = "gemini"

[providers.gemini]
api_key = "AIza-example-key"
model = "gemini-3-pro-preview"
fast_model = "gemini-2.5-flash"
"#;

    let config: Config = toml::from_str(config_toml).expect("valid config");
    let migrated = Config::migrate_if_needed(config);

    assert_eq!(migrated.default_provider, "google");
    assert!(!migrated.providers.contains_key("gemini"));

    let google_config = migrated
        .providers
        .get("google")
        .expect("google config should exist after migration");
    assert_eq!(google_config.model, "gemini-3-pro-preview");
    assert_eq!(
        google_config.fast_model.as_deref(),
        Some("gemini-2.5-flash")
    );
    assert!(migrated.validate().is_ok());
}

#[test]
fn canonical_provider_config_wins_over_legacy_alias() {
    let config_toml = r#"
default_provider = "gemini"

[providers.google]
api_key = "AIza-canonical"
model = "gemini-3-pro-preview"

[providers.gemini]
api_key = "AIza-legacy"
model = "gemini-2.5-flash"
"#;

    let config: Config = toml::from_str(config_toml).expect("valid config");
    let migrated = Config::migrate_if_needed(config);

    assert_eq!(migrated.default_provider, "google");
    assert!(!migrated.providers.contains_key("gemini"));

    let google_config = migrated
        .providers
        .get("google")
        .expect("google config should exist after migration");
    assert_eq!(google_config.api_key, "AIza-canonical");
    assert_eq!(google_config.model, "gemini-3-pro-preview");
}
