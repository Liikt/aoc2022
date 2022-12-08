#[cfg(test)]
mod day8 {
    use day8::{solve1, solve2};

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT.to_string()), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT.to_string()), 8);
    }
}