use std::{
    collections::HashSet,
    ops::{AddAssign, SubAssign},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn distance_between(p1: &Position, p2: &Position) -> i32 {
        let mut dis = i32::abs(p1.x - p2.x) + i32::abs(p1.y - p2.y);

        // Account for diagonal
        if p1.x != p2.x && p1.y != p2.y {
            dis -= 1;
        }

        dis
    }

    /// This function should only be called with a 'head' Position
    fn move_head(&mut self, tail: &mut Position, dir: &Direction) {
        *self += *dir;
        Position::move_tail(self, tail, dir);
    }

    fn move_tail(head: &mut Position, tail: &mut Position, dir: &Direction) {
        // Deal with the possibilities of moving the tail
        if Position::distance_between(head, tail) > 1 {
            // They are still on the same row/col
            if tail.x == head.x || tail.y == head.y {
                *tail += *dir;
            }
            // A diagonal movement has been made
            else {
                *tail = head.clone();
                *tail -= *dir;
            }
        }
    }

    fn move_head_2(&mut self, tails: &mut [Position; 9], dir: &Direction) {
        *self += *dir;

        Position::move_tail(self, &mut tails[0], dir);

        for i in 0..8 {
            Position::move_tail_2(i, tails, dir);
        }
    }

    fn move_tail_2(idx: usize, tails: &mut [Position; 9], dir: &Direction) {
        if Position::distance_between(&tails[idx], &tails[idx + 1]) > 1 {
            if tails[idx + 1].x == tails[idx].x || tails[idx + 1].y == tails[idx].y {
                tails[idx + 1] += *dir;
            } else {
                tails[idx + 1] = tails[idx].clone();
                tails[idx + 1] -= *dir;
            }
        }
    }
}

impl Direction {
    fn get_direction(dir: &str) -> Direction {
        match dir {
            "L" => Direction::L,
            "R" => Direction::R,
            "U" => Direction::U,
            _ => Direction::D,
        }
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::L => self.x -= 1,
            Direction::R => self.x += 1,
            Direction::D => self.y -= 1,
            Direction::U => self.y += 1,
        }
    }
}

impl SubAssign<Direction> for Position {
    fn sub_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::L => self.x += 1,
            Direction::R => self.x -= 1,
            Direction::D => self.y += 1,
            Direction::U => self.y -= 1,
        }
    }
}

fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let mut head = Position::new(0, 0);
    let mut tail = Position::new(0, 0);
    let mut tail_visited: HashSet<Position> = HashSet::new();
    let mut head_2 = Position::new(0, 0);
    let mut tails = [Position::new(0, 0); 9];
    let mut tail_visited_2: HashSet<Position> = HashSet::new();

    for instruction in contents.iter() {
        let mut instr = instruction.split(' ');
        let (dir, len) = (
            Direction::get_direction(instr.next().unwrap()),
            instr.next().unwrap().parse::<u32>().unwrap(),
        );

        for _ in 0..len {
            head.move_head(&mut tail, &dir);
            tail_visited.insert(tail.clone());
            head_2.move_head_2(&mut tails, &dir);
            tail_visited_2.insert(tails[8].clone());
        }
    }

    println!("Part 1: {}", tail_visited.len());
    println!("Part 2: {}", tail_visited_2.len());
}
