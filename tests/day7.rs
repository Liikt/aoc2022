#[cfg(test)]
mod day7 {
    use day7::{solve1, solve2};

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn part1() {
        assert_eq!(solve1(INPUT.to_string()), 95437);
    }
    
    #[test]
    fn part2() {
        assert_eq!(solve2(INPUT.to_string()), 24933642);
    }
}
