pub fn part1() -> i64 {
    42
}

pub fn part2() -> i64 {
    84
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1();
        assert_eq!(result, 42);
    }

    #[test]
    fn part2_works() {
        let result = part2();
        assert_eq!(result, 84);
    }
}
