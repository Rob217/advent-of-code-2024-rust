use regex::Regex;

pub fn part1(input: &Vec<String>) -> i64 {
    let mut result = 0;
    for row in input {
        let mul_matches = find_all_matches_in_row_regex(row);
        result += calculate_all_multiplications_in_row(&mul_matches);
    }
    result
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut result = 0;
    let mut enable_mul = true;
    for row in input {
        let mul_matches = find_all_matches_in_row_mul_do_dont(row);
        let valid_matches = find_all_valid_multiplications_in_row_do_dont(&mul_matches, &mut enable_mul);
        result += calculate_all_multiplications_in_row(&valid_matches);
    }
    result
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

fn calculate_all_multiplications_in_row(matches: &Vec<&str>) -> i64 {
    let mut result = 0;
    for m in matches {
        result += calculate_value_of_mul(m);
    }
    result
}

fn find_all_matches_in_row_mul_do_dont(row: &str) -> Vec<&str> {
    // find every instance of mul(X,Y) where X and Y are 1-3 digit numbers and "do()" and "don't()"
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let matches = re.find_iter(row);
    let mut result = Vec::new();
    for m in matches {
        result.push(m.as_str());
    }
    result
}

fn find_all_valid_multiplications_in_row_do_dont<'a>(matches: &'a Vec<&'a str>, do_mul: &mut bool) -> Vec<&'a str> {
    let mut valid_matches = Vec::new();
    for m in matches {
        if *m == "do()" {
            *do_mul = true;
        } else if *m == "don't()" {
            *do_mul = false;
        } else if *do_mul {
            valid_matches.push(*m);
        }
    }
    valid_matches
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
        let test_input = utils::lines_from_file("src/test2.in");
        let result = part2(&test_input);
        assert_eq!(result, 48);
        assert_eq!(part2(&vec!["mul(1000,2)".to_string()]), 0);
        assert_eq!(part2(&vec!["mul(1,2)".to_string(), "mul(3,4)".to_string()]), 14);
        assert_eq!(part2(&vec!["don't()mul(1,2)".to_string()]), 0);
        assert_eq!(part2(&vec!["domul(1,2)".to_string()]), 2);
        assert_eq!(part2(&vec!["mul(3,4)don't()mul(1,2)".to_string()]), 12);
        assert_eq!(part2(&vec!["mul(3,4)do()mul(1,2)".to_string()]), 14);
        assert_eq!(part2(&vec!["mul(3,4)do()mul(1,2)don't()mul(5,6)".to_string()]), 14);
        assert_eq!(part2(&vec!["don't()".to_string(), "mul(1,2)".to_string()]), 0);
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
        let row = vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"];
        let result = calculate_all_multiplications_in_row(&row);
        assert_eq!(result, 161);
    }

    #[test]
    fn find_all_matches_in_row_mul_do_dont_works() {
        let row = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let matches = find_all_matches_in_row_mul_do_dont(row);
        assert_eq!(matches, vec!["mul(2,4)", "don't()", "mul(5,5)", "mul(11,8)", "do()", "mul(8,5)"]);
    }

    #[test]
    fn find_all_valid_multiplications_in_row_do_dont_works() {
        let row = vec!["mul(2,4)", "don't()", "mul(5,5)", "mul(11,8)", "do()", "mul(8,5)"];
        assert_eq!(find_all_valid_multiplications_in_row_do_dont(&row, &mut true), vec!["mul(2,4)", "mul(8,5)"]);
        assert_eq!(find_all_valid_multiplications_in_row_do_dont(&row, &mut false), vec!["mul(8,5)"]);
    }
}
