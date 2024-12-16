advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    input.lines()
        .for_each(|line| {
            let mut parts = line.split(":");
            let left = parts.next().unwrap().trim().parse::<u64>().unwrap();
            let right = parts.next().unwrap().trim().split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            sum += eval_line_p1(left, right);
        });

    Some(sum)
}

/*
OK, so let's dig into this solution.
if we have 2 numbers, we have 2 potential operations
if we have 3 numbers, we have A + B + C, A + B * C, A * B * C, A * B + C for 4 operations
for 4 numbers:
A + B + C + D
A + B + C * D
A + B * C + D
A * B + C + D
A + B * C * D
A * B + C * D
A * B * C + D
A * B * C * D

I need to parse lines with the following structure of numbers `123: 4 6 8` where I need to store the left side value and create a vector with the right numbers that are separated by whitespace.

A = 2, B = 4, C = 5, D = 7

A + B, A * B => 6, 8
Res + C, Res * C => 11, 30, 13, 40
Res + D, Res * D => 18, 77, 37, 210, 20, 91, 47, 280

for 5 numbers:
A + B + C + D + E
A + B + C * D + E
A + B * C + D + E
A * B + C + D + E
A + B * C * D + E
A * B + C * D + E
A * B * C + D + E
A * B * C * D + E
A + B + C + D * E
A + B + C * D * E
A + B * C + D * E
A * B + C + D * E
A + B * C * D * E
A * B + C * D * E
A * B * C + D * E
A * B * C * D * E

number of operation possibilities is (n-1)^2
*/

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    input.lines()
        .for_each(|line| {
            let mut parts = line.split(":");
            let left = parts.next().unwrap().trim().parse::<u64>().unwrap();
            let right = parts.next().unwrap().trim().split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            sum += eval_line_p2(left, right);
        });

    Some(sum)
}

pub fn eval_line_p1(total: u64, nums: Vec<u64>) -> u64 {
    let mut results: Vec<Vec<u64>> = Vec::new();
    let mut sum: u64 = 0;
    // set up the conditions to be building the evaluations.
    results.push(vec![nums[0]]);
    for &num in &nums[1..] {
        let mut vals: Vec<u64> = Vec::new();
        if let Some(last_vec) = results.last() {
            for &r in last_vec.iter() {
                vals.push(r + num);
                vals.push(r * num);
            }
        }
        results.push(vals);
        // println!("Total: {} => Results Vec: {:?}", total, results);
    }
    if let Some(last_vec) = results.last() {
        if last_vec.contains(&total) {
            sum = total;
        }
    }
    sum
}

pub fn eval_line_p2(total: u64, nums: Vec<u64>) -> u64 {
    let mut results: Vec<Vec<u64>> = Vec::new();
    let mut sum: u64 = 0;
    // set up the conditions to be building the evaluations.
    results.push(vec![nums[0]]);
    for &num in &nums[1..] {
        let mut vals: Vec<u64> = Vec::new();
        if let Some(last_vec) = results.last() {
            for &r in last_vec.iter() {
                vals.push(r + num);
                vals.push(r * num);
                vals.push(concatenate(r, num));
            }
        }
        results.push(vals);
        // println!("Total: {} => Results Vec: {:?}", total, results);
    }
    if let Some(last_vec) = results.last() {
        if last_vec.contains(&total) {
            sum = total;
        }
    }
    sum
}

pub fn concatenate(a: u64, b: u64) -> u64 {
    let concatenated = format!("{}{}", a, b);
    concatenated.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
