use std::collections::HashSet;
use std::io;
use std::path::Path;

pub fn collect_source_files(paths: &Vec<&str>, extension: &str) -> io::Result<HashSet<String>> {
    let mut collected = HashSet::new();

    for path in paths {
        if let Err(err) = collect_files_in_path(Path::new(path), extension, &mut collected) {
            println!("failed to check {} due to error {}", path, err);
        }
    }

    Ok(collected)
}

fn collect_files_in_path(dir: &Path, extension: &str, collected: &mut HashSet<String>) -> io::Result<()> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        if entry.path().is_file() && entry.file_name().to_string_lossy().ends_with(extension) {
            collected.insert(entry.path().to_string_lossy().to_string());
        }

        if entry.path().is_dir() {
            if let Err(err) = collect_files_in_path(&entry.path(), extension, collected) {
                println!(
                    "failed to collect files from path {} due to error {}",
                    entry.path().to_string_lossy().to_string(),
                    err
                );
            }
        }
    }

    Ok(())
}
