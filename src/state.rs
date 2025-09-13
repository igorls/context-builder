//! Project state representation for context-builder.
//!
//! This module provides structured data types to represent the state of a project
//! at a point in time. This replaces the previous approach of caching generated
//! markdown and enables more robust diff generation.

use chrono::Utc;
use ignore::DirEntry;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::config::Config;
use crate::diff::{PerFileDiff, PerFileStatus, diff_file_contents};

/// Complete state representation of a project at a point in time
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectState {
    /// Timestamp when this state was captured
    pub timestamp: String,
    /// Hash of the configuration used to generate this state
    pub config_hash: String,
    /// Map of file paths to their state information
    pub files: BTreeMap<PathBuf, FileState>,
    /// Project metadata
    pub metadata: ProjectMetadata,
}

/// State information for a single file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileState {
    /// Raw file content as string
    pub content: String,
    /// File size in bytes
    pub size: u64,
    /// Last modified time
    pub modified: SystemTime,
    /// Content hash for quick comparison
    pub content_hash: String,
}

/// Metadata about the project
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectMetadata {
    /// Project directory name
    pub project_name: String,
    /// Total number of files processed
    pub file_count: usize,
    /// Filters applied during processing
    pub filters: Vec<String>,
    /// Ignore patterns applied
    pub ignores: Vec<String>,
    /// Whether line numbers were enabled
    pub line_numbers: bool,
}

/// Result of comparing two project states
#[derive(Debug, Clone)]
pub struct StateComparison {
    /// Per-file differences
    pub file_diffs: Vec<PerFileDiff>,
    /// Summary of changes
    pub summary: ChangeSummary,
}

/// Summary of changes between two states
#[derive(Debug, Clone)]
pub struct ChangeSummary {
    /// Files that were added
    pub added: Vec<PathBuf>,
    /// Files that were removed
    pub removed: Vec<PathBuf>,
    /// Files that were modified
    pub modified: Vec<PathBuf>,
    /// Total number of changed files
    pub total_changes: usize,
}

impl ProjectState {
    /// Create a new project state from collected files
    pub fn from_files(
        files: &[DirEntry],
        base_path: &Path,
        config: &Config,
        line_numbers: bool,
    ) -> std::io::Result<Self> {
        let mut file_states = BTreeMap::new();

        for entry in files {
            let relative_path = entry
                .path()
                .strip_prefix(base_path)
                .unwrap_or(entry.path())
                .to_path_buf();

            let file_state = FileState::from_path(entry.path())?;
            file_states.insert(relative_path, file_state);
        }

        let project_name = base_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let metadata = ProjectMetadata {
            project_name,
            file_count: files.len(),
            filters: config.filter.clone().unwrap_or_default(),
            ignores: config.ignore.clone().unwrap_or_default(),
            line_numbers,
        };

        Ok(ProjectState {
            timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            config_hash: Self::compute_config_hash(config),
            files: file_states,
            metadata,
        })
    }

    /// Compare this state with a previous state
    pub fn compare_with(&self, previous: &ProjectState) -> StateComparison {
        // Convert file states to content maps for diff_file_contents
        let previous_content: std::collections::HashMap<String, String> = previous
            .files
            .iter()
            .map(|(path, state)| (path.to_string_lossy().to_string(), state.content.clone()))
            .collect();

        let current_content: std::collections::HashMap<String, String> = self
            .files
            .iter()
            .map(|(path, state)| (path.to_string_lossy().to_string(), state.content.clone()))
            .collect();

        // Generate per-file diffs
        let file_diffs = diff_file_contents(&previous_content, &current_content, true, None);

        // Generate summary
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut modified = Vec::new();

        for diff in &file_diffs {
            let path = PathBuf::from(&diff.path);
            match diff.status {
                PerFileStatus::Added => added.push(path),
                PerFileStatus::Removed => removed.push(path),
                PerFileStatus::Modified => modified.push(path),
                PerFileStatus::Unchanged => {}
            }
        }

        let summary = ChangeSummary {
            total_changes: added.len() + removed.len() + modified.len(),
            added,
            removed,
            modified,
        };

        StateComparison {
            file_diffs,
            summary,
        }
    }

