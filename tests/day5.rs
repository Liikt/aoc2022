#[cfg(test)]
mod day5 {
    use day5::{solve1, solve2};

    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT.to_string()), "CMZ");
    }

    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT.to_string()), "MCD");
    }
}
