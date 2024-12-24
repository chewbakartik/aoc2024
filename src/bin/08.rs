use std::{collections::{HashMap, HashSet}, ops::{Add, Sub, AddAssign}};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Map = Map::from(input);
    map.signal().len().try_into().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Map = Map::from(input);
    map.harmonics().len().try_into().ok()
}

#[derive(Debug)]
struct Map {
    lists: HashMap<u8, Vec<Point>>,
    rows: usize,
    cols: usize
}

impl Map {
    // we don't actually care about the number of antinodes per antenna, just total amount
    // so we need to build the pairs list which relates the same antenna type to each other
    // but adds to the total list that we can work through.
    fn get_pairs(&self) -> Vec<(Point, Point)> {
        self.lists.values()
           .flat_map(|antenna| {
                antenna.iter()
                    .flat_map(|&p1| antenna.iter().map(move |&p2| (p1, p2)))
                    .filter(|(p1, p2)| p1 != p2)
                })
                .collect()
    }

    fn signal(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for (p1, p2) in self.get_pairs() {
            [p1 + (p1 - p2), p2 + (p2 - p1)]
                .into_iter()
                .filter(move |&antinode| self.is_valid_antinode(antinode))
                .for_each(|antinode| {
                    antinodes.insert(antinode);
                })
        }
        antinodes
    }

    fn harmonics(&self) -> HashSet<Point> {
        let mut harmonics = HashSet::new();
        for (p1, p2) in self.get_pairs() {
            harmonics.insert(p1);
            harmonics.insert(p2);
            harmonics.extend(self.check_harmonics(p1, p1 - p2));
            harmonics.extend(self.check_harmonics(p2, p2 - p1));
        }
        harmonics
    }

    fn check_harmonics(&self, p: Point, dp: Point) -> HashSet<Point> {
        let mut harmonics: HashSet<Point> = HashSet::new();
        let np = p + dp;
        if self.is_valid_antinode(np) {
            harmonics.insert(np);
            harmonics.extend(self.check_harmonics(np, dp));
        }
        harmonics
    }

    fn is_valid_antinode(&self, node: Point) -> bool {
        node.0 >= 0 && node.0 <= self.cols as i32 && node.1 >= 0 && node.1 <= self.rows as i32
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut lists: HashMap<u8, Vec<Point>> = HashMap::new();
        let ignored_chars: HashSet<u8> = vec![b'.'].into_iter().collect();
        let (mut rows, mut cols) = (0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, byte) in line.bytes().enumerate() {
                if !ignored_chars.contains(&byte) {
                    lists.entry(byte).or_insert_with(Vec::new).push(Point(x as i32, y as i32));
                }
                cols = x;
            }
            rows = y;
        }
        Self { lists, rows, cols }
    }
}

#[derive(Clone, Copy, Default, Hash, Eq, Debug)]
struct Point(i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
