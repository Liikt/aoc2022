#[cfg(test)]
mod day6 {
    use day6::{solve1, solve2};

    const INPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
    ];

    #[test]
    fn part1() {
        assert_eq!(INPUTS.into_iter().map(|x| solve1(x.to_string()))
            .collect::<Vec<usize>>(), vec![7, 5, 6, 10, 11]);
    }

    #[test]
    fn part2() {
        assert_eq!(INPUTS.into_iter().map(|x| solve2(x.to_string()))
            .collect::<Vec<usize>>(), vec![19, 23, 23, 29, 26]);
    }
}
