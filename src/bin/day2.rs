fn main() {
    let input = include_str!("./day2_input.txt");

    #[derive(Clone, Copy)]
    enum Move {
        Rock,
        Paper,
        Scissors,
        Unknown,
    }

    impl From<&str> for Move {
        fn from(word: &str) -> Self {
            match word {
                "A" | "X" => Self::Rock,
                "B" | "Y" => Self::Paper,
                "C" | "Z" => Self::Scissors,
                _ => Self::Unknown,
            }
        }
    }

    #[derive(Clone, Copy)]
    enum Strategy {
        Lose,
        Draw,
        Win,
        Unknown,
    }

    impl From<&str> for Strategy {
        fn from(word: &str) -> Self {
            match word {
                "X" => Self::Lose,
                "Y" => Self::Draw,
                "Z" => Self::Win,
                _ => Self::Unknown,
            }
        }
    }

    let rounds = input
        .split("\n")
        .map(|x| x.split(" ").filter(|x| !x.is_empty()).collect::<Vec<_>>())
        .filter(|x| !x.is_empty());

    let part1: i32 = rounds
        .clone()
        .map(|x| {
            let opponent_move = Move::from(x[0]);
            let player_move = Move::from(x[1]);

            let round_score = match (opponent_move, player_move) {
                (Move::Rock, Move::Rock)
                | (Move::Paper, Move::Paper)
                | (Move::Scissors, Move::Scissors) => 3,
                (Move::Rock, Move::Paper)
                | (Move::Paper, Move::Scissors)
                | (Move::Scissors, Move::Rock) => 6,
                (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper) => 0,
                (_, _) => 0,
            };

            let move_score = match player_move {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
                _ => 0,
            };

            round_score + move_score
        })
        .sum();

    let part2: i32 = rounds
        .clone()
        .map(|x| {
            let opponent_move = Move::from(x[0]);
            let strategy = Strategy::from(x[1]);

            let player_move = match (opponent_move, strategy) {
                (Move::Rock, Strategy::Lose)
                | (Move::Scissors, Strategy::Draw)
                | (Move::Paper, Strategy::Win) => Move::Scissors,
                (Move::Paper, Strategy::Lose)
                | (Move::Rock, Strategy::Draw)
                | (Move::Scissors, Strategy::Win) => Move::Rock,
                (Move::Scissors, Strategy::Lose)
                | (Move::Paper, Strategy::Draw)
                | (Move::Rock, Strategy::Win) => Move::Paper,
                (_, _) => Move::Unknown,
            };

            let round_score = match (opponent_move, player_move) {
                (Move::Rock, Move::Rock)
                | (Move::Paper, Move::Paper)
                | (Move::Scissors, Move::Scissors) => 3,
                (Move::Rock, Move::Paper)
                | (Move::Paper, Move::Scissors)
                | (Move::Scissors, Move::Rock) => 6,
                (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper) => 0,
                (_, _) => 0,
            };

            let move_score = match player_move {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3,
                _ => 0,
            };

            round_score + move_score
        })
        .sum();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
