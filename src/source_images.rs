use std::iter::Iterator;
use std::path::PathBuf;

use glob::{glob_with, MatchOptions};

#[derive(Debug)]
pub struct SourceImage {
    pub path: PathBuf,
}

pub fn search(root: &String) -> Vec<SourceImage> {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    glob_with(&format!("{}/**/*.jpg", root), options)
        .unwrap()
        .chain(glob_with(&format!("{}/**/*.jpeg", root), options).unwrap())
        .filter_map(|entry| entry.ok().map(|path| SourceImage { path: path }))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempdir::TempDir;

    static NO_IMAGES_DIRNAME: &str = "no_images";
    static TEST_IMAGES_DIRNAME: &str = "test_images";

    pub fn setup_temp_dir() -> TempDir {
        let temp_dir = TempDir::new("source_images_unittest").unwrap();

        std::fs::create_dir(temp_dir.path().join(NO_IMAGES_DIRNAME)).unwrap();

        let images_dir_path = temp_dir.path().join(TEST_IMAGES_DIRNAME);
        std::fs::create_dir(&images_dir_path).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image1.jpg")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image2.JPG")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image3.jpeg")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("image4.JPEG")).unwrap();
        std::fs::File::create(images_dir_path.clone().join("other-file.txt")).unwrap();

        temp_dir
    }

    #[test]
    fn no_images_found_in_empty_dir() {
        let temp_dir = setup_temp_dir();
        let root = String::from(temp_dir.path().join(NO_IMAGES_DIRNAME).to_str().unwrap());

        let source_images = search(&root);
        assert_eq!(source_images.len(), 0);
    }

    #[test]
    fn test_images_found_in_dir() {
        let temp_dir = setup_temp_dir();
        let root = String::from(temp_dir.path().join(TEST_IMAGES_DIRNAME).to_str().unwrap());

        let source_images = search(&root);
        assert_eq!(source_images.len(), 4);
    }

    #[test]
    fn images_are_searched_recursively() {
        let temp_dir = setup_temp_dir();
        let root = String::from(temp_dir.path().to_str().unwrap());

        let source_images = search(&root);
        assert_eq!(source_images.len(), 4);
    }
}
