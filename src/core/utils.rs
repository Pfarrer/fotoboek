use std::path::Path;

pub fn abs_to_rel_path(abs_path: &str) -> &str {
    let image_root_string = dotenv::var("IMAGE_ROOT").unwrap();
    let rel_path = Path::new(abs_path).strip_prefix(image_root_string).unwrap();
    rel_path.to_str().unwrap()
}
