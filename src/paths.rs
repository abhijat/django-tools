use std::collections::HashSet;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

fn lossy_name(entry: &DirEntry) -> String {
    entry.path().to_string_lossy().to_string()
}

pub fn collect_source_files(paths: &Vec<&str>, extension: &str) -> HashSet<String> {
    paths
        .iter()
        .map(|path| collect_files_in_path(Path::new(path), extension))
        .filter_map(Result::ok)
        .fold(HashSet::new(), |acc, x| acc.into_iter().chain(x).collect())
}

fn collect_files_in_path(dir: &Path, extension: &str) -> io::Result<HashSet<String>> {
    let mut entries = HashSet::new();

    for entry in dir.read_dir()? {
        let entry = entry?;
        if entry.path().is_file() && entry.file_name().to_string_lossy().ends_with(extension) {
            entries.insert(lossy_name(&entry));
        }

        if entry.path().is_dir() {
            match collect_files_in_path(&entry.path(), extension) {
                Ok(subpath_entries) => entries.extend(subpath_entries.into_iter()),
                Err(err) => println!("error traversing subpath {}: {}", lossy_name(&entry), err),
            }
        }
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::path::PathBuf;

    use super::*;

    fn path() -> PathBuf {
        PathBuf::from("test_data")
    }

    #[test]
    fn can_find_files_of_extension() {
        let files = collect_files_in_path(&path(), ".js").unwrap();
        assert_eq!(1, files.len());

        let files = collect_files_in_path(&path(), ".py").unwrap();
        assert_eq!(2, files.len());

        let files = collect_files_in_path(&path(), ".pbj").unwrap();
        assert!(files.is_empty());
    }
}
