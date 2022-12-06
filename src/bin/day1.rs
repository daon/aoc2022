fn parse_calories(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>()
}

fn solve_part1(input: &str) -> usize {
    let calories = parse_calories(input);

    *calories.iter().max().unwrap()
}

fn solve_part2(input: &str) -> usize {
    let mut calories = parse_calories(input);

    calories.sort_by(|a, b| b.cmp(a));
    calories.iter().take(3).sum()
}

fn main() {
    let input = include_str!("./day1_input.txt");

    let part1 = solve_part1(input);

    let part2 = solve_part2(input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        let input = include_str!("./day1_test.txt");

        let result = solve_part1(input);

        assert_eq!(result, 24000);
    }

    #[test]
    fn part1_puzzle_input() {
        let input = include_str!("./day1_input.txt");

        let result = solve_part1(input);

        assert_eq!(result, 71300);
    }

    #[test]
    fn part2_test_input() {
        let input = include_str!("./day1_test.txt");

        let result = solve_part2(input);

        assert_eq!(result, 45000);
    }

    #[test]
    fn part2_puzzle_input() {
        let input = include_str!("./day1_input.txt");

        let result = solve_part2(input);

        assert_eq!(result, 209691);
    }
}
