fn main() {
    let input = include_str!("./day1_input.txt");

    let part1 = input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    let mut part2 = input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    part2.sort_by(|a, b| b.cmp(a));
    let part2 = part2[..3].iter().sum::<i32>();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
