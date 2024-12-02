
pub fn part1(input: &Vec<String>) -> i64 {
    42
}

pub fn part2(input: &Vec<String>) -> i64 {
    84
}


#[cfg(test)]
mod tests {
    use utils;
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = utils::lines_from_file("src/test.in");
        let result = part1(&test_input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let test_input = utils::lines_from_file("src/test.in");
        let result = part2(&test_input);
        assert_eq!(result, 4);
    }

}
