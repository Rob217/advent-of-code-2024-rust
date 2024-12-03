use regex::Regex;

pub fn part1(input: &Vec<String>) -> i64 {
    let mut result = 0;
    for row in input {
        result += calculate_all_multiplications_in_row(row);
    }
    result
}

pub fn part2(input: &Vec<String>) -> i64 {
    84
}


// find all instances of pattern "mul(X,Y)" in a string where X and Y are 1-3 digit numbers using regex
fn find_all_matches_in_row_regex(row: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let matches = re.find_iter(row);
    let mut result = Vec::new();
    for m in matches {
        result.push(m.as_str());
    }
    result
}

fn calculate_value_of_mul(mul: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let captures = re.captures(mul).unwrap();
    let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
    x * y
}

fn calculate_all_multiplications_in_row(row: &str) -> i64 {
    let matches = find_all_matches_in_row_regex(row);
    let mut result = 0;
    for m in matches {
        result += calculate_value_of_mul(m);
    }
    result
}


#[cfg(test)]
mod tests {
    use utils;
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = utils::lines_from_file("src/test.in");
        let result = part1(&test_input);
        assert_eq!(result, 161);
    }

    #[test]
    fn part2_works() {
        let test_input = utils::lines_from_file("src/test.in");
        let result = part2(&test_input);
        assert_eq!(result, 48);
    }

    #[test]
    fn find_all_matches_in_row_regex_works() {
        let row = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let matches = find_all_matches_in_row_regex(row);
        assert_eq!(matches, vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]);
    }

    #[test]
    fn calculate_value_of_mul_works() {
        let mul = "mul(2,4)";
        let result = calculate_value_of_mul(mul);
        assert_eq!(result, 8);
    }

    #[test]
    fn calculate_all_multiplications_in_row_works() {
        let row = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = calculate_all_multiplications_in_row(row);
        assert_eq!(result, 161);
    }

}
