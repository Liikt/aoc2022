enum Instruction {
    Nop,
    Addx(isize)
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut split = value.split(" ");
        match split.next().unwrap() {
            "noop" => Self::Nop,
            "addx" => 
                Self::Addx(split.next().unwrap().parse::<isize>().unwrap()),
            _ => unreachable!()
        }
    }
}

fn process(input: String) -> Vec<Instruction> {
    input.split("\n").map(|x| Instruction::from(x)).collect()
}

macro_rules! advance_cycle {
    ($cycle:expr, $milestone:expr, $x:expr, $total:expr) => {
        {
            $cycle += 1;
            if $cycle == $milestone * 40 + 20 { 
                $milestone += 1; 
                $total += $cycle * $x;
            }
        }
    };

    ($cycle:expr, $sprite:expr, $ret:expr) => {
        {
            if $sprite.hit($cycle) {
                $ret.push('#');
            } else {
                $ret.push('.');
            }
            $cycle += 1;
            if $cycle % 40 == 0 {
                $ret.push('\n');
            }
        }
    };
}

pub fn solve1(input: String) -> isize {
    let insns = process(input);
    let mut total = 0;
    let mut cycle = 1;
    let mut x = 1;
    let mut milestone = 0;

    for insn in insns {
        match insn {
            Instruction::Nop => advance_cycle!(cycle, milestone, x, total),
            Instruction::Addx(val) => {
                advance_cycle!(cycle, milestone, x, total);
                x += val;
                advance_cycle!(cycle, milestone, x, total);
            }
        }
    }

    total
}

struct Sprite(isize);

impl Sprite {
    fn hit(&self, x: isize) -> bool {
        let lower = (self.0-1).max(0);
        let upper = (self.0+1).min(39);
        (lower..=upper).contains(&(x % 40))
    }
}

pub fn solve2(input: String) -> String {
    let insns = process(input);
    let mut cycle = 0;
    let mut sprite = Sprite(1);
    let mut ret = String::from("\n");

    for insn in insns {
        match insn {
            Instruction::Nop => advance_cycle!(cycle, sprite, ret),
            Instruction::Addx(val) => {
                advance_cycle!(cycle, sprite, ret);
                advance_cycle!(cycle, sprite, ret);
                sprite.0 += val;
            }
        }
    }

    let ret = ret.trim_end();
    ret.to_owned()
}