
pub fn part1(input: &Vec<String>) -> i64 {
    println!("{:?}", input);
    42
}

pub fn part2(input: &Vec<String>) -> i64 {
    84
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = vec!["foo".to_string(), "bar".to_string()];
        let result = part1(&test_input);
        let result = part1(&test_input);
        assert_eq!(result, 42);
    }

    #[test]
    fn part2_works() {
        let test_input = vec!["foo".to_string(), "bar".to_string()];
        let result = part2(&test_input);
        assert_eq!(result, 84);
    }
}
