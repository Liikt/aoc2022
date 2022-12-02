#[cfg(test)]
mod tests {
    use day2::{solve1, solve2};

    const INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT.to_string()), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT.to_string()), 12);
    }
}
