
pub fn part1(input: &Vec<String>) -> i64 {
    let mut result = 0;
    for i_row in 0..input.len() {
        for i_col in 0..input[i_row].len() {
            if get_char(&input, i_row as i64, i_col as i64) == 'X' {
                for row_offset in -1..2 {
                    for col_offset in -1..2 {
                        if row_offset == 0 && col_offset == 0 {
                            continue;
                        }
                        if
                            get_char(&input, i_row as i64 + row_offset, i_col as i64 + col_offset) == 'M' &&
                            get_char(&input, i_row as i64 + row_offset * 2, i_col as i64 + col_offset * 2) == 'A' &&
                            get_char(&input, i_row as i64 + row_offset * 3, i_col as i64 + col_offset * 3) == 'S'
                        {
                            result += 1;
                        }
                    }
                }
            }
        }
    }
    result
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut result = 0;
    let mut output = &mut Vec::new();
    for i_row in 0..input.len() {
        let mut row = String::new();
        for i_col in 0..input[i_row].len() {
            row.push('.');
        }
        output.push(row);
    }
    for i_row in 1..(input.len() - 1) {
        for i_col in 1..(input[i_row].len() - 1) {
            if get_char(&input, i_row as i64, i_col as i64) == 'A' {
                let diagonals = vec![
                    get_char(&input, i_row as i64 - 1, i_col as i64 - 1),
                    get_char(&input, i_row as i64 + 1, i_col as i64 + 1),
                    get_char(&input, i_row as i64 - 1, i_col as i64 + 1),
                    get_char(&input, i_row as i64 + 1, i_col as i64 - 1)
                ];
                // check if there are 2 M's and 2 S's in the diagonals
                if
                    diagonals.iter().filter(|&c| *c == 'M').count() == 2 &&
                    diagonals.iter().filter(|&c| *c == 'S').count() == 2 &&
                    diagonals.iter().nth(0).unwrap() != diagonals.iter().nth(1).unwrap()
                {
                    result += 1;
                    // overwrite output[i_row, i_col] with 'A'
                    output[i_row as usize].replace_range(i_col as usize..i_col as usize + 1, "A");
                }
            }
        }
    }
    // display A locations
    // for row in output {
    //     println!("{:?}", row);
    // }
    result
}

fn get_char(input: &Vec<String>, row: i64, col: i64) -> char {
    if row < 0 || row >= input.len() as i64 {
        return '.';
    }
    if col < 0 || col >= input[row as usize].len() as i64 {
        return '.';
    }
    input[row as usize].chars().nth(col as usize).unwrap()
}


#[cfg(test)]
mod tests {
    use utils;
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = utils::lines_from_file("src/test.in");
        let result = part1(&test_input);
        assert_eq!(result, 18);
    }

    #[test]
    fn part2_works() {
        let test_input = utils::lines_from_file("src/test.in");
        let result = part2(&test_input);
        assert_eq!(result, 9);
        assert_eq!(part2(&vec![
            String::from("MAM"),
            String::from("XAX"),
            String::from("SMS"),
        ]), 1);
        assert_eq!(part2(&vec![
            String::from("MAM"),
            String::from("SAS"),
            String::from("SMX"),
        ]), 0);
        assert_eq!(part2(&vec![
            String::from("MAS"),
            String::from("SAS"),
            String::from("SMM"),
        ]), 0);
    }


    #[test]
    fn get_char_works() {
        let test_input = vec![
            String::from("123"),
            String::from("456"),
            String::from("789"),
        ];
        assert_eq!(get_char(&test_input, 0, 0), '1');
        assert_eq!(get_char(&test_input, 0, 1), '2');
        assert_eq!(get_char(&test_input, 0, 2), '3');
        assert_eq!(get_char(&test_input, 1, 0), '4');
        assert_eq!(get_char(&test_input, 1, 1), '5');
        assert_eq!(get_char(&test_input, 1, 2), '6');
        assert_eq!(get_char(&test_input, 2, 0), '7');
        assert_eq!(get_char(&test_input, 2, 1), '8');
        assert_eq!(get_char(&test_input, 2, 2), '9');
        assert_eq!(get_char(&test_input, 3, 0), '.');
        assert_eq!(get_char(&test_input, 0, 3), '.');
        assert_eq!(get_char(&test_input, -1, 0), '.');
        assert_eq!(get_char(&test_input, 0, -1), '.');
    }
}
