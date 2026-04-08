use git_iris::gitmoji::{get_gitmoji, get_gitmoji_list, get_gitmoji_prompt_guide};

// Use our centralized test infrastructure
#[path = "test_utils.rs"]
mod test_utils;
use test_utils::TestAssertions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_gitmoji_list() {
        let list = get_gitmoji_list();

        // Use our centralized assertion for gitmoji validation
        TestAssertions::assert_contains_gitmoji(&list);

        // Additional specific checks
        assert!(list.contains("✨ - :feat: - Introduce new features"));
        assert!(list.contains("🐛 - :fix: - Fix a bug"));
        assert!(list.contains("📝 - :docs: - Add or update documentation"));
        assert!(list.contains("💄 - :style: - Add or update the UI and style files"));
        assert!(list.contains("♻️ - :refactor: - Refactor code"));
        assert!(list.contains("✅ - :test: - Add or update tests"));
        assert!(list.contains("🔨 - :chore: - Other changes that don't modify src or test files"));
    }

    #[test]
    fn test_get_gitmoji_prompt_guide() {
        let guide = get_gitmoji_prompt_guide();

        TestAssertions::assert_contains_gitmoji(&guide);
        assert!(guide.contains("Common gitmoji choices:"));
        assert!(guide.contains("- ✨ `:feat:` - Introduce new features"));
        assert!(guide.contains("- 🐛 `:fix:` - Fix a bug"));
        assert!(guide.contains("- 🔥 `:remove:` - Remove code or files"));
        assert!(!guide.contains(":accessibility:"));
        assert!(!guide.contains(":analytics:"));
    }

    #[test]
    fn test_get_gitmoji() {
        // Test valid gitmoji lookups
        assert_eq!(get_gitmoji("feat"), Some("✨"));
        assert_eq!(get_gitmoji("fix"), Some("🐛"));
        assert_eq!(get_gitmoji("docs"), Some("📝"));
        assert_eq!(get_gitmoji("style"), Some("💄"));
        assert_eq!(get_gitmoji("refactor"), Some("♻️"));
        assert_eq!(get_gitmoji("test"), Some("✅"));
        assert_eq!(get_gitmoji("chore"), Some("🔨"));

        // Test invalid lookup
        assert_eq!(get_gitmoji("unknown"), None);
    }
}
