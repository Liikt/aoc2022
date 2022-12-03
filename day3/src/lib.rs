use std::collections::HashSet;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn process(input: String) -> Vec<String> {
    input.split("\n").map(|x| x.to_string()).collect()
}

pub fn solve1(input: String) -> usize {
    let inp = process(input);

    let mut total = 0;
    for rucksack in inp {
        let mut found = HashSet::new();
        let first: HashSet<char> = HashSet::from_iter(
            rucksack.as_bytes()[..rucksack.len()/2].into_iter().map(|&x| x.into()));
        let second: Vec<char> = rucksack.as_bytes()[rucksack.len()/2..]
            .into_iter().map(|&x| x.into()).collect();
        for b in second {
            if first.contains(&b) && !found.contains(&b) {
                found.insert(b);
                total += ALPHA.chars().position(|x| x == b).unwrap() + 1;
            }
        }
    }

    total
}

pub fn solve2(input: String) -> usize {
    let inp = process(input);
    let mut idx = 0;
    let mut total = 0;

    loop {
        let one: HashSet<char> = HashSet::from_iter(
            inp[idx].as_bytes().into_iter().map(|&x| x.into()));
        let two: HashSet<char> = HashSet::from_iter(
            inp[idx+1].as_bytes().into_iter().map(|&x| x.into()));
        let thr: HashSet<char> = HashSet::from_iter(
            inp[idx+2].as_bytes().into_iter().map(|&x| x.into()));

        for x in one {
            if two.contains(&x) && thr.contains(&x) {
                total += ALPHA.chars().position(|y| x == y).unwrap() + 1;
            }
        }

        idx += 3;
        if idx == inp.len() { break; }
    }

    total
}