use super::TaskTrait;
use std::{collections::HashMap, fs};

pub struct Task {
    sorted_lists: (Vec<i32>, Vec<i32>)
}

impl Task {
    pub fn create() -> Self {
        return Task {
            sorted_lists: (Vec::new(), Vec::new()),
        };
    }

    fn part1(&self) {
        let (list1, list2) = &self.sorted_lists;

        let total_pairs = list1.len();

        let mut total_diff = 0;

        for i in 0..total_pairs {
            let diff = list1[i].abs_diff(list2[i]);

            total_diff += diff;
        }

        println!("Task 1, part 1 answer is {total_diff}");
    }

    fn part2(&self) {
        let (list1, list2) = &self.sorted_lists;

        let mut list2_map: HashMap<i32, i32> = HashMap::new();

        for number in list2 {
            let count = list2_map.get(&number).unwrap_or(&0);
            list2_map.insert(*number, count + 1);
        }

        let mut total_sum = 0;
        for number in list1 {
            let count = list2_map.get(&number).unwrap_or(&0);

            total_sum += number * count;
        }

        println!("Task 1, part 2 answer is {total_sum}");
    }

    fn init(&mut self) {
        let input = fs::read_to_string("src/tasks/task01/input.txt").unwrap();

        let mut list1: Vec<i32> = Vec::new();
        let mut list2: Vec<i32> = Vec::new();

        for line in input.lines() {
            let numbers: Vec<&str> = line.split("   ").collect();

            list1.push(numbers[0].parse().unwrap());
            list2.push(numbers[1].parse().unwrap());
        }

        list1.sort();
        list2.sort();

        self.sorted_lists = (list1, list2);
    }
}

impl TaskTrait for Task {
    fn run(&mut self) {
        self.init();

        self.part1();
        self.part2();
    }
}
