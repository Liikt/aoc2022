#[cfg(test)]
mod day9 {
    use day9::{solve1, solve2};

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT.to_string()), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT.to_string()),   1);
        assert_eq!(solve2(INPUT2.to_string()), 36);
    }
}