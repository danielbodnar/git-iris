use git_iris::github::find_pull_request_template;
use std::fs;
use tempfile::tempdir;

#[test]
fn finds_github_pull_request_template_first() {
    let dir = tempdir().expect("tempdir should be created");
    fs::create_dir_all(dir.path().join(".github")).expect("template dir should be created");
    fs::write(
        dir.path().join(".github/pull_request_template.md"),
        "## Summary\n",
    )
    .expect("template should be written");
    fs::write(dir.path().join("pull_request_template.md"), "root template")
        .expect("root template should be written");

    let template = find_pull_request_template(dir.path())
        .expect("template lookup should succeed")
        .expect("template should be found");

    assert_eq!(template.path, ".github/pull_request_template.md");
    assert_eq!(template.body, "## Summary\n");
}

#[test]
fn finds_single_template_from_template_directory() {
    let dir = tempdir().expect("tempdir should be created");
    fs::create_dir_all(dir.path().join(".github/PULL_REQUEST_TEMPLATE"))
        .expect("template dir should be created");
    fs::write(
        dir.path().join(".github/PULL_REQUEST_TEMPLATE/feature.md"),
        "## Feature\n",
    )
    .expect("template should be written");

    let template = find_pull_request_template(dir.path())
        .expect("template lookup should succeed")
        .expect("template should be found");

    assert_eq!(template.path, ".github/PULL_REQUEST_TEMPLATE/feature.md");
    assert_eq!(template.body, "## Feature\n");
}

#[test]
fn skips_ambiguous_template_directory() {
    let dir = tempdir().expect("tempdir should be created");
    fs::create_dir_all(dir.path().join(".github/PULL_REQUEST_TEMPLATE"))
        .expect("template dir should be created");
    fs::write(
        dir.path().join(".github/PULL_REQUEST_TEMPLATE/feature.md"),
        "## Feature\n",
    )
    .expect("feature template should be written");
    fs::write(
        dir.path().join(".github/PULL_REQUEST_TEMPLATE/bug.md"),
        "## Bug\n",
    )
    .expect("bug template should be written");

    let template = find_pull_request_template(dir.path()).expect("template lookup should succeed");

    assert!(template.is_none());
}
