#[cfg(test)]
mod day3 {
    use day3::{solve1, solve2};

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT.to_string()), 157);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT.to_string()), 70);
    }
}
