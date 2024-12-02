
pub fn part1(input: &Vec<String>) -> i64 {
    let processed_inputs = split_vec_of_strings_into_vec_of_ints(input);
    let mut n_safe_vecs: i64 = 0;
    for vec in processed_inputs {
        if vec_of_ints_is_safe(&vec) {
            n_safe_vecs += 1;
        }
    }
    n_safe_vecs
}

pub fn part2(input: &Vec<String>) -> i64 {
    let processed_inputs = split_vec_of_strings_into_vec_of_ints(input);
    let mut n_safe_vecs: i64 = 0;
    for vec in processed_inputs {
        if vec_of_ints_is_safe(&vec) {
            n_safe_vecs += 1;
        }
        else {
            for i in 0..vec.len() {
                let new_vec = drop_element_from_vec(&vec, i);
                if vec_of_ints_is_safe(&new_vec) {
                    n_safe_vecs += 1;
                    break;
                }
            }
        }
    }
    n_safe_vecs
}

fn split_vec_of_strings_into_vec_of_ints(input: &Vec<String>) -> Vec<Vec<i64>> {
    // input is a vector of strings (e.g., <"1 2 3", "2 -2 3", "3 1 2">)
    // we want to convert it into a vector of vectors of integers (e.g., <[1, 2, 3], [2, -2, 3], [3, 1, 2]>)
    let mut result = Vec::new();
    for line in input {
        let mut vec = Vec::new();
        for number in line.split_whitespace() {
            vec.push(number.parse::<i64>().unwrap());
        }
        result.push(vec);
    }
    result
}

fn vec_of_ints_is_safe(input: &Vec<i64>) -> bool {
    let diffs = calculate_diff_between_pairs(input);
    let min_diff = diffs.iter().min().unwrap();
    let max_diff = diffs.iter().max().unwrap();
    if *min_diff >= -3 && *max_diff <= -1 {
        return true
    }
    if *min_diff >= 1 && *max_diff <= 3 {
        return true
    }
    false
}

fn calculate_diff_between_pairs(input: &Vec<i64>) -> Vec<i64> {
    let mut diffs = Vec::new();
    for i in 0..input.len() - 1 {
        let diff = input[i + 1] - input[i];
        diffs.push(diff);
    }
    diffs
}

fn drop_element_from_vec(input: &Vec<i64>, index: usize) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        if i != index {
            result.push(input[i]);
        }
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
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let test_input = utils::lines_from_file("src/test.in");
        let result = part2(&test_input);
        assert_eq!(result, 4);
    }

    #[test]
    fn split_vec_of_strings_into_vec_of_ints_works() {
        let test_input = vec!["1 2 3".to_string(), "2 -2 3".to_string(), "3 1 2".to_string()];
        let result = split_vec_of_strings_into_vec_of_ints(&test_input);
        assert_eq!(result, vec![vec![1, 2, 3], vec![2, -2, 3], vec![3, 1, 2]]);
    }

    #[test]
    fn calculate_diff_between_pairs_works() {
        let test_input = vec![1, 3, 4, 2, 3, -3];
        let result = calculate_diff_between_pairs(&test_input);
        assert_eq!(result, vec![2, 1, -2, 1, -6]);
    }

    #[test]
    fn vec_of_ints_is_safe_works() {
        for (vec, is_safe) in vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], false),
            (vec![8, 6, 4, 4, 1], false),
            (vec![1, 3, 6, 7, 9], true),
        ] {
            let result = vec_of_ints_is_safe(&vec);
            assert_eq!(result, is_safe);
        }
    }

    #[test]
    fn drop_element_from_vec_works() {
        let test_input = vec![1, 2, 3, 4, 5];
        for (index, expected) in vec![
            (0, vec![2, 3, 4, 5]),
            (1, vec![1, 3, 4, 5]),
            (2, vec![1, 2, 4, 5]),
            (3, vec![1, 2, 3, 5]),
            (4, vec![1, 2, 3, 4]),
        ] {
            let result = drop_element_from_vec(&test_input, index);
            assert_eq!(result, expected);
        }
    }
}
