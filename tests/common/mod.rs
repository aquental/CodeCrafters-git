use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Test fixture for setting up a test repository
pub struct TestRepo {
    pub temp_dir: TempDir,
    pub repo_path: PathBuf,
}

impl TestRepo {
    /// Create a new test repository
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo_path = temp_dir.path().to_path_buf();
        
        TestRepo {
            temp_dir,
            repo_path,
        }
    }
    
    /// Create a test repository with a subdirectory
    pub fn with_subdir(subdir: &str) -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo_path = temp_dir.path().join(subdir);
        fs::create_dir(&repo_path).expect("Failed to create subdirectory");
        
        TestRepo {
            temp_dir,
            repo_path,
        }
    }
    
    /// Get the path to the repository
    pub fn path(&self) -> &Path {
        &self.repo_path
    }
    
    /// Get the path to the .git directory
    pub fn git_dir(&self) -> PathBuf {
        self.repo_path.join(".git")
    }
    
    /// Create a test file in the repository
    pub fn create_file(&self, name: &str, content: &str) -> PathBuf {
        let file_path = self.repo_path.join(name);
        fs::write(&file_path, content).expect("Failed to write test file");
        file_path
    }
    
    /// Create a directory in the repository
    pub fn create_dir(&self, name: &str) -> PathBuf {
        let dir_path = self.repo_path.join(name);
        fs::create_dir(&dir_path).expect("Failed to create test directory");
        dir_path
    }
    
    /// Create nested directories
    pub fn create_nested_dirs(&self, path: &str) -> PathBuf {
        let dir_path = self.repo_path.join(path);
        fs::create_dir_all(&dir_path).expect("Failed to create nested directories");
        dir_path
    }
    
    /// Check if a path exists relative to the repo
    pub fn exists(&self, path: &str) -> bool {
        self.repo_path.join(path).exists()
    }
    
    /// Read file content relative to the repo
    pub fn read_file(&self, path: &str) -> String {
        fs::read_to_string(self.repo_path.join(path))
            .expect("Failed to read file")
    }
}

/// Helper to create multiple test repos for parallel testing
pub struct TestRepoGroup {
    repos: Vec<TestRepo>,
}

impl TestRepoGroup {
    /// Create a group of test repositories
    pub fn new(count: usize) -> Self {
        let repos = (0..count)
            .map(|_| TestRepo::new())
            .collect();
        
        TestRepoGroup { repos }
    }
    
    /// Get a reference to all repos
    pub fn repos(&self) -> &[TestRepo] {
        &self.repos
    }
    
    /// Get a mutable reference to all repos
    pub fn repos_mut(&mut self) -> &mut [TestRepo] {
        &mut self.repos
    }
}

/// Test data fixtures
pub mod fixtures {
    /// Sample blob content for testing
    pub const SAMPLE_BLOB: &str = "Hello, Git!\n";
    
    /// Sample commit message
    pub const SAMPLE_COMMIT_MSG: &str = "Initial commit";
    
    /// Sample file names for testing
    pub const TEST_FILES: &[&str] = &["file1.txt", "file2.txt", "README.md"];
    
    /// Sample file contents
    pub fn sample_file_content(index: usize) -> String {
        format!("This is test file {}\nLine 2\nLine 3\n", index)
    }
}

/// Assertions for git-specific checks
pub mod assertions {
    use std::path::Path;
    
    /// Assert that a git repository structure exists
    pub fn assert_git_repo_exists(repo_path: &Path) {
        let git_dir = repo_path.join(".git");
        assert!(git_dir.exists(), ".git directory should exist");
        assert!(git_dir.join("objects").exists(), ".git/objects should exist");
        assert!(git_dir.join("refs").exists(), ".git/refs should exist");
        assert!(git_dir.join("HEAD").exists(), ".git/HEAD should exist");
    }
    
    /// Assert that HEAD points to the expected reference
    pub fn assert_head_ref(repo_path: &Path, expected_ref: &str) {
        let head_content = std::fs::read_to_string(repo_path.join(".git/HEAD"))
            .expect("Failed to read HEAD");
        assert_eq!(head_content.trim(), expected_ref);
    }
}

/// Cleanup utilities
pub mod cleanup {
    use std::fs;
    use std::path::Path;
    
    /// Safely remove a directory if it exists
    pub fn remove_if_exists(path: &Path) {
        if path.exists() {
            fs::remove_dir_all(path).ok();
        }
    }
    
    /// Clean up multiple paths
    pub fn cleanup_paths(paths: &[&Path]) {
        for path in paths {
            remove_if_exists(path);
        }
    }
}