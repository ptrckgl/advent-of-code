use std::str::FromStr;

fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let num_stacks = contents.get(0).unwrap().len() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];
    let mut read_stacks = true;

    for line in contents.iter() {
        if read_stacks {
            read_stacks = read_stack(*line, &mut stacks);

            // Reverse the stacks so end of the stack is at the end of the vec
            if !read_stacks {
                for vec in stacks.iter_mut() {
                    vec.reverse();
                }
            }
            continue;
        }

        if *line == "" {
            continue;
        }

        // It is an 'instruction' line
        let instr = line.split(' ').collect::<Vec<_>>();
        let num_boxes: usize = FromStr::from_str(instr.get(1).unwrap()).unwrap();
        let src: usize = FromStr::from_str(instr.get(3).unwrap()).unwrap();
        let dest: usize = FromStr::from_str(instr.get(5).unwrap()).unwrap();

        // _part1(num_boxes, src, dest, &mut stacks);
        _part2(num_boxes, src, dest, &mut stacks);
    }

    // I guess we assume there will always be at least one box on each stack?
    let mut top_boxes = String::from("");
    for stack in stacks.iter_mut() {
        top_boxes.push(stack.get(stack.len() - 1).unwrap().clone());
    }

    println!("Part ?: {top_boxes}");
}

fn read_stack(line: &str, stacks: &mut Vec<Vec<char>>) -> bool {
    let mut count = 1;
    for c in line.chars() {
        if c == '1' {
            return false;
        }

        if count > 1 && (count - 2) % 4 == 0 && c != ' ' {
            stacks.get_mut(count / 4).unwrap().push(c.clone());
        }

        count += 1;
    }
    true
}

fn _part1(num_boxes: usize, src: usize, dest: usize, stacks: &mut Vec<Vec<char>>) {
    for _ in 0..num_boxes {
        let boxx = stacks.get_mut(src - 1).unwrap().pop().unwrap();
        stacks.get_mut(dest - 1).unwrap().push(boxx);
    }
}

fn _part2(num_boxes: usize, src: usize, dest: usize, stacks: &mut Vec<Vec<char>>) {
    let mut temp_stack: Vec<char> = Vec::new();
    for _ in 0..num_boxes {
        let boxx = stacks.get_mut(src - 1).unwrap().pop().unwrap();
        temp_stack.push(boxx);
    }
    temp_stack.reverse();
    for boxx in temp_stack {
        stacks.get_mut(dest - 1).unwrap().push(boxx);
    }
}
