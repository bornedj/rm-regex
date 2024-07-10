use rm_regex::{collect_entries, destructure_args};

fn main() {
    let (_, starting_dir, dir, file) = destructure_args();
    let entries = collect_entries(&starting_dir).expect("Failed to find entries under the specified starting directory.");
    println!("{:?}", entries);
}