    /// Check if this state has any content changes compared to another
    pub fn has_changes(&self, other: &ProjectState) -> bool {
        if self.files.len() != other.files.len() {
            return true;
        }

        for (path, state) in &self.files {
            match other.files.get(path) {
                Some(other_state) => {
                    if state.content_hash != other_state.content_hash {
                        return true;
                    }
                }
                None => return true,
            }
        }

        false
    }

    /// Generate a configuration hash for cache validation
    fn compute_config_hash(config: &Config) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        config.filter.hash(&mut hasher);
        config.ignore.hash(&mut hasher);
        config.line_numbers.hash(&mut hasher);
        config.auto_diff.hash(&mut hasher);
        config.diff_context_lines.hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }
}

impl FileState {
    /// Create a file state from a file path
    pub fn from_path(path: &Path) -> std::io::Result<Self> {
        use std::collections::hash_map::DefaultHasher;
        use std::fs;
        use std::hash::{Hash, Hasher};
        use std::io::ErrorKind;

        let metadata = fs::metadata(path)?;

        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) if e.kind() == ErrorKind::InvalidData => {
                // Handle binary files gracefully
                log::warn!("Skipping binary file in auto-diff mode: {}", path.display());
                format!("<Binary file - {} bytes>", metadata.len())
            }
            Err(e) => return Err(e),
        };

        // Compute content hash
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let content_hash = format!("{:x}", hasher.finish());

        Ok(FileState {
            content,
            size: metadata.len(),
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            content_hash,
        })
    }
}

