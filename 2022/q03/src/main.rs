use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let mut part1_total: u32 = 0;
    let mut part2_total: u32 = 0;
    let mut count = 0;
    let mut group_rucksack: Vec<char> = Vec::new();

    for line in contents {
        let first_half = &line[..line.len() / 2];
        let second_half = &line[line.len() / 2..];
        let first_chars: HashSet<char> = first_half.chars().collect();

        for c in second_half.chars() {
            if first_chars.contains(&c) {
                part1_total += get_priority(&c);

                // Need to do this to avoid double counting
                break;
            }
        }

        // Part 2 Stuff
        count += 1;
        group_rucksack.append(
            &mut line
                .chars()
                .collect::<HashSet<char>>() // convert to hashset first to remove duplicates
                .into_iter()
                .collect(),
        );

        if count == 3 {
            // This is inefficient but eh... O(n^2) go brrr
            let counts = group_rucksack.into_iter().counts();

            for (key, val) in counts.iter() {
                if *val == 3 {
                    part2_total += get_priority(key);
                }
            }

            group_rucksack = Vec::new();
            count = 0;
        }
    }

    println!("Part 1: {part1_total}");
    println!("Part 2: {part2_total}")
}

fn get_priority(c: &char) -> u32 {
    if *c >= 'A' && *c <= 'Z' {
        *c as u32 - 'A' as u32 + 27
    } else {
        *c as u32 - 'a' as u32 + 1
    }
}
