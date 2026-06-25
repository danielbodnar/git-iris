use super::IrisAgentService;
use crate::config::Config;

#[test]
fn saved_config_instructions_are_pr_defaults() {
    let config = Config {
        instructions: "Lead with reviewer context.".to_string(),
        ..Config::default()
    };

    assert_eq!(
        IrisAgentService::custom_instructions_for_capability(&config, "pr", None),
        Some("Lead with reviewer context.")
    );
    assert_eq!(
        IrisAgentService::custom_instructions_for_capability(&config, "commit", None),
        None
    );
}

#[test]
fn runtime_instructions_apply_to_any_capability() {
    let config = Config {
        instructions: "Saved PR default.".to_string(),
        ..Config::default()
    };

    assert_eq!(
        IrisAgentService::custom_instructions_for_capability(
            &config,
            "commit",
            Some("One-shot commit instruction."),
        ),
        Some("One-shot commit instruction.")
    );
    assert_eq!(
        IrisAgentService::custom_instructions_for_capability(
            &config,
            "review",
            Some("One-shot review instruction."),
        ),
        Some("One-shot review instruction.")
    );
}

#[test]
fn blank_runtime_instructions_clear_saved_pr_defaults() {
    let config = Config {
        instructions: "Saved PR default.".to_string(),
        ..Config::default()
    };

    assert_eq!(
        IrisAgentService::custom_instructions_for_capability(&config, "pr", Some("   ")),
        None
    );
}
