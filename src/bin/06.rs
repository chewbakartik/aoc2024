use std::{ops::Add, collections::HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lab = Lab::from(input);
    let count = lab.walk().visited.len() as u32;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lab = Lab::from(input);
    let count = lab.sabatoge().diversions.len() as u32;
    Some(count)
}

struct Lab {
    grid: Grid,
    guard: Guard,
    visited: HashSet<Position>,
    diversions: HashSet<Position>,
    og: Guard,
}

impl Lab {
    fn walk(&mut self) -> &Self {
        loop {
            let next = self.guard.position + self.guard.position.direction.offset();

            // This takes a lot longer than using contains on the HashSet that doesn't track position direction,
            // but we need to store unique row, col values for Part 1
            // Part 2 requires us to also know the direction of the position, however it breaks if we treat the same x, y as different
            // depending on the way the guard is facing.
            let v = self.visited.iter().any(|pos| pos.row == self.guard.position.row && pos.col == self.guard.position.col);
            if !v {
                self.visited.insert(self.guard.position);
            }

            match self.grid.get(next) {
                Some(b'#') => self.guard.position.direction = self.guard.position.direction.turn(),
                Some(_) => self.guard.position = next,
                None => break,
            }
        }
        self
    }

    fn sabatoge(&mut self) -> &Self {
        // Get the conditions right.
        self.walk();
        for (i, pos) in self.visited.iter().enumerate() {
            // reset guard to starting point
            let guard = self.og.clone();
            let mut grid = self.grid.clone();
            // change grid to add obstacle
            grid.set(*pos, b'#');
            if self.check_diversion(grid, guard) {
                self.diversions.insert(*pos);
            }
        }
        self
    }

    fn check_diversion(&self, grid: Grid, mut guard: Guard) -> bool {
        let mut visited: HashSet<Position> = HashSet::new();
        let mut i = 0;
        loop {
            let next = guard.position + guard.position.direction.offset();
            if visited.contains(&guard.position) {
                return true;
            }
            visited.insert(guard.position);
            match grid.get(next) {
                Some(b'#') => guard.position.direction = guard.position.direction.turn(),
                Some(_) => guard.position = next,
                None => return false
            }
            i += 1;
        }
    }
}

impl From<&str> for Lab {
    fn from(input: &str) -> Self {
        let grid = Grid::from(input);
        let guard = Guard {
            position: grid.find_character(b'^', Direction::Up),
        };
        let og = guard.clone();
        // Self { grid, guard, routes }
        Self { grid, guard, visited: HashSet::new(), diversions: HashSet::new(), og }
    }
}

#[derive(Clone, Debug)]
struct Grid {
    bytes: Vec<Vec<u8>>,
}

impl Grid {
    fn get(&self, position: Position) -> Option<u8> {
        self.bytes
            .get(position.row as usize)?.get(position.col as usize).copied()
    }

    fn set(&mut self, position: Position, byte: u8) -> Self {
        if let Some(row) = self.bytes.get_mut(position.row as usize) {
            if let Some(cell) = row.get_mut(position.col as usize) {
                *cell = byte;
            }
        }
        self.clone()
    }

    fn find_character(&self, target: u8, direction: Direction) -> Position {
        self.bytes.iter().enumerate().find_map(|(row_index, row)| {
            row.iter()
                .position(|&byte| byte == target)
                .map(|col_index| Position::new(col_index as i32, row_index as i32, direction))
            })
            .unwrap_or_default()
    }

    fn show(&self) {
        for row in &self.bytes {
            let line: String = row.iter()
                .map(|&byte| char::from(byte))
                .collect();
            println!("{}", line);
        }
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let bytes: Vec<Vec<u8>> = input.lines().map(|row| row.bytes().collect()).collect();
        Self { bytes }
    }
}

#[derive(Clone)]
struct Guard {
    position: Position,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
enum Direction {
    Up, Down, Right, Left
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

impl Direction {
    fn offset(self) -> Offset {
        match self {
            Direction::Up => Offset(0, -1),
            Direction::Down => Offset(0, 1),
            Direction::Left => Offset(-1, 0),
            Direction::Right => Offset(1, 0),
        }
    }

    fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Default, Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Position{
    row: i32,
    col: i32,
    direction: Direction,
}

impl Add<Offset> for Position {
    type Output = Self;
    fn add(self, Offset(dx, dy): Offset) -> Self::Output {
        Position::new(self.col + dx, self.row + dy, self.direction)
    }
}

impl Position {
    fn new(col: i32, row: i32, direction: Direction) -> Self {
        Position { col, row, direction }
    }
}

struct Offset(i32, i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
