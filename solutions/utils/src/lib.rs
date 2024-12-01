use std::{
    fs::File,
    io::{prelude::*, BufReader},
    env,
    path::PathBuf
};


pub fn lines_from_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


pub fn get_input_file(day: &str) -> String {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let inputs_dir = manifest_dir.join("../../inputs");
    let filename = format!("{}/{}.in", inputs_dir.display(), day);
    filename
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = lines_from_file("src/test_lines_from_file.txt");
        assert_eq!(result, vec!["some test data", "some more test data", "!!! 1234", "", "foo"]);
    }
}
