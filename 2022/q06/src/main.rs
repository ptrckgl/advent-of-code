use std::collections::{HashSet, VecDeque};

const _PART1_LENGTH: usize = 4;
const _PART2_LENGTH: usize = 14;

fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let mut curr_four: VecDeque<char> = VecDeque::new();
    let mut count = 0;

    for c in contents.get(0).unwrap().chars() {
        count += 1;
        curr_four.push_back(c);
        if curr_four.len() == _PART2_LENGTH {
            if curr_four.iter().cloned().collect::<HashSet<char>>().len() == _PART2_LENGTH {
                break;
            }
            curr_four.pop_front();
        }
    }

    println!("Part ?: {count}");
}
