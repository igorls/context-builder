//! Cache management for context-builder.
//!
//! This module handles caching of project states to enable the auto-diff feature.
//! It uses a hash of the project path and configuration to avoid cache collisions
//! between different projects or configurations.

use fs2::FileExt;

use std::fs;
use std::fs::File;

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::state::ProjectState;

/// Manages cache operations with file locking to prevent corruption
pub struct CacheManager {
    cache_dir: PathBuf,
    project_hash: String,
    config_hash: String,
}

impl CacheManager {
    /// Create a new cache manager for the given project path and configuration
    pub fn new(project_path: &Path, config: &Config) -> Self {
        // Normalize the project path first for consistency
        let normalized_project_path = Self::normalize_project_path(project_path);

        let project_hash = Self::hash_path(&normalized_project_path);
        let config_hash = Self::hash_config(config);

        // Ensure cache directory exists relative to normalized project root
        let cache_dir = normalized_project_path
            .join(".context-builder")
            .join("cache");
        if !cache_dir.exists() {
            let _ = fs::create_dir_all(&cache_dir);
        }

        let cache_manager = Self {
            cache_dir,
            project_hash,
            config_hash,
        };

        // Migrate old cache format if present
        cache_manager.migrate_old_cache();

        cache_manager
    }

    /// Normalize project path for consistent hashing and cache directory creation
    fn normalize_project_path(path: &Path) -> PathBuf {
        // Always resolve to absolute path first
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            match std::env::current_dir() {
                Ok(cwd) => cwd.join(path),
                Err(_) => path.to_path_buf(),
            }
        };

