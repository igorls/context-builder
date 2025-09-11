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

pub type FileTree = BTreeMap<String, FileNode>;

/// Builds a nested BTreeMap representing the file structure.
pub fn build_file_tree(files: &[DirEntry], base_path: &Path) -> FileTree {
    let mut tree = BTreeMap::new();
    for entry in files {
        let path = entry.path().strip_prefix(base_path).unwrap_or_else(|_| entry.path());
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
pub fn write_tree_to_file(output: &mut impl Write, tree: &FileTree, depth: usize) -> io::Result<()> {
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
        // 1. Setup a temporary directory with a file structure
        let dir = tempdir().unwrap();
        let base_path = dir.path();

        fs::create_dir(base_path.join("src")).unwrap();
        fs::File::create(base_path.join("src/main.rs")).unwrap();
        fs::File::create(base_path.join("README.md")).unwrap();
        // Add a hidden file that should be ignored by default
        fs::File::create(base_path.join(".env")).unwrap();

        // 2. Collect files using the actual function
        let files = collect_files(base_path, &[], &[]).unwrap();

        // 3. Assert that the correct files were collected (hidden file is ignored)
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
}