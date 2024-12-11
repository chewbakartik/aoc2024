use std::collections::{HashMap};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u16> {
    let manual: Manual = input.into();
    let val: u16;

    val = manual.sum_of_ordered();

    Some(val)
}

pub fn part_two(input: &str) -> Option<u16> {
    let manual: Manual = input.into();
    let val: u16;

    val = manual.sum_of_unordered();
    Some(val)
}

struct Manual {
    rules: HashMap<u16, Vec<u16>>,
    updates: Vec<Vec<u16>>,
}

impl Manual {
    fn sum_of_ordered(&self) -> u16 {
        let mut val = 0;
        self.updates.iter()
            .filter(|&update| self.is_correct_order(&update))
            .for_each(|update| {
                val += update[update.len() / 2];
            });
        val
    }

    fn sum_of_unordered(&self) -> u16 {
        let mut val = 0;
        self.updates.iter()
            .filter(|&update| !self.is_correct_order(&update))
            .for_each(|update| {
                let corrected_order = self.fix_order(&update);
                val += corrected_order[corrected_order.len() / 2];
            });
        val
    }

    fn is_correct_order(&self, update: &Vec<u16>) -> bool {
        let mut processed = Vec::<u16>::new();
        for num in update {
            for page in &processed {
                if let Some(rule) = &self.rules.get(page) {
                    if rule.contains(num) {
                        return false;
                    }
                }
            }
            processed.push(*num);
        }
        true
    }

    fn fix_order(&self, update: &Vec<u16>) -> Vec<u16> {
        let mut processed: Vec<u16> = Vec::new();
        for (i, num) in update.iter().enumerate() {
            let mut index_to_insert = i;
            for (index, page) in processed.iter().enumerate() {
                if let Some(rule) = &self.rules.get(page) {
                    if rule.contains(num) {
                        index_to_insert = index;
                        break;
                    }
                }
            }
            processed.insert(index_to_insert, *num);
        }
        processed
    }
}

impl From<&str> for Manual {
    fn from(input: &str) -> Self {
        let manual: Vec<&str> = input.split("\n\n").collect(); // this splits the top part and the bottom part of the inputs.
        let (rules_part, updates_part) = (manual[0], manual[1]);

        let mut rules: HashMap<u16, Vec<u16>> = HashMap::new();
        for line in rules_part.lines() {
            let parts: Vec<u16> = line.split('|')
                .filter_map(|p| p.trim().parse().ok())
                .collect();
            rules.entry(parts[1]).or_insert(vec![]).push(parts[0]);
        }

        let mut updates: Vec<Vec<u16>> = Vec::new();
        for line in updates_part.lines() {
            let update: Vec<u16> = line.split(',')
                .filter_map(|p| p.trim().parse().ok())
                .collect();
            if !update.is_empty() {
                updates.push(update);
            }
        }

        Manual { rules, updates }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
