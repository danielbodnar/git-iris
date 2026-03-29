use crate::companion::BranchMemory;
use chrono::{Duration, Utc};

#[test]
fn new_branch_memory_starts_without_a_returning_visit() {
    let memory = BranchMemory::new("feature/refactor".to_string());

    assert_eq!(memory.session_count, 0);
    assert!(!memory.is_returning_visit());
    assert!(memory.welcome_message().is_none());
}

#[test]
fn recorded_visits_can_surface_a_returning_welcome() {
    let mut memory = BranchMemory::new("feature/refactor".to_string());
    memory.record_visit();
    memory.last_visited = Utc::now() - Duration::minutes(10);

    assert_eq!(memory.session_count, 1);
    assert!(memory.is_returning_visit());

    let welcome = memory.welcome_message().expect("welcome message expected");
    assert!(welcome.contains("feature/refactor"));
    assert!(welcome.contains("Welcome back"));
}
