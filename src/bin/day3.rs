use std::{collections::HashSet, ops::BitAnd};

fn get_priority(letter: char) -> u32 {
    let char_value = u32::from(letter);
    let a_value = u32::from('a');
    let z_value = u32::from('z');
    let capital_a_value = u32::from('A');
    let capital_z_value = u32::from('Z');

    if char_value >= a_value && char_value <= z_value {
        return char_value - a_value + 1;
    }

    if char_value >= capital_a_value && char_value <= capital_z_value {
        return char_value - capital_a_value + 27;
    }

    return 0;
}

fn main() {
    let input = include_str!("./day3_input.txt");

    let part1: u32 = input
        .lines()
        .flat_map(|rucksack| {
            let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);
            let compartment1 = compartment1.chars().collect::<HashSet<char>>();
            let compartment2 = compartment2.chars().collect::<HashSet<char>>();

            let item_types = compartment1.bitand(&compartment2);

            item_types
                .into_iter()
                .map(|x| get_priority(x))
                .collect::<Vec<u32>>()
        })
        .sum();

    let part2: u32 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .flat_map(|groups| {
            let elf1 = groups[0].chars().collect::<HashSet<char>>();
            let elf2 = groups[1].chars().collect::<HashSet<char>>();
            let elf3 = groups[2].chars().collect::<HashSet<char>>();

            let badges = elf1.bitand(&elf2).bitand(&elf3);

            badges
                .into_iter()
                .map(|x| get_priority(x))
                .collect::<Vec<u32>>()
        })
        .sum();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
