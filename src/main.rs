use std::path::Path;

use rm_regex::{collect_entries, destructure_args, print_collected_entries, visit_dirs};

fn main() {
    let (_, starting_dir, dir, file) = destructure_args();
    let entries = collect_entries(&starting_dir).expect("Failed to find entries under the specified starting directory.");
    println!("{:?}", entries);

    visit_dirs(&Path::new(&starting_dir), &print_collected_entries);
}
