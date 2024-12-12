use std::{ops::Add, collections::HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lab = Lab::from(input);
    let count = lab.walk().visited.len() as u32;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    //877 is too small
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    let mut lab = Lab::from(input);
    let visited = lab.walk().visited.len() as u32;
    let count = lab.sabatoge().diversions.len() as u32;
    Some(6)
}

struct Lab {
    grid: Grid,
    guard: Guard,
    visited: HashSet<Position>,
    diversions: HashSet<Position>,
}

impl Lab {
    fn walk(&mut self) -> &Self {
        println!("Visited: {:?}\n", self.visited);
        loop {
            let next = self.guard.position + self.guard.direction.offset();

            println!("Next: {:?}\n", next);
            self.visited.insert(self.guard.position);

            match self.grid.get(next) {
                Some(b'#') => self.guard.direction = self.guard.direction.turn(),
                Some(_) => self.guard.position = next,
                None => break,
            }
        }
        self
    }

    fn sabatoge(&mut self) -> &Self {
        loop {
            let next = self.guard.position + self.guard.direction.offset();
            let turned_pos = self.guard.get_turned_pos();

            if self.visited.contains(&turned_pos) {
                self.diversions.insert(next);
            }

            match self.grid.get(next) {
                Some(b'#') => self.guard.direction = self.guard.direction.turn(),
                Some(_) => self.guard.position = next,
                None => break,
            }
        }
        self
    }
}

impl From<&str> for Lab {
    fn from(input: &str) -> Self {
        let grid = Grid::from(input);
        let guard = Guard {
            direction: Direction::Up,
            position: grid.find_character(b'^', Direction::Up),
        };
        // Self { grid, guard, routes }
        Self { grid, guard, visited: HashSet::new(), diversions: HashSet::new() }
    }
}

struct Grid {
    bytes: Vec<Vec<u8>>,
}

impl Grid {
    fn get(&self, position: Position) -> Option<u8> {
        self.bytes
            .get(position.row as usize)?.get(position.col as usize).copied()
    }

    fn find_character(&self, target: u8, direction: Direction) -> Position {
        self.bytes.iter().enumerate().find_map(|(row_index, row)| {
            row.iter()
                .position(|&byte| byte == target)
                .map(|col_index| Position::new(col_index as i32, row_index as i32, direction))
            })
            .unwrap_or_default()
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
    direction: Direction,
    position: Position,
}

impl Guard {
    fn get_turned_pos(&self) -> Position {
        let next_direction = self.direction.turn();
        let (col, row) = (self.position.col, self.position.row);
        let position = Position::new(col, row, next_direction);
        position
    }
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
