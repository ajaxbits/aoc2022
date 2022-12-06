#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win(u32),
    Loss(u32),
    Draw(u32),
}

impl From<Move> for u32 {
    fn from(play: Move) -> Self {
        match play {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Move {
    fn new(s: &str) -> Move {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("help"),
        }
    }

    fn beats(self, other: Move) -> bool {
        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => true,
            _ => false,
        }
    }
}

fn tally_score(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Loss(s) => s,
        Outcome::Draw(s) => s + 3,
        Outcome::Win(s) => s + 6,
    }
}

fn play_game(you: Move, opp: Move) -> Outcome {
    if you.beats(opp) {
        Outcome::Win(you.into())
    } else if opp.beats(you) {
        Outcome::Loss(you.into())
    } else {
        Outcome::Draw(you.into())
    }
}

fn find_score(input_str: &str) -> u32 {
    let score = input_str.lines().collect::<Vec<_>>();
    let score = score
        .into_iter()
        .map(|l| {
            let s = dbg!(l.split(' ').map(|s| Move::new(s)).collect::<Vec<_>>());
            dbg!(play_game(s[0], s[1]))
        })
        .map(tally_score)
        .collect::<Vec<_>>();
    let mut total_score = 0;
    score.into_iter().for_each(|score| total_score += score);
    total_score
}

fn main() -> jane_eyre::Result<()> {
    jane_eyre::install()?;
    let score = find_score(include_str!("./input.txt"));

    println!("The strategy would yield a score of {score}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided_example() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(15, find_score(input))
    }
}
