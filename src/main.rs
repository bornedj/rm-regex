use std::path::Path;

use rm_regex::{destructure_args, visit_dirs};

fn main() {
    let (_, starting_dir, _, _) = destructure_args();
    let path_buf = Path::new(&starting_dir);
    let entries = visit_dirs(&path_buf.into());
    println!("{:?}", entries);
}
