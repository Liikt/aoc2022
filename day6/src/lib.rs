use std::collections::HashSet;

pub fn solve1(input: String) -> usize {
    let mut ctr = 4;
    loop {
        let window = &input[ctr-4..ctr];
        if HashSet::<&u8>::from_iter(window.as_bytes()).len() == 4 {
            return ctr;
        }
        ctr += 1;
    }
}

pub fn solve2(input: String) -> usize {
    let mut ctr = 14;
    loop {
        let window = &input[ctr-14..ctr];
        if HashSet::<&u8>::from_iter(window.as_bytes()).len() == 14 {
            return ctr;
        }
        ctr += 1;
    }
}