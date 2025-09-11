use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io::{self, stdin, stdout, Write};
use std::path::Path;
use std::time::Instant;
use chrono::Utc;

/// CLI tool to aggregate directory contents into a single markdown file optimized for LLM consumption
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Directory path to process
    #[clap(short = 'd', long)]
    input: String,

    /// Output file path
    #[clap(short, long, default_value = "output.md")]
    output: String,

    /// Comma-separated list of file extensions to include (e.g., "rs,toml")
    #[clap(short = 'f', long, default_value = "")]
    filter: String,

    /// Comma-separated list of folder names to ignore (e.g., "target,node_modules")
    #[clap(short = 'i', long, default_value = "")]
    ignore: String,

    /// Preview mode: only print the file tree to console, don't generate the documentation file
    #[clap(long)]
    preview: bool,

    /// Add line numbers to code blocks in the output
    #[clap(long)]
    line_numbers: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    let base_path = Path::new(&args.input);
    
    // Collect ignore list
    let ignore_list: HashSet<String> = if !args.ignore.is_empty() {
        args.ignore.split(',').map(|s| s.to_string()).collect()
    } else {
        HashSet::new()
    };
    
    // Collect filter list
    let filter_list: Vec<&str> = if !args.filter.is_empty() {
        args.filter.split(',').collect()
    } else {
        vec![]
    };
    
    // Count files first to check if we exceed the limit
    println!("Counting files...");
    let file_count = count_files(base_path, base_path, &filter_list, &ignore_list)?;
    println!("Found {} files.", file_count);
    
    // If preview mode, just print the tree and exit
    if args.preview {
        println!("\n# File Tree Structure (Preview)\n");
        print_file_tree(base_path, base_path, &filter_list, &ignore_list, 0)?;
        return Ok(());
    }
    
    // Warn if too many files (but not in preview mode)
    if file_count > 100 {
        println!("Warning: You're about to process {} files. This might take a while.", file_count);
        println!("Do you want to continue? [y/N]");
        stdout().flush()?;
        
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Operation cancelled.");
            return Ok(());
        }
    }
    
    // Start timer for benchmarking
    let start_time = Instant::now();
    
    let mut output = fs::File::create(&args.output)?;
    
    writeln!(output, "# Project Documentation\n")?;
    if !args.filter.is_empty() {
        writeln!(output, "This document contains files from the `{}` directory with extensions: {}", args.input, args.filter)?;
    } else {
        writeln!(output, "This document contains all files from the `{}` directory, optimized for LLM consumption.", args.input)?;
    }
    
    if !args.ignore.is_empty() {
        writeln!(output, "Ignored folders: {}", args.ignore)?;
    }
    
    writeln!(output, "Processed at: {}", Utc::now().format("%Y-%m-%d %H:%M:%S UTC"))?;
    writeln!(output)?;
    
    // Generate and write file tree
    writeln!(output, "# File Tree Structure\n")?;
    write_file_tree(base_path, base_path, &mut output, &filter_list, &ignore_list, 0)?;
    writeln!(output)?;
    
    // Process files
    process_directory(base_path, base_path, &mut output, &filter_list, &ignore_list, args.line_numbers)?;
    
    let duration = start_time.elapsed();
    writeln!(output, "\n\n---\n*Documentation generated in {:.2?}*", duration)?;
    
    println!("Documentation created successfully: {}", args.output);
    println!("Processing time: {:.2?}", duration);
    Ok(())
}

fn count_files(base_path: &Path, current_path: &Path, filter_list: &[&str], ignore_list: &HashSet<String>) -> io::Result<usize> {
    let mut count = 0;
    
    if current_path.is_dir() {
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            // Skip ignored directories
            if path.is_dir() && ignore_list.contains(&file_name) {
                continue;
            }
            
            if path.is_dir() {
                count += count_files(base_path, &path, filter_list, ignore_list)?;
            } else {
                // If filter list is empty, include all files, otherwise check if extension matches
                if filter_list.is_empty() || should_include_file(&path, filter_list) {
                    count += 1;
                }
            }
        }
    }
    
    Ok(count)
}

fn write_file_tree(
    base_path: &Path,
    current_path: &Path,
    output: &mut fs::File,
    filter_list: &[&str],
    ignore_list: &HashSet<String>,
    depth: usize,
) -> io::Result<()> {
    if current_path.is_dir() {
        let indent = "  ".repeat(depth);
        
        // Skip the base path itself in the tree display
        if current_path != base_path {
            let relative_path = current_path.strip_prefix(base_path).unwrap_or(current_path);
            writeln!(output, "{}- 📁 {}", indent, relative_path.display())?;
        }
        
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            // Skip ignored directories and files
            if ignore_list.contains(&file_name) {
                continue;
            }
            
            if path.is_dir() {
                // Check if directory contains any files that will be included
                if has_included_files(&path, filter_list, ignore_list)? {
                    write_file_tree(base_path, &path, output, filter_list, ignore_list, depth + 1)?;
                }
            } else {
                // Check if this file will be included
                if filter_list.is_empty() || should_include_file(&path, filter_list) {
                    let relative_path = path.strip_prefix(base_path).unwrap_or(&path);
                    writeln!(output, "{}  - 📄 {}", indent, relative_path.display())?;
                }
            }
        }
    }
    
    Ok(())
}

