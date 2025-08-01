use std::env;
use std::fs;
use std::path::Path;

fn list_directory(path: &Path) -> Vec<String> {
    let mut entries = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                entries.push(file_name);
            }
        }
    } else {
        eprintln!("Failed to read directory: {}", path.display());
    }

    entries.sort();
    entries
}

fn main() {
    println!("Hello to drivediff!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: drive-diff <folder1> <folder2>");
        return;
    }

    let folder1 = Path::new(&args[1]);
    let folder2 = Path::new(&args[2]);

    if !folder1.exists() || !folder2.exists() {
        eprintln!("Both folders must exist.");
        return;
    }

    println!("Listing contents of {}:", folder1.display());
    let list1 = list_directory(folder1);
    for entry in &list1 {
        println!("  {}", entry);
    }

    println!("Listing contents of {}:", folder2.display());
    let list2 = list_directory(folder2);
    for entry in &list2 {
        println!("  {}", entry);
    }
}
