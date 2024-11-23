use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::check_path::is_valid_directory;

/// # 概要
/// この関数は指定されたディレクトリ内の指定された拡張子のファイルを探し、そのパスのリストを返します。
///
/// # 引数
/// * `directory_path`: ファイルを探すディレクトリのパスを指定します。
/// * `extension`: ファイルの拡張子を指定します。
///
/// # 戻り値
/// ファイルが見つかった場合はそのパスのリストを返します。
/// ファイルが見つからなかった場合は空のリストを返します。
/// ディレクトリが無効な場合はエラーを返します。
/// 拡張子が指定されていない場合はエラーを返します。
///
/// # 例
/// ```
/// let result = seek_file(Path::new("/path/to/directory"));
/// match result {
///     Ok(paths) => for path in paths {
///         println!("Found at: {}", path.display());
///     },
///     Err(e) => println!("An error occurred: {}", e),
/// }
/// ```
///
pub fn seek_file_by_extension(
    directory_path: &Path,
    extension: &str,
) -> std::io::Result<Vec<std::path::PathBuf>> {
    let src_dir: std::path::PathBuf = match is_valid_directory(directory_path) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };
    //extensionが指定されていない場合はエラーを返す。
    if extension.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Extension is not specified.",
        ));
    }
    let mut files = Vec::new();
    for entry in fs::read_dir(src_dir.as_path())? {
        let entry: fs::DirEntry = entry?;
        let path: PathBuf = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(extension) {
            files.push(path);
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seek_file_empty_directory() {
        let dir_str: &str = "test_file/empty_dir";
        let dmpty_dir_path = Path::new(dir_str);
        //file_pathのディレクトリが存在しない場合は作成する。作成に失敗した場合はpanicする。
        fs::create_dir_all(dmpty_dir_path).expect("COULD NOT MAKE DIRECTORY.");
        let files = seek_file_by_extension(dmpty_dir_path, "txt").unwrap();
        assert_eq!(files.len(), 0);
        //file_pathのディレクトリが存在している場合は削除する。削除に失敗した場合はpanicする。
        fs::remove_dir_all(dmpty_dir_path).expect("COULD NOT REMOVE DIRECTORY.");
    }

    #[test]
    fn test_seek_file_not_specifiled_extension() {
        let temp_dir: PathBuf = PathBuf::from("test_file/dummy_target_files_dir");
        let result = seek_file_by_extension(temp_dir.as_path(), "");
        assert!(result.is_err());
    }

    #[test]
    fn test_seek_file_with_files() {
        let temp_dir: PathBuf = PathBuf::from("test_file/dummy_target_files_dir");

        let mut file1: PathBuf = temp_dir.clone();
        file1.push("file1.pdf");
        let full_path_file1 = fs::canonicalize(file1.as_path()).unwrap();

        let mut file2: PathBuf = temp_dir.clone();
        file2.push("file2.pdf");
        let full_path_file2 = fs::canonicalize(file2.as_path()).unwrap();

        let mut file3: PathBuf = temp_dir.clone();
        file3.push("file3.pdf");
        let full_path_file3 = fs::canonicalize(file3.as_path()).unwrap();

        let files: Vec<std::path::PathBuf> =
            seek_file_by_extension(temp_dir.as_path(), "pdf").unwrap();
        assert_eq!(files.len(), 3);
        assert!(files.contains(&full_path_file1));
        assert!(files.contains(&full_path_file2));
        assert!(files.contains(&full_path_file3));
    }

    #[test]
    fn test_seek_file_with_non_files() {
        let temp_dir: PathBuf = PathBuf::from("test_file/dummy_not_target_files_dir");
        let files = seek_file_by_extension(temp_dir.as_path(), "pdf").unwrap();
        assert_eq!(files.len(), 0);
    }
}
