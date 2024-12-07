use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum = 0;
    for line in input.lines() {
        sum += p1(&line);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split("do()").collect();
        for (index, part) in parts.iter().enumerate() {
            let parts_ignore: Vec<&str> = part.split("don't()").collect();
            if let Some(first_part) = parts_ignore.get(0) {
                println!("Parts Ignore: {}", &first_part);
                sum += p1(&first_part.to_string());
            }
        }
    }
    Some(sum)
}

pub fn p1(line: &str) -> i32 {
    let re = Regex::new(
        r"(?x)
            mul         # match literal
            \(
            (?<op1>\d{1,3}) # first operand
            ,
            (?<op2>\d{1,3}) # second operand
            \)
        ",
    ).unwrap();
    let sum = re
        .captures_iter(line)
        .map(|cap| cap["op1"].parse::<i32>().unwrap() * cap["op2"].parse::<i32>().unwrap())
        .sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
