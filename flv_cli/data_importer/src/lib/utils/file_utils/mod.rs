use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Gets a vector of file paths from the given directory.
///
/// # Parameters
///
/// * `path` - The path of the directory to get file paths from. Must implement `AsRef<Path>`.
///
/// # Returns
///
/// A Result containing a vector of `PathBuf` representing the file paths found, or an `io::Error` if the directory could not be read.
///
/// # Remarks
///
/// This filters out hidden files and directories, only returning file paths for visible files.
pub fn get_file_paths_from_directory<P>(path: P) -> Result<Vec<PathBuf>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)
        .expect("Failed to read directory")
        .filter(|r| !is_file_hidden(r.as_ref().unwrap()))
        .map(|r| r.map(|d| d.path()))
        .filter(|r| r.is_ok() && r.as_deref().unwrap().is_file())
        .collect()
}

/// Checks if the given file entry is hidden.
///
/// # Parameters
///
/// * `entry` - The DirEntry to check if hidden
///
/// # Returns
///
/// Returns true if the entry's file name starts with a '.', false otherwise
pub fn is_file_hidden(entry: &DirEntry) -> bool {
    // https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
