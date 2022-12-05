#[derive(Debug, Clone, Copy)]
struct Move {
    from: usize,
    to: usize,
    amount: usize
}

fn process(input: String) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut stacks = Vec::new();
    let mut moves = Vec::new();

    for line in input.split("\n") {
        if stacks.len() == 0 {
            stacks = vec![Vec::new(); (line.len() + 1)/4]
        }

        if line.contains("[") {
            for x in (1..line.len()).step_by(4) {
                let c = line.chars().nth(x).unwrap();
                if c == ' ' { continue; }
                stacks[x/4].reverse();
                stacks[x/4].push(c);
                stacks[x/4].reverse();
            }
        } else if line.contains("move") {
            let mut split = line.split(" ");
            let m = Move { 
                amount: split.nth(1).unwrap().parse().unwrap(),
                from: split.nth(1).unwrap().parse().unwrap(),
                to: split.nth(1).unwrap().parse().unwrap(),
            };
            moves.push(m);
        }
    }

    (stacks, moves)
}

pub fn solve1(input: String) -> String {
    let (mut stacks, moves) = process(input);
    let mut ret = String::new();

    for m in moves {
        for _ in 0..m.amount {
            let c = stacks[m.from-1].pop().unwrap();
            stacks[m.to-1].push(c);
        }
    }

    for mut s in stacks {
        ret.push(s.pop().unwrap());
    }
    ret
}

pub fn solve2(input: String) -> String {
    let (mut stacks, moves) = process(input);
    let mut ret = String::new();

    for m in moves {
        let len = stacks[m.from-1].len();
        let chars: Vec<char> = stacks[m.from-1].drain(len-m.amount..).collect();
        stacks[m.to-1].extend(chars);
    }

    for mut s in stacks {
        ret.push(s.pop().unwrap());
    }
    ret
}