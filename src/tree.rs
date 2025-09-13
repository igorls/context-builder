use ignore::DirEntry;
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::Path;

/// A nested map to represent the file tree structure.
#[derive(Debug, Clone, PartialEq)]
pub enum FileNode {
    File,
    Directory(BTreeMap<String, FileNode>),
}

/// Type alias for the file tree structure.
pub type FileTree = BTreeMap<String, FileNode>;

/// Builds a nested BTreeMap representing the file structure.
pub fn build_file_tree(files: &[DirEntry], base_path: &Path) -> FileTree {
    let mut tree = BTreeMap::new();
    for entry in files {
        let path = entry
            .path()
            .strip_prefix(base_path)
            .unwrap_or_else(|_| entry.path());
        let components: Vec<_> = path.components().collect();

        // Insert this path into the tree
        insert_path(&mut tree, &components);
    }
    tree
}

/// Helper function to insert a path into the tree structure
fn insert_path(tree: &mut FileTree, components: &[std::path::Component]) {
    if components.is_empty() {
        return;
    }

    let name = components[0].as_os_str().to_string_lossy().to_string();

    if components.len() == 1 {
        // This is the last component, so it's a file
        tree.insert(name, FileNode::File);
    } else {
        // This is a directory component
        // Make sure the directory exists
        tree.entry(name.clone())
            .or_insert_with(|| FileNode::Directory(BTreeMap::new()));

        // Recursively insert the rest of the path
        if let Some(FileNode::Directory(next_dir)) = tree.get_mut(&name) {
            insert_path(next_dir, &components[1..]);
        }
    }
}

/// Recursively prints the file tree to the console.
pub fn print_tree(tree: &FileTree, depth: usize) {
    for (name, node) in tree {
        let indent = "  ".repeat(depth);
        match node {
            FileNode::File => {
                println!("{}- 📄 {}", indent, name);
            }
            FileNode::Directory(children) => {
                println!("{}- 📁 {}", indent, name);
                print_tree(children, depth + 1);
            }
        }
    }
}

