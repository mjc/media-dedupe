use camino::Utf8Path;
use walkdir::{DirEntry, WalkDir};

pub fn hi() {
    println!("hi");
}

pub fn get_all_files(path: &Utf8Path) -> Vec<DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter(|entry| !entry.file_type().is_dir())
        .collect()
}

#[cfg(test)]
mod tests {
    use camino::Utf8Path;

    use crate::locate::get_all_files;

    #[test]
    fn test_walk() {
        let path = Utf8Path::new("tests");
        assert!(get_all_files(path).len() > 1);
    }
}
