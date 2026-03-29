use git_iris::config::Config;
use git2::Repository;
use std::fs;

#[path = "test_utils.rs"]
mod test_utils;
use test_utils::setup_git_repo;

#[tokio::test]
async fn test_get_git_info_with_excluded_files() {
    let (temp_dir, git_repo) = setup_git_repo();
    let config = Config::default();

    // Create files that should be excluded
    fs::create_dir_all(temp_dir.path().join("node_modules"))
        .expect("Failed to create node_modules directory");
    fs::write(
        temp_dir.path().join("node_modules/excluded.js"),
        "console.log('excluded');",
    )
    .expect("Failed to write excluded file");
    fs::write(temp_dir.path().join(".gitignore"), "node_modules/")
        .expect("Failed to write .gitignore");
    fs::write(
        temp_dir.path().join("package-lock.json"),
        r#"{"name": "test-package"}"#,
    )
    .expect("Failed to write package-lock.json");

    // Create a non-excluded file
    fs::write(
        temp_dir.path().join("included.js"),
        "console.log('included');",
    )
    .expect("Failed to write included file");

    // Stage all files
    let repo = Repository::open(temp_dir.path()).expect("Failed to open repository");
    let mut index = repo.index().expect("Failed to get repository index");
    index
        .add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .expect("Failed to add all files to index");
    index.write().expect("Failed to write index");

    let context = git_repo
        .get_git_info(&config)
        .expect("Failed to get git info");

    // Check excluded files
    let excluded_files: Vec<_> = context
        .staged_files
        .iter()
        .filter(|file| file.content_excluded)
        .collect();

    assert!(!excluded_files.is_empty(), "Should have excluded files");

    println!("{excluded_files:?}");
    assert!(
        excluded_files
            .iter()
            .any(|file| file.path == "package-lock.json")
    );

    for file in &excluded_files {
        assert_eq!(file.diff, "[Content excluded]");
    }

    // Check included file
    let included_files: Vec<_> = context
        .staged_files
        .iter()
        .filter(|file| !file.content_excluded)
        .collect();

    assert!(!included_files.is_empty(), "Should have included files");
    assert!(included_files.iter().any(|file| file.path == "included.js"));

    for file in &included_files {
        assert_ne!(file.diff, "[Content excluded]");
    }
}

#[tokio::test]
async fn test_multiple_staged_files_with_exclusions() {
    let (temp_dir, git_repo) = setup_git_repo();
    let config = Config::default();

    // Create files that should be excluded
    fs::create_dir_all(temp_dir.path().join(".vscode"))
        .expect("Failed to create .vscode directory");
    fs::write(
        temp_dir.path().join(".vscode/settings.json"),
        r#"{"editor.formatOnSave": true}"#,
    )
    .expect("Failed to write .vscode/settings.json");
    fs::write(
        temp_dir.path().join("large.min.js"),
        "console.log('minified')",
    )
    .expect("Failed to write large.min.js");

    // Create non-excluded files
    for i in 1..=3 {
        fs::write(
            temp_dir.path().join(format!("file{i}.txt")),
            format!("Content {i}"),
        )
        .expect("Failed to write non-excluded file");
    }

    // Stage all files
    let repo = Repository::open(temp_dir.path()).expect("Failed to open repository");
    let mut index = repo.index().expect("Failed to get repository index");
    index
        .add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .expect("Failed to add all files to index");
    index.write().expect("Failed to write index");

    let context = git_repo
        .get_git_info(&config)
        .expect("Failed to get git info");

    assert_eq!(context.staged_files.len(), 5);

    let excluded_files: Vec<_> = context
        .staged_files
        .iter()
        .filter(|file| file.content_excluded)
        .collect();
    assert_eq!(excluded_files.len(), 2);

    let included_files: Vec<_> = context
        .staged_files
        .iter()
        .filter(|file| !file.content_excluded)
        .collect();
    assert_eq!(included_files.len(), 3);

    for file in &excluded_files {
        assert!(file.path.contains(".vscode") || file.path.contains(".min.js"));
        assert_eq!(file.diff, "[Content excluded]");
    }

    for file in &included_files {
        assert!(
            file.path.starts_with("file")
                && std::path::Path::new(&file.path)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("txt"))
        );
        assert_ne!(file.diff, "[Content excluded]");
    }
}
