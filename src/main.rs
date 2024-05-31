// Import necessary modules and types from the standard library.
use std::env;
use std::fs::{self};
use std::path::PathBuf;

// The `main` function is the entry point of every executable Rust program.
fn main() {
    // Retrieve command-line arguments. `env::args()` returns an iterator of the command line arguments.
    let args: Vec<String> = env::args().collect();

    // Retrieve the user's home directory from the environment using the "HOME" variable.
    // `expect` is used to handle the Result; it will crash the program with the specified message if the result is an error.
    let home_dir = env::var("HOME").expect("HOME environment variable not set");

    // Determine the directory to operate on based on command-line arguments or default to ~/Documents.
    // `PathBuf` is used for constructing and managing file paths.
    let directory = match args.get(1) {
        Some(dir) => PathBuf::from(dir), // Use the directory specified as a command line argument.
        None => PathBuf::from(home_dir).join("Documents"), // Default to the Documents directory in the home directory.
    };

    // Define a hashmap to map file extensions to their corresponding folder names.
    // `.iter().cloned().collect()` is used to create a HashMap from an array of tuples.
    let extensions = [
        (".jpg", "Images"), (".jpeg", "Images"), (".png", "Images"),
        (".gif", "Images"), (".mp4", "Videos"), (".mov", "Videos"),
        (".avi", "Videos"), (".doc", "Documents"), (".pdf", "Documents"),
        (".txt", "Documents"), (".mp3", "Music"), (".wav", "Music"),
    ].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    // Check if the specified directory is indeed a directory and not a file or a nonexistent path.
    if directory.is_dir() {
        // Read the contents of the directory.
        // `read_dir` returns an iterator that yields entries within a directory.
        if let Ok(entries) = fs::read_dir(directory.clone()) {
            for entry in entries.filter_map(Result::ok) { // Use `filter_map` to filter out errors and unwrap `Result`s.
                let path = entry.path(); // Get the path of the current entry.
                if path.is_file() { // Check if the path represents a file.
                    if let Some(extension) = path.extension() { // Get the file extension, returned as an `Option`.
                        if let Some(extension) = extension.to_str() { // Convert the `OsStr` to a string slice if possible.
                            if let Some(folder_name) = extensions.get(extension) { // Retrieve the folder name based on the file extension.
                                let folder_path = directory.join(folder_name); // Join paths to construct the new folder path.
                                fs::create_dir_all(&folder_path).expect("Failed to create directory"); // Create the directory if it doesn't exist.
                                let destination_path = folder_path.join(path.file_name().unwrap()); // Append the file name to the folder path.
                                fs::rename(&path, &destination_path).expect("Failed to move file"); // Move the file to the new location.
                                println!("Moved {:?} to {} folder", path.file_name().unwrap(), folder_name); // Print a success message.
                            } else {
                                println!("Skipping {:?}", path.file_name().unwrap()); // Print a message for files with unrecognized extensions.
                            }
                        }
                    }
                } else {
                    println!("Skipping {:?} as it is a directory", path.file_name().unwrap()); // Print a message for directories.
                }
            }
        }
    }

    println!("Done"); // Indicate that the program has finished executing.
}
