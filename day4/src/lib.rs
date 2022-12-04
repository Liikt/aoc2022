#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize
}

impl Range {
    fn includes(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end) || 
        (other.start >= self.start && other.start <= self.end)
    }
}

fn process(input: String) -> Vec<(Range, Range)> {
    input.trim().split("\n").map(|x| {
        let first  = x.split(",").nth(0).unwrap();
        let second = x.split(",").nth(1).unwrap();
        let first_start = 
            first.split("-").nth(0).unwrap().parse::<usize>().unwrap();
        let first_end = 
            first.split("-").nth(1).unwrap().parse::<usize>().unwrap();
        let second_start = 
            second.split("-").nth(0).unwrap().parse::<usize>().unwrap();
        let second_end = 
            second.split("-").nth(1).unwrap().parse::<usize>().unwrap();
        let first = Range{start: first_start, end: first_end};
        let second = Range{start: second_start, end: second_end};
        (first, second)
    }).collect()
}

pub fn solve1(input: String) -> usize {
    process(input).into_iter().filter(|(f, s)| f.includes(s) || s.includes(f))
        .count()
}

pub fn solve2(input: String) -> usize {
    process(input).into_iter().filter(|(f, s)| f.overlaps(s)).count()
}