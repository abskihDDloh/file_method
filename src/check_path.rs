use std::fs;
use std::path::{Path, PathBuf};

/// Converts the given source path to a full path.
///
/// # Arguments
///
/// * `src_path` - The source path to be converted.
///
/// # Returns
///
/// Returns a `Result` containing the full path as a `PathBuf` if successful, or an `std::io::Error` if an error occurs.
pub fn get_full_path(src_path: &Path) -> std::io::Result<PathBuf> {
    let full_path = fs::canonicalize(src_path)?;
    Ok(full_path)
}

/// Converts the given `PathBuf` to a string representation.
///
/// # Arguments
///
/// * `src_path` - The `Path` to be converted.
///
/// # Returns
///
/// Returns a `Result` containing the string representation of the path if successful, or an `std::io::Error` if an error occurs.
fn get_path_str(src_path: &Path) -> std::io::Result<String> {
    let path_str_option: Option<&str> = src_path.to_str();
    let path_str = match path_str_option {
        Some(s) => s.to_string(),
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to convert path to string",
            ))
        }
    };
    Ok(path_str)
}

/// Checks if the given directory path is valid.
///
/// # Arguments
///
/// * `directory_path` - The directory path to be checked.
///
/// # Returns
///
/// Returns a `Result` containing the full path as a `PathBuf` if the directory is valid, or an `std::io::Error` if it is not valid.
pub fn is_valid_directory(directory_path: &Path) -> std::io::Result<PathBuf> {
    let full_path: PathBuf = get_full_path(directory_path)?;
    let path_str: String = get_path_str(&full_path)?;
    if !full_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{} is not a directory", path_str),
        ));
    }
    Ok(full_path)
}

/// Checks if the given file path is valid.
///
/// # Arguments
///
/// * `file_path` - The file path to be checked.
///
/// # Returns
///
/// Returns a `Result` containing the full path as a `PathBuf` if the file is valid, or an `std::io::Error` if it is not valid.
pub fn is_valid_file(file_path: &Path) -> std::io::Result<PathBuf> {
    let full_path: PathBuf = get_full_path(file_path)?;
    let path_str: String = get_path_str(&full_path)?;
    if !full_path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{} is not a file", path_str),
        ));
    }
    Ok(full_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_directory_valid_directory() {
        let path = Path::new("test_file");
        let result = is_valid_directory(path);
        assert!(result.is_ok());
        let _fill_path = result.unwrap();
    }

    #[test]
    fn test_is_valid_directory_invalid_directory() {
        let path = Path::new("test_file/dummy_not_target_files_dir/file1.txt");
        let result = is_valid_directory(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_directory_failed_conversion() {
        let path = Path::new("nonexistent_dir");
        let result = is_valid_directory(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_file_valid_file() {
        let path = Path::new("test_file/dummy_target_files_dir/file2.pdf");
        let result = is_valid_file(path);
        assert!(result.is_ok());
        let _full_path = result.unwrap();
    }

    #[test]
    fn test_is_valid_file_invalid_file() {
        let path = Path::new("test_file");
        let result = is_valid_file(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_file_failed_conversion() {
        let path = Path::new("nonexistent_file.pdf");
        let result = is_valid_file(path);
        assert!(result.is_err());
    }
}
