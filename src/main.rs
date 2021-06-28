mod cl_args;
mod source_images;

fn main() {
    let args = cl_args::parse_args().unwrap();
    let source_image = source_images::search(&args.image_root);

    for source_image in source_image {
        println!("Found image: {:?}", source_image);
    }
}
