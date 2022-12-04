#[cfg(test)]
mod day4 {
    use day4::{solve1, solve2};

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT.to_string()), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT.to_string()), 4);
    }
}
