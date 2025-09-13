//! Cache management for context-builder.
//!
//! This module handles caching of project states to enable the auto-diff feature.
//! It uses a hash of the project path and configuration to avoid cache collisions
//! between different projects or configurations.

use fs2::FileExt;

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
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
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        format!("{:x}", hasher.finish())
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
        let mut hasher = DefaultHasher::new();
        // Hash the relevant configuration parameters that affect output
        config.filter.hash(&mut hasher);
        config.ignore.hash(&mut hasher);
        config.line_numbers.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Get the cache file path for this specific project and configuration
    fn get_cache_path(&self) -> PathBuf {
        self.cache_dir.join(format!(
            "state_{}_{}.json",
            self.project_hash, self.config_hash
        ))
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

        let file = File::create(&cache_path)?;
        // Acquire exclusive lock to prevent concurrent writes
        file.lock_exclusive()?;

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
}
