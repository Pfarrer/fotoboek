mod cl_args;
mod data_store;
mod source_images;

use crate::data_store::DataStore;

fn main() {
    env_logger::init();
    let args = cl_args::parse_args().unwrap();
    let data_store = DataStore::open(&args.data_root).unwrap();
    let source_image = source_images::search(&args.image_root);

    // for source_image in source_image {
    //     println!("Found image: {:?}", source_image);
    // }
}
