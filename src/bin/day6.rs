use std::collections::HashSet;

fn main() {
    let input = include_str!("./day6_input.txt");

    let part1 = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|line| {
            line.windows(4)
                .enumerate()
                .map(|(index, window)| {
                    let mut code: HashSet<&char> = HashSet::new();
                    window.iter().for_each(|c| {
                        code.insert(c);
                    });
                    if code.len() == 4 {
                        return Some(index + 4);
                    }
                    return None;
                })
                .filter(|window| window.is_some())
                .map(|window| window.unwrap())
                .next()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let part2 = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|line| {
            line.windows(14)
                .enumerate()
                .map(|(index, window)| {
                    let mut code: HashSet<&char> = HashSet::new();
                    window.iter().for_each(|c| {
                        code.insert(c);
                    });
                    if code.len() == 14 {
                        return Some(index + 14);
                    }
                    return None;
                })
                .filter(|window| window.is_some())
                .map(|window| window.unwrap())
                .next()
                .unwrap()
        })
        .collect::<Vec<_>>();

    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
}
