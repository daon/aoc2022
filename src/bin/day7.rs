fn parse_dirs(input: &str) -> Vec<(&str, usize)> {
    let mut stack = vec![("/", 0)];
    let mut dirs = vec![];

    for line in input.lines().filter(|line| !line.is_empty()) {
        if line == "$ cd /" || line == "$ ls" || line.starts_with("dir") {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (name, size) = stack.pop().unwrap();
                stack.last_mut().unwrap().1 += size;
                dirs.push((name, size));
            } else {
                stack.push((dir, 0))
            }
            continue;
        }

        let (size, _) = line.split_once(" ").unwrap();

        if let Ok(size) = size.parse::<usize>() {
            stack.last_mut().unwrap().1 += size;
        }
    }

    while !stack.is_empty() {
        let (name, size) = stack.pop().unwrap();
        dirs.push((name, size));

        if !stack.is_empty() {
            stack.last_mut().unwrap().1 += size;
        }
    }

    return dirs;
}

fn solve_part1(input: &str) -> usize {
    let max_size = 100_000;

    let dirs = parse_dirs(input);
    dirs.into_iter()
        .filter(|(_, size)| size <= &max_size)
        .map(|(_, size)| size)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let total_space = 70_000_000;
    let required_unused_space = 30_000_000;

    let dirs = parse_dirs(input);
    let root_size = dirs.last().unwrap().1;
    let free_space = total_space - root_size;
    let required_space_to_delete = required_unused_space - free_space;
    dirs.into_iter()
        .filter(|(_, size)| size >= &required_space_to_delete)
        .map(|(_, size)| size)
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("./day7_input.txt");

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
        let input = include_str!("./day7_test.txt");

        let result = solve_part1(input);

        assert_eq!(result, 95437);
    }

    #[test]
    fn part1_puzzle_input() {
        let input = include_str!("./day7_input.txt");

        let result = solve_part1(input);

        assert_eq!(result, 1390824);
    }

    #[test]
    fn part2_test_input() {
        let input = include_str!("./day7_test.txt");

        let result = solve_part2(input);

        assert_eq!(result, 24933642);
    }

    #[test]
    fn part2_puzzle_input() {
        let input = include_str!("./day7_input.txt");

        let result = solve_part2(input);

        assert_eq!(result, 7490863);
    }
}
