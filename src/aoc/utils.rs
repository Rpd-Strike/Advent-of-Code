use std::{fs::File, io::{self, BufRead}, path::Path};

const ROOT_PATH: &str = "../";

pub fn input_lines_iter() -> impl Iterator<Item = String> {
    read_lines(format!("{ROOT_PATH}/data/input.txt")).unwrap().flatten()
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}