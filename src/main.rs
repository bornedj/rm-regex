use rm_regex::{collect_entries, destructure_args};

fn main() {
    let (expression, starting_dir, dir, file) = destructure_args();
    println!("express: {expression}");
    println!("{}",&starting_dir);
    println!("dir: {dir}");
    println!("file: {file}");

    let entries = collect_entries(&starting_dir).expect("Failed to find entries under the specified starting directory.");
    println!("{:?}", entries);
}
