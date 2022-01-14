use std::collections::HashMap;
use std::path::Path;
use chrono::NaiveDateTime;
use persistance::FotoboekDatabase;
use rocket::serde::json::Json;
use persistance::queries::gallery;
use persistance::queries::gallery::GalleryFileInfo;
use serde::Serialize;

#[get("/gallery/paths")]
pub async fn get_paths(db: FotoboekDatabase) -> Json<GalleryPath> {
    let gallery_file_infos = gallery::get_gallery_file_infos(&db).await;
    let path_structure = create_gallery_path_structure(gallery_file_infos);
    Json(path_structure)
}

#[derive(Serialize, Debug, PartialEq)]
pub struct GalleryPath {
    pub sub_paths: HashMap<String, GalleryPath>,
    pub files: Vec<GalleryFile>,
}

impl GalleryPath {
    fn empty() -> GalleryPath {
        GalleryPath {
            sub_paths: HashMap::new(),
            files: Vec::new(),
        }
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct GalleryFile {
    pub id: i32,
    pub file_type: String,
    pub effective_date: NaiveDateTime,
}

fn create_gallery_path_structure(file_infos: Vec<GalleryFileInfo>) -> GalleryPath {
    let empty_path = Path::new("");

    let mut gallery_root = GalleryPath::empty();

    file_infos.iter().for_each(|file_info| {
        let path = Path::new(&file_info.rel_path);
        let sub_paths: Vec<_> = path.ancestors()
            .skip(1)
            .filter(|subpath| !empty_path.eq(*subpath))
            .collect();

        let matching_gallery_item = sub_paths.iter()
            .rev()
            .fold(&mut gallery_root, |gallery_item, sub_path| {
                gallery_item.sub_paths
                    .entry(sub_path.file_name().unwrap().to_str().unwrap().to_string())
                    .or_insert(GalleryPath::empty())
            });

        let gallery_file = create_gallery_file(&file_info);
        matching_gallery_item.files.push(gallery_file);
    });

    gallery_root
}

fn create_gallery_file(file_info: &GalleryFileInfo) -> GalleryFile {
    GalleryFile {
        id: file_info.file_id,
        file_type: file_info.file_type.clone(),
        effective_date: file_info.effective_date.clone(),
    }
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
