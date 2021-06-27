mod cl_args;

fn main() {
    let args = cl_args::parse_args();

    println!("Ready {:?}", args);
}