/// Recursively writes the file tree to a file.
pub fn write_tree_to_file(
    output: &mut impl Write,
    tree: &FileTree,
    depth: usize,
) -> io::Result<()> {
    for (name, node) in tree {
        let indent = "  ".repeat(depth);
        match node {
            FileNode::File => {
                writeln!(output, "{}- 📄 {}", indent, name)?;
            }
            FileNode::Directory(children) => {
                writeln!(output, "{}- 📁 {}", indent, name)?;
                write_tree_to_file(output, children, depth + 1)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utils::collect_files;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_build_file_tree_with_collected_files() {
        // 1. Set up a temporary directory with a file structure
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        fs::create_dir(base_path.join("src")).unwrap();
        fs::File::create(base_path.join("src/main.rs")).unwrap();
        fs::File::create(base_path.join("README.md")).unwrap();
        // Add a hidden file that should be ignored by default
        fs::File::create(base_path.join(".env")).unwrap();

        // 2. Collect files using the actual function
        let files = collect_files(base_path, &[], &[]).unwrap();

        // 3. Assert that the correct files were collected (a hidden file is ignored)
        assert_eq!(files.len(), 2);

        // 4. Build the tree with the collected files
        let tree = build_file_tree(&files, base_path);

        // 5. Assert the tree structure is correct
        let mut expected: FileTree = BTreeMap::new();
        let mut src_tree = BTreeMap::new();
        src_tree.insert("main.rs".to_string(), FileNode::File);
        expected.insert("src".to_string(), FileNode::Directory(src_tree));
        expected.insert("README.md".to_string(), FileNode::File);

        assert_eq!(tree, expected);
    }

    #[test]
    fn test_build_file_tree_empty() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        let files = collect_files(base_path, &[], &[]).unwrap();
        let tree = build_file_tree(&files, base_path);

        assert!(tree.is_empty());
    }

    #[test]
    fn test_build_file_tree_single_file() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        fs::File::create(base_path.join("single.txt")).unwrap();

        let files = collect_files(base_path, &[], &[]).unwrap();
        let tree = build_file_tree(&files, base_path);

        let mut expected: FileTree = BTreeMap::new();
        expected.insert("single.txt".to_string(), FileNode::File);

        assert_eq!(tree, expected);
    }

    #[test]
    fn test_build_file_tree_nested_directories() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        fs::create_dir_all(base_path.join("a/b/c")).unwrap();
        fs::File::create(base_path.join("a/b/c/deep.txt")).unwrap();
        fs::File::create(base_path.join("a/shallow.txt")).unwrap();

        let files = collect_files(base_path, &[], &[]).unwrap();
        let tree = build_file_tree(&files, base_path);

        // Build expected structure
        let mut c_tree = BTreeMap::new();
        c_tree.insert("deep.txt".to_string(), FileNode::File);

        let mut b_tree = BTreeMap::new();
        b_tree.insert("c".to_string(), FileNode::Directory(c_tree));

        let mut a_tree = BTreeMap::new();
        a_tree.insert("b".to_string(), FileNode::Directory(b_tree));
        a_tree.insert("shallow.txt".to_string(), FileNode::File);

        let mut expected: FileTree = BTreeMap::new();
        expected.insert("a".to_string(), FileNode::Directory(a_tree));

        assert_eq!(tree, expected);
    }

    #[test]
    fn test_build_file_tree_unicode_filenames() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        fs::create_dir(base_path.join("测试目录")).unwrap();
        fs::File::create(base_path.join("测试目录/文件.txt")).unwrap();
        fs::File::create(base_path.join("🦀.rs")).unwrap();

        let files = collect_files(base_path, &[], &[]).unwrap();
        let tree = build_file_tree(&files, base_path);

        let mut test_dir = BTreeMap::new();
        test_dir.insert("文件.txt".to_string(), FileNode::File);

        let mut expected: FileTree = BTreeMap::new();
        expected.insert("测试目录".to_string(), FileNode::Directory(test_dir));
        expected.insert("🦀.rs".to_string(), FileNode::File);

        assert_eq!(tree, expected);
    }

    #[test]
    fn test_insert_path_empty_components() {
        let mut tree = BTreeMap::new();
        insert_path(&mut tree, &[]);
        assert!(tree.is_empty());
    }

    #[test]
    fn test_write_tree_to_file() {
        let mut tree = BTreeMap::new();
        tree.insert("file1.txt".to_string(), FileNode::File);

        let mut subdir = BTreeMap::new();
        subdir.insert("file2.md".to_string(), FileNode::File);
        tree.insert("src".to_string(), FileNode::Directory(subdir));

        let mut output = Vec::new();
        write_tree_to_file(&mut output, &tree, 0).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("- 📄 file1.txt"));
        assert!(result.contains("- 📁 src"));
        assert!(result.contains("  - 📄 file2.md"));
    }

    #[test]
    fn test_write_tree_to_file_with_depth() {
        let mut tree = BTreeMap::new();
        tree.insert("nested.txt".to_string(), FileNode::File);

        let mut output = Vec::new();
        write_tree_to_file(&mut output, &tree, 2).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("    - 📄 nested.txt")); // 2 levels of indentation
    }

    #[test]
    fn test_write_tree_to_file_empty_tree() {
        let tree = BTreeMap::new();
        let mut output = Vec::new();
        write_tree_to_file(&mut output, &tree, 0).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_file_node_equality() {
        let file1 = FileNode::File;
        let file2 = FileNode::File;
        assert_eq!(file1, file2);

        let mut dir1 = BTreeMap::new();
        dir1.insert("test.txt".to_string(), FileNode::File);
        let node1 = FileNode::Directory(dir1.clone());
        let node2 = FileNode::Directory(dir1);
        assert_eq!(node1, node2);

        // Different directories should not be equal
        let mut dir2 = BTreeMap::new();
        dir2.insert("other.txt".to_string(), FileNode::File);
        let node3 = FileNode::Directory(dir2);
        assert_ne!(node1, node3);

        // File and directory should not be equal
        assert_ne!(file1, node1);
    }

    #[test]
    fn test_build_file_tree_multiple_files_same_directory() {
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        fs::create_dir(base_path.join("docs")).unwrap();
        fs::File::create(base_path.join("docs/readme.md")).unwrap();
        fs::File::create(base_path.join("docs/guide.md")).unwrap();
        fs::File::create(base_path.join("docs/api.md")).unwrap();

        let files = collect_files(base_path, &[], &[]).unwrap();
        let tree = build_file_tree(&files, base_path);

        let mut docs_tree = BTreeMap::new();
        docs_tree.insert("api.md".to_string(), FileNode::File);
        docs_tree.insert("guide.md".to_string(), FileNode::File);
        docs_tree.insert("readme.md".to_string(), FileNode::File);

        let mut expected: FileTree = BTreeMap::new();
        expected.insert("docs".to_string(), FileNode::Directory(docs_tree));

        assert_eq!(tree, expected);
    }
}
