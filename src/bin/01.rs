use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let (mut first, mut second) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        (first.push(nums[0]), second.push(nums[1]));
    }
    (first.sort(), second.sort());
    let sum = first.iter().zip(second.iter())
        .map(|(a, b)| (b-a).abs())
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (mut first, mut second) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        (first.push(nums[0]), second.push(nums[1]));
    }
    first.sort();

    let mut count_map: HashMap<i32, i32> = HashMap::new();
    for &item in &second {
        *count_map.entry(item).or_insert(0) += 1;
    }
    let sum = first.iter()
        .map(|&x| x * count_map.get(&x).unwrap_or(&0))
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
