fn main() {
    let input = include_str!("./day4_input.txt");

    let part1 = input
        .lines()
        .map(|line| {
            let (elf1, elf2) = line.split_once(",").unwrap();
            let (elf1_section1, elf1_section2) = elf1.split_once("-").unwrap();
            let (elf2_section1, elf2_section2) = elf2.split_once("-").unwrap();

            let elf1_section1 = elf1_section1.parse::<u32>().unwrap();
            let elf1_section2 = elf1_section2.parse::<u32>().unwrap();
            let elf2_section1 = elf2_section1.parse::<u32>().unwrap();
            let elf2_section2 = elf2_section2.parse::<u32>().unwrap();

            if elf1_section1 <= elf2_section1 && elf1_section2 >= elf2_section2
                || elf1_section1 >= elf2_section1 && elf1_section2 <= elf2_section2
            {
                return 1;
            }

            return 0;
        })
        .sum::<u32>();

    let part2 = input
        .lines()
        .map(|line| {
            let (elf1, elf2) = line.split_once(",").unwrap();
            let (elf1_section1, elf1_section2) = elf1.split_once("-").unwrap();
            let (elf2_section1, elf2_section2) = elf2.split_once("-").unwrap();

            let elf1_section1 = elf1_section1.parse::<u32>().unwrap();
            let elf1_section2 = elf1_section2.parse::<u32>().unwrap();
            let elf2_section1 = elf2_section1.parse::<u32>().unwrap();
            let elf2_section2 = elf2_section2.parse::<u32>().unwrap();

            if elf1_section1 >= elf2_section1 && elf1_section1 <= elf2_section1
                || elf1_section2 >= elf2_section1 && elf1_section1 <= elf2_section2
            {
                println!("{}", line);
                return 1;
            }

            return 0;
        })
        .sum::<u32>();

    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
}
