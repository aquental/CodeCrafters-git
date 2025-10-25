use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

// Import functions from the main crate
use code_crafters_git::{create_dir, init_git_repo_at};

#[test]
fn test_init_creates_git_directory_structure() {
    // Create a temporary directory that will be automatically cleaned up
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // Initialize git repo in the temporary directory
    let result = init_git_repo_at(temp_path.to_str().unwrap());
    assert!(
        result.is_ok(),
        "Failed to initialize git repo: {:?}",
        result
    );

    // Verify .git directory was created
    let git_dir = temp_path.join(".git");
    assert!(git_dir.exists(), ".git directory should exist");
    assert!(git_dir.is_dir(), ".git should be a directory");

    // Verify subdirectories were created
    assert!(
        git_dir.join("objects").exists(),
        ".git/objects should exist"
    );
    assert!(git_dir.join("refs").exists(), ".git/refs should exist");

    // Verify HEAD file was created with correct content
    let head_path = git_dir.join("HEAD");
    assert!(head_path.exists(), ".git/HEAD should exist");

    let head_content = fs::read_to_string(&head_path).expect("Failed to read HEAD file");
    assert_eq!(
        head_content, "ref: refs/heads/master\n",
        "HEAD file should contain correct reference"
    );
}

#[test]
fn test_init_fails_when_git_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    // First initialization should succeed
    let result = init_git_repo_at(temp_path.to_str().unwrap());
    assert!(result.is_ok());

    // Second initialization should fail
    let result = init_git_repo_at(temp_path.to_str().unwrap());
    assert!(result.is_err(), "Should fail when .git already exists");
    assert!(
        result
            .unwrap_err()
            .contains(".git directory already exists"),
        "Error message should indicate .git already exists"
    );
}

#[test]
fn test_create_dir_success() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let test_dir = temp_dir.path().join("test_dir");

    let result = create_dir(test_dir.to_str().unwrap());
    assert!(result.is_ok(), "Should successfully create directory");
    assert!(test_dir.exists(), "Directory should exist after creation");
}

#[test]
fn test_create_dir_fails_on_existing() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let test_dir = temp_dir.path().join("test_dir");

    // Create directory first time
    fs::create_dir(&test_dir).expect("Failed to create test directory");

    // Try to create again - should fail
    let result = create_dir(test_dir.to_str().unwrap());
    assert!(result.is_err(), "Should fail when directory already exists");
}

#[test]
fn test_cli_init_command() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Run the init command in the temporary directory
    let mut cmd = Command::cargo_bin("code_crafters_git").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized git repository"));

    // Verify the .git directory was created
    assert!(temp_dir.path().join(".git").exists());
}

#[test]
fn test_cli_init_fails_when_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create .git directory first
    fs::create_dir(temp_dir.path().join(".git")).unwrap();

    // Run the init command - should fail
    let mut cmd = Command::cargo_bin("code_crafters_git").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains(".git directory already exists"));
}

#[test]
fn test_cli_no_command() {
    Command::cargo_bin("code_crafters_git")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("No command provided"));
}

#[test]
fn test_cli_unknown_command() {
    Command::cargo_bin("code_crafters_git")
        .unwrap()
        .arg("unknown")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown command"));
}

#[test]
fn test_cli_cat_file_no_args() {
    Command::cargo_bin("code_crafters_git")
        .unwrap()
        .arg("cat-file")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No object hash provided"));
}

#[test]
fn test_cli_add_no_args() {
    Command::cargo_bin("code_crafters_git")
        .unwrap()
        .arg("add")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No file provided"));
}

#[test]
fn test_multiple_repos_in_subdirs() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create subdirectories for different repos
    let repo1 = temp_dir.path().join("repo1");
    let repo2 = temp_dir.path().join("repo2");

    fs::create_dir(&repo1).unwrap();
    fs::create_dir(&repo2).unwrap();

    // Initialize git repos in both subdirectories
    let result1 = init_git_repo_at(repo1.to_str().unwrap());
    let result2 = init_git_repo_at(repo2.to_str().unwrap());

    assert!(result1.is_ok(), "Should init repo1");
    assert!(result2.is_ok(), "Should init repo2");

    // Verify both repos were created independently
    assert!(repo1.join(".git").exists());
    assert!(repo2.join(".git").exists());
}

#[test]
fn test_nested_git_repos() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let parent_dir = temp_dir.path();
    let child_dir = parent_dir.join("child");

    fs::create_dir(&child_dir).unwrap();

    // Initialize parent repo
    let parent_result = init_git_repo_at(parent_dir.to_str().unwrap());
    assert!(parent_result.is_ok());

    // Initialize child repo (Git allows nested repos, though it's not common)
    let child_result = init_git_repo_at(child_dir.to_str().unwrap());
    assert!(child_result.is_ok());

    // Verify both exist
    assert!(parent_dir.join(".git").exists());
    assert!(child_dir.join(".git").exists());
}

#[test]
fn test_init_with_invalid_permissions() {
    // This test is platform-specific and may require adjustments
    // Skip on CI or non-Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let restricted_dir = temp_dir.path().join("restricted");

        fs::create_dir(&restricted_dir).unwrap();

        // Remove write permissions
        let mut perms = fs::metadata(&restricted_dir).unwrap().permissions();
        perms.set_mode(0o555); // Read and execute only
        fs::set_permissions(&restricted_dir, perms).unwrap();

        // Try to initialize repo - should fail
        let result = init_git_repo_at(restricted_dir.to_str().unwrap());
        assert!(result.is_err(), "Should fail with no write permissions");

        // Restore permissions for cleanup
        let mut perms = fs::metadata(&restricted_dir).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&restricted_dir, perms).unwrap();
    }
}
