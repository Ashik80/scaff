use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub fn scaffold_dir_file(
    directory: &PathBuf,
    file_name: &str,
    content: &[u8],
) -> Result<PathBuf, String> {
    fs::create_dir_all(directory)
        .map_err(|e| format!("failed to create {} directory: {}", directory.display(), e))?;

    let file_path = directory.join(file_name);
    let mut file =
        File::create(&file_path).map_err(|e| format!("failed to create {}: {}", file_name, e))?;

    file.write_all(content)
        .map_err(|e| format!("failed to write {}: {}", file_name, e))?;

    Ok(file_path)
}
