#[cfg(test)]
mod tests {
    use day1::{solve1, solve2};

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT.to_string()), 24000);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT.to_string()), 45000);
    }
}
