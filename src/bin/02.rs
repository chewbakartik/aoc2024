advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if core(&levels) {
            safe += 1;
        }
    }
    Some(safe)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if check_dampened(&levels) {
            safe += 1;
        }
    }
    Some(safe)
}

pub fn core(levels: &[i32]) -> bool {
    let diffs = levels.windows(2).map(|w| w[0] - w[1]);

    let in_range_decreasing = diffs.clone().all(|x| 1 <= x && x <= 3);
    let in_range_increasing = diffs.clone().all(|x| -3 <= x && x <= -1 );

    in_range_decreasing || in_range_increasing
}

pub fn check_dampened(levels: &[i32]) -> bool {
    for (i, _) in levels.iter().enumerate() {
        let mut dampened = levels.to_vec();
        dampened.remove(i);
        if core(&dampened) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
