use std::fs;
use std::path::Path;

pub fn create_dir(path: &str) -> Result<(), String> {
    fs::create_dir(path)
        .map_err(|e| format!("Error creating {} directory: {}", path, e))
}

pub fn init_git_repo() -> Result<(), String> {
    init_git_repo_at(".")
}

pub fn init_git_repo_at(path: &str) -> Result<(), String> {
    let base_path = Path::new(path);
    let git_path = base_path.join(".git");
    
    if git_path.exists() {
        return Err("Error: .git directory already exists".to_string());
    }
    
    // Create .git directory structure with proper error handling
    fs::create_dir(&git_path)
        .map_err(|e| format!("Error creating .git directory: {}", e))?;
    fs::create_dir(git_path.join("objects"))
        .map_err(|e| format!("Error creating .git/objects directory: {}", e))?;
    fs::create_dir(git_path.join("refs"))
        .map_err(|e| format!("Error creating .git/refs directory: {}", e))?;
    
    fs::write(git_path.join("HEAD"), "ref: refs/heads/master\n")
        .map_err(|e| format!("Error writing .git/HEAD file: {}", e))?;
    
    println!("Initialized git repository");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_create_dir_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test_dir");
        
        let result = create_dir(test_path.to_str().unwrap());
        
        assert!(result.is_ok());
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_create_dir_fails_if_exists() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test_dir");
        
        // Create directory first
        fs::create_dir(&test_path).unwrap();
        
        // Try to create again
        let result = create_dir(test_path.to_str().unwrap());
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Error creating"));
    }

    #[test]
    fn test_init_git_repo_at_creates_structure() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        let result = init_git_repo_at(temp_path.to_str().unwrap());
        
        assert!(result.is_ok());
        
        // Check directory structure
        let git_dir = temp_path.join(".git");
        assert!(git_dir.exists());
        assert!(git_dir.join("objects").exists());
        assert!(git_dir.join("refs").exists());
        
        // Check HEAD file
        let head_content = fs::read_to_string(git_dir.join("HEAD")).unwrap();
        assert_eq!(head_content, "ref: refs/heads/master\n");
    }

    #[test]
    fn test_init_git_repo_at_fails_if_exists() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Initialize once
        let result1 = init_git_repo_at(temp_path.to_str().unwrap());
        assert!(result1.is_ok());
        
        // Try to initialize again
        let result2 = init_git_repo_at(temp_path.to_str().unwrap());
        assert!(result2.is_err());
        assert_eq!(
            result2.unwrap_err(),
            "Error: .git directory already exists"
        );
    }

    #[test]
    fn test_init_git_repo_uses_current_dir() {
        // This test would need to change the current directory
        // which could affect other tests, so we just verify
        // that init_git_repo calls init_git_repo_at with "."
        // This is more of a smoke test
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let result = init_git_repo();
        
        assert!(result.is_ok());
        assert!(temp_dir.path().join(".git").exists());
    }
}
