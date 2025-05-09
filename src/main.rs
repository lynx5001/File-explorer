// include filesystem manipulation operations
use std::fs;
use std::io;
//use std::path::Path;

fn main() -> io::Result<()> {
    println!("Hello, File Explorer!");

    // set current dir
    let current_dir = "."; // Current directory

    // Read the directory
    let entries = fs::read_dir(current_dir)?;

    // Iterate through the entries and print their names
    println!("Files and directories in '{}':", current_dir);
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("[DIR] {}", path.display());
        } else {
            println!("[FILE] {}", path.display());
        }
    }

    Ok(())
}