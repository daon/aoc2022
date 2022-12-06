use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;

fn parse_stacks(stacks_input: &str) -> Vec<VecDeque<char>> {
    let mut stack_lookup: HashMap<usize, usize> = HashMap::new();
    let mut stacks: Vec<VecDeque<char>> = vec![];

    stacks_input.lines().rev().for_each(|chunks| {
        chunks.char_indices().for_each(|(i, c)| {
            if c != ' ' && c != '[' && c != ']' {
                if !stack_lookup.contains_key(&i) {
                    stacks.push(VecDeque::new());
                    stack_lookup.insert(i, c.to_string().parse::<usize>().unwrap() - 1);
                } else {
                    let stack_index = stack_lookup.get(&i).unwrap();
                    stacks[*stack_index].push_back(c);
                }
            }
        });
    });

    return stacks;
}

fn run_procedure(stacks: &mut Vec<VecDeque<char>>, procedure_input: &str, part2: bool) -> String {
    let re = Regex::new(r"move\s(\d*)\sfrom\s(\d*)\sto\s(\d*)").unwrap();
    procedure_input.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let n = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        let pos = stacks[to - 1].len();
        for _i in 0..n {
            let c = stacks[from - 1].pop_back().unwrap();
            if part2 == true {
                stacks[to - 1].insert(pos, c)
            } else {
                stacks[to - 1].push_back(c)
            }
        }
    });

    stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect::<String>()
}

fn main() {
    let input = include_str!("./day5_input.txt");

    let (stacks_input, procedure_input) = input.split_once("\n\n").expect("aoc to be good");

    let mut stacks = parse_stacks(stacks_input);
    let part1 = run_procedure(&mut stacks, procedure_input, false);

    let mut stacks = parse_stacks(stacks_input);
    let part2 = run_procedure(&mut stacks, procedure_input, true);

    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
}
