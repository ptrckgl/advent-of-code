use std::{
    collections::HashSet,
    ops::{AddAssign, SubAssign},
};

#[derive(PartialEq, Eq, Hash, Clone)]
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

        // Deal with the possibilities of moving the tail
        if Position::distance_between(self, tail) > 1 {
            // They are still on the same row/col
            if tail.x == self.x || tail.y == self.y {
                *tail += *dir;
            }
            // A diagonal movement has been made
            else {
                *tail = self.clone();
                *tail -= *dir;
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

    for instruction in contents.iter() {
        let mut instr = instruction.split(' ');
        let (dir, len) = (
            Direction::get_direction(instr.next().unwrap()),
            instr.next().unwrap().parse::<u32>().unwrap(),
        );

        for _ in 0..len {
            head.move_head(&mut tail, &dir);
            tail_visited.insert(tail.clone());
        }
    }

    println!("Part 1: {}", tail_visited.len());
}
