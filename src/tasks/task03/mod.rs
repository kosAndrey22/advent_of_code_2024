use std::fs;

use regex::Regex;

use super::TaskTrait;

pub struct Task {
    input: String
}

impl Task {
    pub fn create() -> Self {
        return Task {
            input: String::new(),
        };
    }

    fn part1(&self) {
        let regex = Regex::new(r"mul\((?<f>[0-9]{1,3}),(?<s>[0-9]{1,3})\)").unwrap();
        let pairs: Vec<(i32, i32)> = regex
            .captures_iter(&&self.input)
            .map(|caps| {
                let first: i32 = caps.name("f").unwrap().as_str().parse().unwrap();
                let second: i32 = caps.name("s").unwrap().as_str().parse().unwrap();
                (first, second)
            })
            .collect();

        let mut total_amount = 0;

        for (first, second) in pairs {
            total_amount += first * second;
        }

        println!("Task 3, part 1 answer is {total_amount}");
    }

    fn part2(&self) {
        let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
        let matches: Vec<&str> = regex.find_iter(&self.input).map(|m| m.as_str()).collect();

        let mut total_amount = 0;
        let mut enabled = true;

        for instruction in matches {
            match instruction {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    let pair = instruction
                        .strip_suffix(")")
                        .unwrap()
                        .strip_prefix("mul(")
                        .unwrap();
                    let numbers: Vec<&str> = pair.split(",").collect();
                    let number1: i32 = numbers[0].parse().unwrap();
                    let number2: i32 = numbers[1].parse().unwrap();

                    if enabled {
                      total_amount += number1 * number2;
                    }
                }
            }
        }

        println!("Task 3, part 2 answer is {total_amount}");
    }

    fn init(&mut self) {
        let input = fs::read_to_string("src/tasks/task03/input.txt").unwrap();
        self.input = input;
    }
}

impl TaskTrait for Task {
    fn run(&mut self) {
        self.init();

        self.part1();
        self.part2();
    }
}
