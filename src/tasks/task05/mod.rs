use std::{collections::HashMap, fs};

use super::TaskTrait;

pub struct Task {
    before_rules_map: HashMap<String, Vec<String>>,
    after_rules_map: HashMap<String, Vec<String>>,
    updates: Vec<Vec<String>>,
    correct_updates_indexes: Vec<usize>,
}

impl Task {
    pub fn create() -> Self {
        return Task {
            before_rules_map: HashMap::new(),
            after_rules_map: HashMap::new(),
            updates: Vec::new(),
            correct_updates_indexes: Vec::new(),
        };
    }

    fn part1(&mut self) {
        let mut valid_updates: Vec<Vec<String>> = Vec::new();

        'update_loop: for update_index in 0..self.updates.len() {
            let numbers: &Vec<String> = &self.updates[update_index];

            for i in 0..numbers.len() {
                let current_number = &numbers[i];

                let previous_numbers = &numbers[0..i];
                let next_numbers = &numbers[i..numbers.len()];

                let before_rules = self.before_rules_map.get(current_number);
                match before_rules {
                    Some(rule_nubers) => {
                        for number in rule_nubers {
                            let is_break_rules =
                                previous_numbers.iter().find(|x| x.to_string() == *number);
                            if is_break_rules.is_some() {
                                continue 'update_loop;
                            }
                        }
                    }
                    None => {}
                }

                let after_rules = self.after_rules_map.get(current_number);
                match after_rules {
                    Some(rule_nubers) => {
                        for number in rule_nubers {
                            let is_break_rules =
                                next_numbers.iter().find(|x| x.to_string() == *number);
                            if is_break_rules.is_some() {
                                continue 'update_loop;
                            }
                        }
                    }
                    None => {}
                }

                if i == numbers.len() - 1 {
                    self.correct_updates_indexes.push(update_index);
                    valid_updates.push(numbers.clone());
                }
            }
        }

        let total_sum = self.calc_sum_of_updates(valid_updates);

        println!("Task 5, part 1 answer is {total_sum}");
    }

    fn part2(&self) {
        for update_index in 0..self.updates.len() {
            let is_update_correct = self
                .correct_updates_indexes
                .iter()
                .find(|x| **x == update_index);
            if is_update_correct.is_some() {
                continue;
            }
        }
    }

    fn calc_sum_of_updates(&self, updates: Vec<Vec<String>>) -> i32 {
        let mut total_sum: i32 = 0;
        for update in updates {
            let update_central_index = update.len() / 2;
            let central_element = &update[update_central_index];
            let parsed: i32 = central_element.parse().unwrap();
            total_sum += parsed;
        }

        total_sum
    }

    fn init(&mut self) {
        let input = fs::read_to_string("src/tasks/task05/input.txt").unwrap();

        let splitted: Vec<&str> = input.split("\n\n").collect();

        let rules = splitted[0];
        let updates = splitted[1];

        for rule in rules.lines() {
            let page_numbers: Vec<&str> = rule.split("|").collect();
            let before_number = page_numbers[0].to_string();
            let after_number = page_numbers[1].to_string();

            let existing_before_rule = self.before_rules_map.get_mut(&before_number);
            match existing_before_rule {
                Some(rule) => rule.push(after_number.clone()),
                None => {
                    self.before_rules_map
                        .insert(before_number.clone(), vec![after_number.clone()]);
                }
            }

            let existing_after_rule = self.after_rules_map.get_mut(&after_number);
            match existing_after_rule {
                Some(rule) => rule.push(before_number),
                None => {
                    self.after_rules_map
                        .insert(after_number, vec![before_number]);
                }
            }
        }

        for update in updates.lines() {
            let numbers: Vec<&str> = update.split(",").collect();

            let mut update_vector = Vec::new();
            for n in numbers {
                update_vector.push(n.to_string());
            }
            self.updates.push(update_vector);
        }
    }
}

impl TaskTrait for Task {
    fn run(&mut self) {
        self.init();

        self.part1();
        self.part2();
    }
}
