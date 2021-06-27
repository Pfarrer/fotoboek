use std::path::{Path, PathBuf};
use std::fs::ReadDir;
use std::iter::Iterator;

pub struct SourceImages {
    iterator: ReadDir,
    recursive_source_images: Option<Box<SourceImages>>,
}

#[derive(Debug)]
pub struct SourceImage {
    pub path: PathBuf
}

impl Iterator for SourceImages {
    type Item = SourceImage;

    fn next(&mut self) -> Option<SourceImage> {
        if let Some(_) = &self.recursive_source_images {
            let next = self.recursive_source_images..next();
            if let Some(_) = next {
                return next;
            } else {
                self.recursive_source_images = None;
            }
        }

        match self.iterator.next() {
            None => None,
            Some(result) => {
                let dir_entry = result.unwrap();
                if dir_entry.file_type().unwrap().is_file() {
                    Some(SourceImage {
                        path: dir_entry.path()
                    })

                } else if dir_entry.file_type().unwrap().is_dir() {
                    None // Recurse here
                } else {
                    None
                }
            }
        }
    }
}

pub fn search(root: &String) -> impl Iterator<Item = SourceImage> {
    let dir_iterator = Path::new(root).read_dir().unwrap();
    
    SourceImages {
        iterator: dir_iterator,
        recursive_source_images: None
    }
//     let options = MatchOptions {
//         case_sensitive: false,
//         require_literal_separator: false,
//         require_literal_leading_dot: false,
//     };
    
//     let pattern = format!("{}/**/*.jpg", root);
//     glob_with(&pattern, options).unwrap().filter_map(|entry| {
//         if let Ok(path) = entry {
//             let source_image = SourceImage {
//                 path: path
//             };
        
//             Some(source_image)
//         } else {
//             None
//         }
//     }).collect()
}