impl ChangeSummary {
    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        self.total_changes > 0
    }

    /// Generate markdown representation of the change summary
    pub fn to_markdown(&self) -> String {
        if !self.has_changes() {
            return String::new();
        }

        let mut output = String::new();
        output.push_str("## Change Summary\n\n");

        for path in &self.added {
            output.push_str(&format!("- Added: `{}`\n", path.display()));
        }

        for path in &self.removed {
            output.push_str(&format!("- Removed: `{}`\n", path.display()));
        }

        for path in &self.modified {
            output.push_str(&format!("- Modified: `{}`\n", path.display()));
        }

        output.push('\n');
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_file_state_creation() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello, world!").unwrap();

        let file_state = FileState::from_path(&file_path).unwrap();

        assert_eq!(file_state.content, "Hello, world!");
        assert_eq!(file_state.size, 13);
        assert!(!file_state.content_hash.is_empty());
    }

    #[test]
    fn test_project_state_comparison() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        // Create initial files
        fs::write(base_path.join("file1.txt"), "content1").unwrap();
        fs::write(base_path.join("file2.txt"), "content2").unwrap();

        let mut state1_files = BTreeMap::new();
        state1_files.insert(
            PathBuf::from("file1.txt"),
            FileState::from_path(&base_path.join("file1.txt")).unwrap(),
        );
        state1_files.insert(
            PathBuf::from("file2.txt"),
            FileState::from_path(&base_path.join("file2.txt")).unwrap(),
        );

        let state1 = ProjectState {
            timestamp: "2023-01-01T00:00:00Z".to_string(),
            config_hash: "test_hash".to_string(),
            files: state1_files,
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 2,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        // Modify and create new state
        fs::write(base_path.join("file1.txt"), "modified_content1").unwrap();
        fs::write(base_path.join("file3.txt"), "content3").unwrap();

        let mut state2_files = BTreeMap::new();
        state2_files.insert(
            PathBuf::from("file1.txt"),
            FileState::from_path(&base_path.join("file1.txt")).unwrap(),
        );
        state2_files.insert(
            PathBuf::from("file2.txt"),
            FileState::from_path(&base_path.join("file2.txt")).unwrap(),
        );
        state2_files.insert(
            PathBuf::from("file3.txt"),
            FileState::from_path(&base_path.join("file3.txt")).unwrap(),
        );

        let state2 = ProjectState {
            timestamp: "2023-01-01T01:00:00Z".to_string(),
            config_hash: "test_hash".to_string(),
            files: state2_files,
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 3,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        let comparison = state2.compare_with(&state1);

        assert_eq!(comparison.summary.added.len(), 1);
        assert_eq!(comparison.summary.modified.len(), 1);
        assert_eq!(comparison.summary.removed.len(), 0);
        assert!(
            comparison
                .summary
                .added
                .contains(&PathBuf::from("file3.txt"))
        );
        assert!(
            comparison
                .summary
                .modified
                .contains(&PathBuf::from("file1.txt"))
        );
    }

    #[test]
    fn test_change_summary_markdown() {
        let summary = ChangeSummary {
            added: vec![PathBuf::from("new.txt")],
            removed: vec![PathBuf::from("old.txt")],
            modified: vec![PathBuf::from("changed.txt")],
            total_changes: 3,
        };

        let markdown = summary.to_markdown();

        assert!(markdown.contains("## Change Summary"));
        assert!(markdown.contains("- Added: `new.txt`"));
        assert!(markdown.contains("- Removed: `old.txt`"));
        assert!(markdown.contains("- Modified: `changed.txt`"));
    }

    #[test]
    fn test_binary_file_handling() {
        let temp_dir = tempdir().unwrap();
        let binary_file = temp_dir.path().join("test.bin");

        // Write binary data (non-UTF8)
        let binary_data = vec![0u8, 255, 128, 42, 0, 1, 2, 3];
        fs::write(&binary_file, &binary_data).unwrap();

        // Should not crash and should handle gracefully
        let file_state = FileState::from_path(&binary_file).unwrap();

        // Content should be a placeholder for binary files
        assert!(file_state.content.contains("Binary file"));
        assert!(file_state.content.contains("8 bytes"));
        assert_eq!(file_state.size, 8);
        assert!(!file_state.content_hash.is_empty());
    }

    #[test]
    fn test_has_changes_identical_states() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        fs::write(base_path.join("test.txt"), "content").unwrap();

        let mut files = BTreeMap::new();
        files.insert(
            PathBuf::from("test.txt"),
            FileState::from_path(&base_path.join("test.txt")).unwrap(),
        );

        let state1 = ProjectState {
            timestamp: "2023-01-01T00:00:00Z".to_string(),
            config_hash: "hash1".to_string(),
            files: files.clone(),
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 1,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        let state2 = ProjectState {
            timestamp: "2023-01-01T01:00:00Z".to_string(),
            config_hash: "hash1".to_string(),
            files,
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 1,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        assert!(!state1.has_changes(&state2));
    }

    #[test]
    fn test_has_changes_different_file_count() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        fs::write(base_path.join("test1.txt"), "content1").unwrap();
        fs::write(base_path.join("test2.txt"), "content2").unwrap();

        let mut files1 = BTreeMap::new();
        files1.insert(
            PathBuf::from("test1.txt"),
            FileState::from_path(&base_path.join("test1.txt")).unwrap(),
        );

        let mut files2 = BTreeMap::new();
        files2.insert(
            PathBuf::from("test1.txt"),
            FileState::from_path(&base_path.join("test1.txt")).unwrap(),
        );
        files2.insert(
            PathBuf::from("test2.txt"),
            FileState::from_path(&base_path.join("test2.txt")).unwrap(),
        );

        let state1 = ProjectState {
            timestamp: "2023-01-01T00:00:00Z".to_string(),
            config_hash: "hash1".to_string(),
            files: files1,
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 1,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        let state2 = ProjectState {
            timestamp: "2023-01-01T01:00:00Z".to_string(),
            config_hash: "hash1".to_string(),
            files: files2,
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 2,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        assert!(state1.has_changes(&state2));
    }

    #[test]
    fn test_has_changes_content_different() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();
        fs::write(base_path.join("test.txt"), "content1").unwrap();

        let file_state1 = FileState::from_path(&base_path.join("test.txt")).unwrap();

        fs::write(base_path.join("test.txt"), "content2").unwrap();
        let file_state2 = FileState::from_path(&base_path.join("test.txt")).unwrap();

        let mut files1 = BTreeMap::new();
        files1.insert(PathBuf::from("test.txt"), file_state1);

        let mut files2 = BTreeMap::new();
        files2.insert(PathBuf::from("test.txt"), file_state2);

        let state1 = ProjectState {
            timestamp: "2023-01-01T00:00:00Z".to_string(),
            config_hash: "hash1".to_string(),
            files: files1,
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 1,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        let state2 = ProjectState {
            timestamp: "2023-01-01T01:00:00Z".to_string(),
            config_hash: "hash1".to_string(),
            files: files2,
            metadata: ProjectMetadata {
                project_name: "test".to_string(),
                file_count: 1,
                filters: vec![],
                ignores: vec![],
                line_numbers: false,
            },
        };

        assert!(state1.has_changes(&state2));
    }

    #[test]
    fn test_config_hash_generation() {
        let config1 = Config {
            filter: Some(vec!["rs".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            auto_diff: Some(false),
            diff_context_lines: Some(3),
            ..Default::default()
        };

        let config2 = Config {
            filter: Some(vec!["rs".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            auto_diff: Some(false),
            diff_context_lines: Some(3),
            ..Default::default()
        };

        let config3 = Config {
            filter: Some(vec!["py".to_string()]), // Different filter
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            auto_diff: Some(false),
            diff_context_lines: Some(3),
            ..Default::default()
        };

        let hash1 = ProjectState::compute_config_hash(&config1);
        let hash2 = ProjectState::compute_config_hash(&config2);
        let hash3 = ProjectState::compute_config_hash(&config3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_change_summary_no_changes() {
        let summary = ChangeSummary {
            added: vec![],
            removed: vec![],
            modified: vec![],
            total_changes: 0,
        };

        assert!(!summary.has_changes());
        assert_eq!(summary.to_markdown(), "");
    }

    #[test]
    fn test_from_files_with_config() {
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path();

        fs::write(base_path.join("test.rs"), "fn main() {}").unwrap();
        fs::write(base_path.join("README.md"), "# Test").unwrap();

        let entries = vec![
            create_mock_dir_entry(&base_path.join("test.rs")),
            create_mock_dir_entry(&base_path.join("README.md")),
        ];

        let config = Config {
            filter: Some(vec!["rs".to_string()]),
            ignore: Some(vec!["target".to_string()]),
            line_numbers: Some(true),
            ..Default::default()
        };

        let state = ProjectState::from_files(&entries, base_path, &config, true).unwrap();

        assert_eq!(state.files.len(), 2);
        assert_eq!(state.metadata.file_count, 2);
        assert_eq!(state.metadata.filters, vec!["rs"]);
        assert_eq!(state.metadata.ignores, vec!["target"]);
        assert!(state.metadata.line_numbers);
        assert!(!state.timestamp.is_empty());
        assert!(!state.config_hash.is_empty());
    }

    // Helper function to create a mock DirEntry for testing
    fn create_mock_dir_entry(path: &std::path::Path) -> ignore::DirEntry {
        // This is a bit of a hack since DirEntry doesn't have a public constructor
        // We use the ignore crate's WalkBuilder to create a real DirEntry
        let walker = ignore::WalkBuilder::new(path.parent().unwrap());
        walker
            .build()
            .filter_map(Result::ok)
            .find(|entry| entry.path() == path)
            .expect("Failed to create DirEntry for test")
    }
}
