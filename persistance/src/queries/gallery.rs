use std::collections::HashMap;
use std::path::Path;
use serde::Serialize;

use crate::FotoboekDatabase;
use crate::models::File;

#[derive(Serialize, Debug, PartialEq)]
pub struct GalleryPath {
    pub preview_image_id: Option<i32>,
    pub sub_paths: HashMap<String, GalleryPath>,
    pub files: Vec<GalleryFile>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct GalleryFile {
    pub id: i32,
    pub file_type: String,
}

pub async fn paths(db: FotoboekDatabase) -> GalleryPath {
    let files = File::all(db).await;
    create_gallery_path_structure(files)
}

fn create_gallery_path_structure(files: Vec<File>) -> GalleryPath {
    let empty_path = Path::new("");

    let mut gallery_root = GalleryPath {
        preview_image_id: None,
        sub_paths: HashMap::new(),
        files: Vec::new(),
    };

    files.iter().for_each(|file| {
        let path = Path::new(&file.rel_path);
        let subpaths: Vec<_> = path.ancestors()
            .skip(1)
            .filter(|subpath| !empty_path.eq(*subpath))
            .collect();

        let matching_gallery_item = subpaths.iter()
            .rev()
            .fold(&mut gallery_root, |gallery_item, subpath| {
                gallery_item.sub_paths
                    .entry(subpath.file_name().unwrap().to_str().unwrap().to_string())
                    .or_insert(GalleryPath {
                        preview_image_id: file.id,
                        sub_paths: HashMap::new(),
                        files: Vec::new(),
                    })
            });

        matching_gallery_item.files.push(GalleryFile {
            id: file.id.unwrap(),
            file_type: file.file_type.clone(),
        });
    });

    gallery_root
}

// TODO
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use maplit::hashmap;
//
//     const IMAGE_TYPE: &str = "IMAGE";
//
//     #[test]
//     fn paths_without_files() {
//         let expected = GalleryPath {
//             sub_paths: HashMap::new(),
//             files: Vec::new(),
//         };
//         let actual = create_gallery_path_structure(vec![]);
//         assert_eq!(expected, actual)
//     }
//
//     #[test]
//     fn paths_with_single_file_one_subdir() {
//         let expected = GalleryPath {
//             sub_paths: hashmap![
//                 "subdir".to_string() => GalleryPath {
//                     sub_paths: hashmap![],
//                     files: vec![
//                         GalleryFile { id: 1, file_type: IMAGE_TYPE.to_string() }
//                     ],
//                 }
//             ],
//             files: vec![],
//         };
//         let actual = create_gallery_path_structure(vec![
//             File {
//                 id: Some(1),
//                 rel_path: "subdir/file.jpg".to_string(),
//                 file_type: IMAGE_TYPE.to_string(),
//                 file_name: "file.jpg".to_string(),
//             }
//         ]);
//         assert_eq!(expected, actual)
//     }
//
//     #[test]
//     fn paths_with_single_file_two_subdirs() {
//         let expected = GalleryPath {
//             sub_paths: hashmap![
//                 "subdir1".to_string() => GalleryPath {
//                     sub_paths: hashmap![
//                         "subdir2".to_string() => GalleryPath {
//                             sub_paths: hashmap![],
//                             files: vec![
//                                 GalleryFile { id: 1, file_type: IMAGE_TYPE.to_string() }
//                             ],
//                         }
//                     ],
//                     files: vec![],
//                 }
//             ],
//             files: vec![],
//         };
//         let actual = create_gallery_path_structure(vec![
//             File {
//                 id: Some(1),
//                 rel_path: "subdir1/subdir2/file.jpg".to_string(),
//                 file_type: IMAGE_TYPE.to_string(),
//                 file_name: "file.jpg".to_string(),
//             }
//         ]);
//         assert_eq!(expected, actual)
//     }
//
//     #[test]
//     fn paths_multiple_files() {
//         let expected = GalleryPath {
//             sub_paths: hashmap![
//                 "subdir1".to_string() => GalleryPath {
//                     sub_paths: hashmap![],
//                     files: vec![
//                         GalleryFile { id: 10, file_type: IMAGE_TYPE.to_string() },
//                         GalleryFile { id: 11, file_type: IMAGE_TYPE.to_string() },
//                     ],
//                 },
//                 "subdir2".to_string() => GalleryPath {
//                     sub_paths: hashmap![
//                         "subdir21".to_string() => GalleryPath {
//                             sub_paths: hashmap![],
//                             files: vec![
//                                 GalleryFile { id: 30, file_type: IMAGE_TYPE.to_string() },
//                             ],
//                         },
//                     ],
//                     files: vec![
//                         GalleryFile { id: 21, file_type: IMAGE_TYPE.to_string() },
//                         GalleryFile { id: 20, file_type: IMAGE_TYPE.to_string() },
//                     ],
//                 },
//             ],
//             files: vec![],
//         };
//         let actual = create_gallery_path_structure(vec![
//             File {
//                 id: Some(10),
//                 rel_path: "subdir1/file1.jpg".to_string(),
//                 file_type: IMAGE_TYPE.to_string(),
//                 file_name: "file1.jpg".to_string(),
//             },
//             File {
//                 id: Some(11),
//                 rel_path: "subdir1/file2.jpg".to_string(),
//                 file_type: IMAGE_TYPE.to_string(),
//                 file_name: "file2.jpg".to_string(),
//             },
//             File {
//                 id: Some(21),
//                 rel_path: "subdir2/file1.jpg".to_string(),
//                 file_type: IMAGE_TYPE.to_string(),
//                 file_name: "file1.jpg".to_string(),
//             },
//             File {
//                 id: Some(20),
//                 rel_path: "subdir2/file2.jpg".to_string(),
//                 file_type: IMAGE_TYPE.to_string(),
//                 file_name: "file2.jpg".to_string(),
//             },
//             File {
//                 id: Some(30),
//                 rel_path: "subdir2/subdir21/file1.jpg".to_string(),
//                 file_type: IMAGE_TYPE.to_string(),
//                 file_name: "file1.jpg".to_string(),
//             },
//         ]);
//         assert_eq!(expected, actual)
//     }
// }
