use rm_regex::destructure_args;

fn main() {
    let (expression, starting_dir, dir, file) = destructure_args();
    println!("express: {expression}");

    match starting_dir {
        Some(buf) => println!("starting dir: {:?}", buf),
        None => println!("No staring dir")
    }

    println!("dir: {dir}");
    println!("file: {file}");
}
