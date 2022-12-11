fn solve_part1(input: &str) -> usize {
    let forest = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = forest.len();
    let width = forest[0].len();
    let mut seen = vec![vec![0isize; width]; height];

    for y in 0..height {
        let mut left_height = -1;
        let mut right_height = -1;

        for x in 0..width {
            let from_left = forest[y][x];
            let from_right = forest[y][width - x - 1];

            if from_left > left_height {
                seen[y][x] += 1;
                left_height = from_left;
            }

            if from_right > right_height {
                seen[y][width - x - 1] += 1;
                right_height = from_right;
            }
        }
    }

    for x in 0..width {
        let mut top_height = -1;
        let mut bottom_heigth = -1;

        for y in 0..height {
            let from_top = forest[y][x];
            let from_bottom = forest[height - y - 1][x];

            if from_top > top_height {
                seen[y][x] += 1;
                top_height = from_top;
            }

            if from_bottom > bottom_heigth {
                seen[height - y - 1][x] += 1;
                bottom_heigth = from_bottom;
            }
        }
    }

    println!("{:?}", seen);
    return seen
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x != 0)
        .count();
}

fn calculate_scenic_score(forest: &Vec<Vec<isize>>, x: usize, y: usize) -> usize {
    let tree = forest[y][x];
    let height = forest.len();
    let width = forest[0].len();
    let mut viewing_distance_left = 0;
    let mut viewing_distance_up = 0;
    let mut viewing_distance_right = 0;
    let mut viewing_distance_down = 0;

    for y in (0..y).rev() {
        viewing_distance_up += 1;
        if tree <= forest[y][x] {
            break;
        }
    }

    for x in (0..x).rev() {
        viewing_distance_left += 1;
        if tree <= forest[y][x] {
            break;
        }
    }

    for x in x + 1..width {
        viewing_distance_right += 1;
        if tree <= forest[y][x] {
            break;
        }
    }

    for y in y + 1..height {
        viewing_distance_down += 1;
        if tree <= forest[y][x] {
            break;
        }
    }

    viewing_distance_left * viewing_distance_up * viewing_distance_right * viewing_distance_down
}

fn solve_part2(input: &str) -> usize {
    let forest = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = forest.len();
    let width = forest[0].len();
    let mut scenic_scores = vec![];

    for y in 0..height {
        for x in 0..width {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                continue;
            }

            scenic_scores.push(calculate_scenic_score(&forest, x, y));
        }
    }

    return scenic_scores.into_iter().max().unwrap();
}

fn main() {
    let input = include_str!("./day8_input.txt");

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
        let input = include_str!("./day8_test.txt");

        let result = solve_part1(input);

        assert_eq!(result, 21);
    }

    #[test]
    fn part1_puzzle_input() {
        let input = include_str!("./day8_input.txt");

        let result = solve_part1(input);

        assert_eq!(result, 1681);
    }

    #[test]
    fn part2_test_input() {
        let input = include_str!("./day8_test.txt");

        let result = solve_part2(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_puzzle_input() {
        let input = include_str!("./day8_input.txt");

        let result = solve_part2(input);

        assert_eq!(result, 201684);
    }
}

