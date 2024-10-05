use std::path::Path;

use regex::Regex;
use rm_regex::{destructure_args, visit_dirs};

fn main() {
    let (expression, starting_dir) = destructure_args();
    let re = Regex::new(&expression).unwrap();
    let path_buf = Path::new(&starting_dir);
    let entries = visit_dirs(&path_buf.into(), &re);
    println!("{:?}", entries);
}
