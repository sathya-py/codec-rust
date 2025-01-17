use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use walkdir::WalkDir;
use rayon::prelude::*;
use clap::{Arg, Command};

fn find_files(
    directory: &str,
    valid_extensions: &HashSet<&str>,
    skip_extensions: &HashSet<&str>,
    skip_folders: &HashSet<&str>,
) -> Vec<PathBuf> {
    let mut file_list = Vec::new();

    for entry in WalkDir::new(directory) {
        let entry = entry.unwrap();
        let path = entry.path();

        // Debugging output
        println!("Checking path: {}", path.display());

        if path.is_dir() {
            if skip_folders.contains(path.file_name().unwrap().to_str().unwrap()) {
                println!("Skipping folder: {}", path.display());
                continue;
            }
        } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_str().unwrap().to_lowercase(); // Normalize to lowercase
            // Debugging output for file extension
            println!("Found file with extension: {}", ext_str);
            if valid_extensions.contains(ext_str.as_str()) && !skip_extensions.contains(ext_str.as_str()) {
                println!("Found valid file: {}", path.display());
                file_list.push(path.to_path_buf());
            } else {
                println!("Skipping file: {} (extension: {})", path.display(), ext_str);
                if !valid_extensions.contains(ext_str.as_str()) {
                    println!("Reason: Not a valid extension.");
                }
                if skip_extensions.contains(ext_str.as_str()) {
                    println!("Reason: Extension is in skip list.");
                }
            }
        }
    }

    file_list
}

fn process_file(file_path: &Path, include_full_path: bool, base_directory: &str) -> (String, Option<String>) {
    let relative_path = if include_full_path {
        file_path.display().to_string()
    } else {
        file_path.strip_prefix(base_directory).unwrap().display().to_string()
    };

    match fs::read_to_string(file_path) {
        Ok(content) => (relative_path, Some(content)),
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path.display(), e);
            (relative_path, None)
        }
    }
}

fn create_summary_file(file_list: Vec<(String, Option<String>)>, output_file: &str) -> io::Result<()> {
    let mut file = File::create(output_file)?;

    for (relative_path, content) in file_list {
        writeln!(file, "Path: {}", relative_path)?;
        writeln!(file, "{}", "=".repeat(relative_path.len()))?;
        
        if let Some(content) = content {
            writeln!(file, "{}", content)?;
        }

        writeln!(file, "-------------------------------------------------")?;
        writeln!(file)?;
    }

    Ok(())
}

fn validate_arguments(directory: &str) -> Result<(), String> {
    if !Path::new(directory).is_dir() {
        return Err(format!("The provided directory '{}' does not exist.", directory));
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define command-line arguments using clap
    let matches = Command::new("File Finder")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Finds specified files in a directory and creates a summary.")
        .arg(Arg::new("directory")
            .help("The directory to search in")
            .required(true)
            .index(1))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .default_value("summary.txt")
            .help("Output summary file name (default: summary.txt)"))
        .arg(Arg::new("skip")
            .short('s')
            .long("skip")
            .num_args(1..) // Accept multiple values
            .help("List of extensions to skip (e.g., .ico .jpg)"))
        .arg(Arg::new("extensions")
            .short('e')
            .long("extensions")
            .num_args(1..) // Accept multiple values
            .help("List of valid extensions to look for (e.g., .txt .dart .json)"))
        .arg(Arg::new("full-path")
            .long("full-path")
            .action(clap::ArgAction::SetTrue)
            .help("Include full paths in the output"))
        .arg(Arg::new("skip-folders")
            .long("skip-folders")
            .num_args(1..) // Accept multiple values
            .help("List of folder names to skip (e.g., folder1 folder2)"))
        // Automatically add help and version flags
        .get_matches();

    // Retrieve arguments
    let directory = matches.get_one::<String>("directory").unwrap();
    
    validate_arguments(directory)?;

    // Default valid and skip extensions
    let valid_extensions: HashSet<&str> = [
        ".txt", ".py", ".c", ".cpp", ".h", 
        ".cs", ".cake", ".cshtml", ".csx", 
        ".ps1", ".vbs", 
        ".js", ".mjs", 
        ".ts", 
        ".svelte",
        ".rb", 
        ".rs", 
        ".html", ".htm", ".xhtml",
        ".css", 
        ".dart",
        ".jsx",
        ".bat",
        ".autoexe",
        // Additional extensions
        ".sh",
        ".bash",
        ".php", ".phtml",
        ".pl", ".pm",
        ".sql",
        ".xml",
        ".csv"
    ].iter().cloned().collect();

    // Debugging output for valid extensions
    println!("Valid extensions: {:?}", valid_extensions);

    let skip_extensions: HashSet<String> = matches.get_many::<String>("skip").map(|v| v.map(|s| s.clone()).collect()).unwrap_or_default();
    
    // Debugging output for skip extensions
    println!("Skip extensions: {:?}", skip_extensions);

    let skip_folders: HashSet<String> = matches.get_many::<String>("skip-folders").map(|v| v.map(|s| s.clone()).collect()).unwrap_or_default();

    // Find files in the specified directory
    let found_files = find_files(directory, &valid_extensions, &skip_extensions.iter().map(|s| s.as_str()).collect(), &skip_folders.iter().map(|s| s.as_str()).collect());

    if !found_files.is_empty() {
        
        // Process files in parallel
        let results: Vec<_> = found_files.into_par_iter()
            .map(|file| process_file(&file, matches.get_flag("full-path"), directory))
            .collect();

        // Create summary file
        create_summary_file(results, matches.get_one::<String>("output").unwrap())?;
        
        println!("Summary created: {}", matches.get_one::<String>("output").unwrap());
    } else {
        println!("No matching files found.");
    }

    Ok(())
}
