
pub fn part1(input: &Vec<String>) -> i64 {
    let (mut first, mut second) = convert_vec_of_strings_into_two_vecs_of_integers(input);
    first.sort();
    second.sort();
    sum_differences_between_two_vectors_row_by_row(&first, &second)
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (first, second) = convert_vec_of_strings_into_two_vecs_of_integers(input);
    let second_counts = count_occurrences_in_vec_of_ints(&second);
    let similarity_score = calculate_similarity_score(&first, &second_counts);
    similarity_score
}

fn split_string_into_two_integers(s: &str) -> (i64, i64) {
    let mut parts = s.split_whitespace();
    let first = parts.next().unwrap().parse::<i64>().unwrap();
    let second = parts.next().unwrap().parse::<i64>().unwrap();
    (first, second)
}

fn convert_vec_of_strings_into_two_vecs_of_integers(input: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for line in input {
        let (f, s) = split_string_into_two_integers(line);
        first.push(f);
        second.push(s);
    }
    (first, second)
}

fn sum_differences_between_two_vectors_row_by_row(first: &Vec<i64>, second: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for (f, s) in first.iter().zip(second.iter()) {
        sum += (f - s).abs();
    }
    sum
}

fn count_occurrences_in_vec_of_ints(input_vec: &Vec<i64>) -> std::collections::HashMap<i64, i64> {
    let mut counts = std::collections::HashMap::new();
    for elem in input_vec {
        let count = counts.entry(*elem).or_insert(0);
        *count += 1;
    }
    counts
}

fn calculate_similarity_score(first: &Vec<i64>, occurences: &std::collections::HashMap<i64, i64>) -> i64 {
    let mut score = 0;
    for elem in first {
        let update_to_score = multiply_value_by_count_in_occurrences_hashmap(*elem, occurences);
        // println!("score = {}, elem = {}, update = {}", &score, &elem, &update_to_score);
        score += update_to_score;
    }
    score
}

fn multiply_value_by_count_in_occurrences_hashmap(value: i64, occurrences: &std::collections::HashMap<i64, i64>) -> i64 {
    let count = occurrences.get(&value).unwrap_or(&0);
    value * count
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils;
    use std::collections::HashMap;

    #[test]
    fn part1_works() {
        let test_input = vec!["4 2".to_string(), "1 5".to_string(), "3 3".to_string()];
        // sorted: [1, 3, 4], [2, 3, 5]
        // differences: 1 + 0 + 1 = 2
        let result = part1(&test_input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let test_input = utils::lines_from_file("src/test.in");
        let result = part2(&test_input);
        assert_eq!(result, 31);
    }

    #[test]
    fn split_string_into_two_integers_works() {
        let test_input = "42 84";
        let result = split_string_into_two_integers(test_input);
        assert_eq!(result, (42, 84));
    }

    #[test]
    fn convert_vec_of_strings_into_two_vecs_of_integers_works() {
        let test_input = vec!["42 84".to_string(), "21 42".to_string()];
        let (first, second) = convert_vec_of_strings_into_two_vecs_of_integers(&test_input);
        assert_eq!(first, vec![42, 21]);
        assert_eq!(second, vec![84, 42]);
    }

    #[test]
    fn sum_differences_between_two_vectors_row_by_row_works() {
        let first = vec![4, 1, 3];
        let second = vec![2, 5, 3];
        let result = sum_differences_between_two_vectors_row_by_row(&first, &second);
        assert_eq!(result, 6);
    }

    #[test]
    fn count_occurrences_in_vec_of_ints_works() {
        let test_input = vec![1, 2, 3, 2, 1, 3, 1, 2, 1];
        let result = count_occurrences_in_vec_of_ints(&test_input);
        let mut expected = HashMap::new();
        expected.insert(1, 4);
        expected.insert(2, 3);
        expected.insert(3, 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn multiply_value_by_count_in_occurrences_hashmap_works() {
        let mut occurrences = std::collections::HashMap::new();
        occurrences.insert(3, 2);
        occurrences.insert(4, 1);
        occurrences.insert(5, 1);
        occurrences.insert(9, 3);
        assert_eq!(multiply_value_by_count_in_occurrences_hashmap(2, &occurrences), 0);
        assert_eq!(multiply_value_by_count_in_occurrences_hashmap(3, &occurrences), 6);
        assert_eq!(multiply_value_by_count_in_occurrences_hashmap(4, &occurrences), 4);
        assert_eq!(multiply_value_by_count_in_occurrences_hashmap(5, &occurrences), 5);
        assert_eq!(multiply_value_by_count_in_occurrences_hashmap(6, &occurrences), 0);
        assert_eq!(multiply_value_by_count_in_occurrences_hashmap(9, &occurrences), 27);
    }

    #[test]
    fn calculate_similarity_score_works() {
        let first = vec![3, 4, 2, 1, 3, 3];
        let mut occurrences = std::collections::HashMap::new();
        occurrences.insert(3, 3);
        occurrences.insert(4, 1);
        occurrences.insert(5, 1);
        occurrences.insert(9, 1);
        let result = calculate_similarity_score(&first, &occurrences);
        assert_eq!(result, 31);
    }
}
