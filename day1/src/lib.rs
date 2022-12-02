fn process(input: String) -> Vec<usize> {
    input.split("\n\n")
        .map(|x| x.split("\n").map(|y| y.parse::<usize>().unwrap()).sum())
        .collect()
}

pub fn solve1(input: String) -> usize { 
    process(input).into_iter().max().unwrap()
}

pub fn solve2(input: String) -> usize { 
    let mut sorted = process(input);
    sorted.sort_unstable();
    sorted[sorted.len()-3..].into_iter().sum()
}