fn has_included_files(path: &Path, filter_list: &[&str], ignore_list: &HashSet<String>) -> io::Result<bool> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            // Skip ignored directories
            if entry_path.is_dir() && ignore_list.contains(&file_name) {
                continue;
            }
            
            if entry_path.is_dir() {
                if has_included_files(&entry_path, filter_list, ignore_list)? {
                    return Ok(true);
                }
            } else {
                if filter_list.is_empty() || should_include_file(&entry_path, filter_list) {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

fn process_directory(
    base_path: &Path,
    current_path: &Path,
    output: &mut fs::File,
    filter_list: &[&str],
    ignore_list: &HashSet<String>,
    line_numbers: bool,
) -> io::Result<()> {
    if current_path.is_dir() {
        // List all entries in the directory
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            // Skip ignored directories
            if path.is_dir() && ignore_list.contains(&file_name) {
                continue;
            }
            
            if path.is_dir() {
                process_directory(base_path, &path, output, filter_list, ignore_list, line_numbers)?;
            } else {
                // If filter list is empty, include all files, otherwise check if extension matches
                if filter_list.is_empty() || should_include_file(&path, filter_list) {
                    process_file(base_path, &path, output, line_numbers)?;
                }
            }
        }
    }
    
    Ok(())
}

fn should_include_file(file_path: &Path, filter_list: &[&str]) -> bool {
    if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
        filter_list.iter().any(|&filter_ext| filter_ext.eq_ignore_ascii_case(extension))
    } else {
        false
    }
}

fn process_file(base_path: &Path, file_path: &Path, output: &mut fs::File, line_numbers: bool) -> io::Result<()> {
    // Get relative path from base
    let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);
    
    // Get file metadata
    let metadata = fs::metadata(file_path)?;
    let modified_time = metadata.modified()
        .ok()
        .and_then(|time| {
            let system_time: chrono::DateTime<Utc> = time.into();
            Some(system_time.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        })
        .unwrap_or_else(|| "Unknown".to_string());
    
    // Write file header
    writeln!(output, "## File: `{}`", relative_path.display())?;
    writeln!(output, "- Size: {} bytes", metadata.len())?;
    writeln!(output, "- Modified: {}", modified_time)?;
    writeln!(output)?;
    
    // Try to determine file extension for syntax highlighting
    let extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("text");
    
    // Special cases for common extensions
    let language = match extension {
        "rs" => "rust",
        "js" => "javascript",
        "ts" => "typescript",
        "jsx" => "jsx",
        "tsx" => "tsx",
        "json" => "json",
        "toml" => "toml",
        "md" => "markdown",
        "yaml" | "yml" => "yaml",
        "html" => "html",
        "css" => "css",
        "py" => "python",
        "java" => "java",
        "cpp" => "cpp",
        "c" => "c",
        "h" => "c",
        "hpp" => "cpp",
        "sql" => "sql",
        "sh" => "bash",
        "xml" => "xml",
        _ => extension,
    };
    
    // Write file content
    match fs::read_to_string(file_path) {
        Ok(content) => {
            if line_numbers {
                writeln!(output, "```{}", language)?;
                for (index, line) in content.lines().enumerate() {
                    writeln!(output, "{:>4} | {}", index + 1, line)?;
                }
                writeln!(output, "```")?;
            } else {
                writeln!(output, "```{}", language)?;
                writeln!(output, "{}", content)?;
                writeln!(output, "```")?;
            }
        }
        Err(_) => {
            writeln!(output, "```text")?;
            writeln!(output, "<Binary file not displayed>")?;
            writeln!(output, "```")?;
        }
    }
    
    writeln!(output)?;
    Ok(())
}

fn print_file_tree(
    base_path: &Path,
    current_path: &Path,
    filter_list: &[&str],
    ignore_list: &HashSet<String>,
    depth: usize,
) -> io::Result<()> {
    if current_path.is_dir() {
        let indent = "  ".repeat(depth);
        
        // Skip the base path itself in the tree display
        if current_path != base_path {
            let relative_path = current_path.strip_prefix(base_path).unwrap_or(current_path);
            println!("{}- 📁 {}", indent, relative_path.display());
        }
        
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            // Skip ignored directories and files
            if ignore_list.contains(&file_name) {
                continue;
            }
            
            if path.is_dir() {
                // Check if directory contains any files that will be included
                if has_included_files(&path, filter_list, ignore_list)? {
                    print_file_tree(base_path, &path, filter_list, ignore_list, depth + 1)?;
                }
            } else {
                // Check if this file will be included
                if filter_list.is_empty() || should_include_file(&path, filter_list) {
                    let relative_path = path.strip_prefix(base_path).unwrap_or(&path);
                    println!("{}  - 📄 {}", indent, relative_path.display());
                }
            }
        }
    }
    
    Ok(())
}