        // Try to canonicalize for consistency, but normalize the result
        if let Ok(canonical) = absolute_path.canonicalize() {
            Self::normalize_path_format(&canonical)
        } else {
            absolute_path
        }
    }

    /// Generate a hash from the normalized project path
    fn hash_path(path: &Path) -> String {
        let path_str = path.to_string_lossy();
        let hash = xxhash_rust::xxh3::xxh3_64(path_str.as_bytes());
        format!("{:x}", hash)
    }

    /// Normalize path format to handle Windows UNC prefixes
    fn normalize_path_format(path: &Path) -> PathBuf {
        let path_str = path.to_string_lossy();

        // Remove Windows UNC prefix if present
        if cfg!(windows) && path_str.starts_with("\\\\?\\") {
            PathBuf::from(&path_str[4..])
        } else {
            path.to_path_buf()
        }
    }

    /// Generate a hash from the configuration
    fn hash_config(config: &Config) -> String {
        // Build a stable string representation of config for hashing.
        // IMPORTANT: Must stay in sync with state.rs::compute_config_hash
        let mut config_str = String::new();
        if let Some(ref filters) = config.filter {
            config_str.push_str(&filters.join(","));
        }
        config_str.push('|');
        if let Some(ref ignores) = config.ignore {
            config_str.push_str(&ignores.join(","));
        }
        config_str.push('|');
        config_str.push_str(&format!(
            "{:?}|{:?}|{:?}",
            config.line_numbers, config.auto_diff, config.diff_context_lines
        ));
        let hash = xxhash_rust::xxh3::xxh3_64(config_str.as_bytes());
        format!("{:x}", hash)
    }

    /// Get the cache file path for this specific project and configuration
    fn get_cache_path(&self) -> PathBuf {
        self.cache_dir.join(format!(
            "state_{}_{}.json",
            self.project_hash, self.config_hash
        ))
    }

    /// Public helper primarily for debugging/tests to inspect the resolved cache path
    pub fn debug_cache_file_path(&self) -> PathBuf {
        self.get_cache_path()
    }

    /// Migrate old markdown-based cache files to new JSON format
    fn migrate_old_cache(&self) {
        let old_cache_patterns = ["last_canonical.md", "last_output.md", "current_output.md"];

        for pattern in &old_cache_patterns {
            let old_cache_path = self.cache_dir.join(pattern);
            if old_cache_path.exists() {
                eprintln!("Migrating old cache format: removing {}", pattern);
                let _ = fs::remove_file(&old_cache_path);
            }
        }

        // Also remove any files that look like timestamped outputs from old versions
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();
                if name.ends_with(".md") && (name.contains("_20") || name.starts_with("output_")) {
                    eprintln!("Migrating old cache format: removing {}", name);
                    let _ = fs::remove_file(entry.path());
                }
            }
        }
    }

    /// Read the cached project state with file locking
    pub fn read_cache(&self) -> Result<Option<ProjectState>, Box<dyn std::error::Error>> {
        let cache_path = self.get_cache_path();

        if !cache_path.exists() {
            return Ok(None);
        }

        let file = File::open(&cache_path)?;
        // Acquire shared lock to prevent reading while writing
        file.lock_shared()?;

        let mut contents = String::new();
        let mut file = std::io::BufReader::new(file);
        file.read_to_string(&mut contents)?;

        // Release lock
        file.get_ref().unlock()?;

        let state: ProjectState = serde_json::from_str(&contents)?;
        Ok(Some(state))
    }

    /// Write the project state to cache with file locking
    pub fn write_cache(&self, state: &ProjectState) -> Result<(), Box<dyn std::error::Error>> {
        let cache_path = self.get_cache_path();

        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&cache_path)?;
        // Acquire exclusive lock BEFORE truncating to prevent TOCTOU races
        file.lock_exclusive()?;
        file.set_len(0)?;

        let json = serde_json::to_string_pretty(state)?;
        let mut file = std::io::BufWriter::new(file);
        file.write_all(json.as_bytes())?;
        file.flush()?;

        // Release lock
        file.get_ref().unlock()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_hash_path() {
        let path1 = Path::new("/project1");
        let path2 = Path::new("/project2");

        let hash1 = CacheManager::hash_path(path1);
        let hash2 = CacheManager::hash_path(path2);

        assert_ne!(
            hash1, hash2,
            "Different paths should produce different hashes"
        );
    }

    #[test]
    fn test_hash_config() {
        let config1 = Config {
            filter: Some(vec!["rs".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            ..Default::default()
        };

        let config2 = Config {
            filter: Some(vec!["md".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            ..Default::default()
        };

        let hash1 = CacheManager::hash_config(&config1);
        let hash2 = CacheManager::hash_config(&config2);

        assert_ne!(
            hash1, hash2,
            "Different configs should produce different hashes"
        );
    }

    #[test]
    fn test_cache_operations() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("test_project");
        let _ = fs::create_dir(&project_path);

        let config = Config::default();
        let cache_manager = CacheManager::new(&project_path, &config);

        use crate::state::ProjectMetadata;

        let state = ProjectState {
            timestamp: "2023-01-01T00:00:00Z".to_string(),
            config_hash: "test_config_hash".to_string(),
            files: std::collections::BTreeMap::new(),
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 0,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        // Write cache
        assert!(cache_manager.write_cache(&state).is_ok());

        // Read cache
        let cached_state = cache_manager.read_cache().unwrap();
        assert!(cached_state.is_some());
        assert_eq!(cached_state.unwrap().timestamp, state.timestamp);
    }

    #[test]
    fn test_old_cache_migration() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("test_project");
        let _ = fs::create_dir(&project_path);

        // Create cache directory with old cache files
        let cache_dir = project_path.join(".context-builder").join("cache");
        let _ = fs::create_dir_all(&cache_dir);

        let old_files = [
            "last_canonical.md",
            "last_output.md",
            "current_output.md",
            "output_20230101120000.md",
        ];

        // Create old cache files
        for file in &old_files {
            let old_path = cache_dir.join(file);
            let _ = fs::write(&old_path, "old cache content");
            assert!(
                old_path.exists(),
                "Old cache file should exist before migration"
            );
        }

        // Create cache manager (this should trigger migration)
        let config = Config::default();
        let _cache_manager = CacheManager::new(&project_path, &config);

        // Verify old files are removed
        for file in &old_files {
            let old_path = cache_dir.join(file);
            assert!(
                !old_path.exists(),
                "Old cache file {} should be removed after migration",
                file
            );
        }
    }

    #[test]
    fn test_cache_consistency_across_path_representations() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("test_project");
        let _ = fs::create_dir(&project_path);

        let config = Config::default();

        // Test different path representations that should resolve to the same cache
        let mut paths_to_test = vec![
            project_path.clone(),
            project_path.canonicalize().unwrap_or(project_path.clone()),
        ];

        // If we can create a relative path, test that too
        if let Ok(current_dir) = std::env::current_dir()
            && let Ok(relative) = project_path.strip_prefix(&current_dir)
        {
            paths_to_test.push(relative.to_path_buf());
        }

        let mut cache_paths = Vec::new();
        for path in &paths_to_test {
            let cache_manager = CacheManager::new(path, &config);
            cache_paths.push(cache_manager.get_cache_path());
        }

        // All cache paths should be identical
        for (i, path1) in cache_paths.iter().enumerate() {
            for (j, path2) in cache_paths.iter().enumerate() {
                if i != j {
                    assert_eq!(
                        path1, path2,
                        "Cache paths should be identical for different representations of the same project path"
                    );
                }
            }
        }
    }

    #[test]
    fn test_normalize_path_format() {
        // Test Windows UNC path normalization
        if cfg!(windows) {
            let unc_path = Path::new("\\\\?\\C:\\test\\path");
            let normalized = CacheManager::normalize_path_format(unc_path);
            assert_eq!(normalized, PathBuf::from("C:\\test\\path"));
        }

        // Test normal path (should remain unchanged)
        let normal_path = Path::new("/normal/path");
        let normalized = CacheManager::normalize_path_format(normal_path);
        assert_eq!(normalized, normal_path);
    }

    #[test]
    fn test_cache_read_nonexistent_file() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("nonexistent_project");

        let config = Config::default();
        let cache_manager = CacheManager::new(&project_path, &config);

        let result = cache_manager.read_cache().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_read_corrupted_file() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("test_project");
        let _ = fs::create_dir(&project_path);

        let config = Config::default();
        let cache_manager = CacheManager::new(&project_path, &config);
        let cache_path = cache_manager.get_cache_path();

        // Create a corrupted cache file
        let _ = fs::create_dir_all(cache_path.parent().unwrap());
        let _ = fs::write(&cache_path, "invalid json content {{{");

        let result = cache_manager.read_cache();
        assert!(result.is_err());
    }

    #[test]
    fn test_cache_write_read_roundtrip() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("test_project");
        let _ = fs::create_dir(&project_path);

        let config = Config {
            filter: Some(vec!["rs".to_string(), "toml".to_string()]),
            ignore: Some(vec!["target".to_string(), ".git".to_string()]),
            line_numbers: Some(true),
            ..Default::default()
        };

        let cache_manager = CacheManager::new(&project_path, &config);

        use crate::state::ProjectMetadata;
        use std::collections::BTreeMap;

        let mut files = BTreeMap::new();
        files.insert(
            PathBuf::from("test.rs"),
            crate::state::FileState {
                content: "fn main() {}".to_string(),
                size: 12,
                modified: std::time::SystemTime::UNIX_EPOCH,
                content_hash: "test_hash".to_string(),
            },
        );

        let original_state = ProjectState {
            timestamp: "2023-01-01T12:00:00Z".to_string(),
            config_hash: "test_config_hash".to_string(),
            files,
            metadata: ProjectMetadata {
                project_name: "test_project".to_string(),
                file_count: 1,
                filters: vec!["rs".to_string(), "toml".to_string()],
                ignores: vec!["target".to_string(), ".git".to_string()],
                line_numbers: true,
            },
        };

        // Write and read back
        cache_manager.write_cache(&original_state).unwrap();
        let cached_state = cache_manager.read_cache().unwrap().unwrap();

        assert_eq!(cached_state.timestamp, original_state.timestamp);
        assert_eq!(cached_state.config_hash, original_state.config_hash);
        assert_eq!(cached_state.files.len(), original_state.files.len());
        assert_eq!(
            cached_state.metadata.project_name,
            original_state.metadata.project_name
        );
        assert_eq!(
            cached_state.metadata.file_count,
            original_state.metadata.file_count
        );
        assert_eq!(
            cached_state.metadata.filters,
            original_state.metadata.filters
        );
        assert_eq!(
            cached_state.metadata.ignores,
            original_state.metadata.ignores
        );
        assert_eq!(
            cached_state.metadata.line_numbers,
            original_state.metadata.line_numbers
        );
    }

    #[test]
    fn test_different_configs_different_cache_files() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("test_project");
        let _ = fs::create_dir(&project_path);

        let config1 = Config {
            filter: Some(vec!["rs".to_string()]),
            ..Default::default()
        };

        let config2 = Config {
            filter: Some(vec!["py".to_string()]),
            ..Default::default()
        };

        let cache_manager1 = CacheManager::new(&project_path, &config1);
        let cache_manager2 = CacheManager::new(&project_path, &config2);

        let cache_path1 = cache_manager1.get_cache_path();
        let cache_path2 = cache_manager2.get_cache_path();

        assert_ne!(
            cache_path1, cache_path2,
            "Different configs should have different cache files"
        );
    }

    #[test]
    fn test_normalize_project_path_absolute() {
        let temp_dir = tempdir().unwrap();
        let project_path = temp_dir.path().join("test_project");
        let _ = fs::create_dir(&project_path);

        let normalized = CacheManager::normalize_project_path(&project_path);
        assert!(normalized.is_absolute());
    }

    #[test]
    fn test_normalize_project_path_relative() {
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to temp directory
        std::env::set_current_dir(&temp_dir).unwrap();

        // Create a project directory
        let project_name = "relative_project";
        let _ = fs::create_dir(project_name);

        let relative_path = Path::new(project_name);
        let normalized = CacheManager::normalize_project_path(relative_path);

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(normalized.is_absolute());
        assert!(normalized.to_string_lossy().contains(project_name));
    }

    #[test]
    fn test_hash_config_same_values() {
        let config1 = Config {
            filter: Some(vec!["rs".to_string(), "toml".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(false),
            ..Default::default()
        };

        let config2 = Config {
            filter: Some(vec!["rs".to_string(), "toml".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(false),
            ..Default::default()
        };

        let hash1 = CacheManager::hash_config(&config1);
        let hash2 = CacheManager::hash_config(&config2);

        assert_eq!(
            hash1, hash2,
            "Identical configs should produce identical hashes"
        );
    }

    #[test]
    fn test_migrate_old_cache_preserves_new_files() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("test_project");
        let _ = fs::create_dir(&project_path);

        let cache_dir = project_path.join(".context-builder").join("cache");
        let _ = fs::create_dir_all(&cache_dir);

        // Create both old and new cache files
        let _ = fs::write(cache_dir.join("last_canonical.md"), "old content");
        let _ = fs::write(cache_dir.join("state_abc123_def456.json"), "new content");

        let config = Config::default();
        let _cache_manager = CacheManager::new(&project_path, &config);

        // Old file should be removed
        assert!(!cache_dir.join("last_canonical.md").exists());

        // New file should be preserved
        assert!(cache_dir.join("state_abc123_def456.json").exists());
    }
}
