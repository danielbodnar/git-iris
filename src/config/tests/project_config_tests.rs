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
