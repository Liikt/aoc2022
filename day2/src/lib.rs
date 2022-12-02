#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissor
}

impl From<&str> for Choice {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,

            _ => panic!("Invalid Choice: {}", value)
        }
    }
}

impl Choice {
    fn score(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3
        }
    }
}

impl Choice {
    fn play(self, other: Self) -> Outcome {
        match self {
            Self::Rock => {
                match other {
                    Self::Rock => Outcome::Draw,
                    Self::Paper => Outcome::Lost,
                    Self::Scissor => Outcome::Won
                }
            }
            Self::Paper => {
                match other {
                    Self::Rock => Outcome::Won,
                    Self::Paper => Outcome::Draw,
                    Self::Scissor => Outcome::Lost
                }
            }
            Self::Scissor => {
                match other {
                    Self::Rock => Outcome::Lost,
                    Self::Paper => Outcome::Won,
                    Self::Scissor => Outcome::Draw
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Won,
    Draw,
    Lost
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Self::Won => 6,
            Self::Draw => 3,
            Self::Lost => 0
        }
    }

    fn get_choice(self, other: Choice) -> Choice {
        match self {
            Self::Lost => {
                match other {
                    Choice::Rock => Choice::Scissor,
                    Choice::Paper => Choice::Rock,
                    Choice::Scissor => Choice::Paper
                }
            }
            Self::Draw => {
                match other {
                    Choice::Rock => Choice::Rock,
                    Choice::Paper => Choice::Paper,
                    Choice::Scissor => Choice::Scissor
                }
            }
            Self::Won => {
                match other {
                    Choice::Rock => Choice::Paper,
                    Choice::Paper => Choice::Scissor,
                    Choice::Scissor => Choice::Rock
                }
            }
        }
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Lost,
            "Y" => Self::Draw,
            "Z" => Self::Won,

            _ => panic!("Invalid Outcome: {}", value)
        }
    }
}

fn process1(input: String) -> Vec<(Choice, Choice)> {
    input.split("\n")
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .map(|x| (Choice::from(x[0]), Choice::from(x[1])))
        .collect()
}

fn process2(input: String) -> Vec<(Choice, Outcome)> {
    input.split("\n")
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .map(|x| (Choice::from(x[0]), Outcome::from(x[1])))
        .collect()
}

pub fn solve1(input: String) -> usize {
    let inp = process1(input);

    let mut total = 0;
    for (elf, my) in inp {
        total += my.score();
        total += my.play(elf).score();
    }

    total
}

pub fn solve2(input: String) -> usize {
    let inp = process2(input);

    let mut total = 0;
    for (elf, outcome) in inp {
        total += outcome.score();
        total += outcome.get_choice(elf).score();
    }

    total
}