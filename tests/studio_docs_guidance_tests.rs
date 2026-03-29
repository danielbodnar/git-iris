#![allow(clippy::unwrap_used)]

use std::fs;

const STUDIO_GUIDE_PATHS: &[&str] = &["docs/extending/index.md", "docs/extending/modes.md"];

#[test]
fn studio_extension_guides_no_longer_claim_a_pure_reducer_architecture() {
    for path in STUDIO_GUIDE_PATHS {
        let doc = fs::read_to_string(path).unwrap();
        assert!(
            !doc.contains("Pure Reducer Pattern"),
            "{path} still teaches the old pure reducer story"
        );
        assert!(
            !doc.contains("State transitions are pure functions"),
            "{path} still claims Studio is fully pure end-to-end"
        );
        assert!(
            doc.contains("reducer")
                && doc.contains("not a")
                && doc.contains("fully")
                && doc.contains("pure reducer architecture"),
            "{path} should describe Studio as reducer-centric but not fully pure"
        );
    }
}
