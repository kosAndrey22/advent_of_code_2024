use super::TaskTrait;
use std::{cmp::Ordering, fs};

pub struct Task;

impl Task {
    fn part1(&self) {
        let reports = self.get_reports();

        let mut safe_amount = 0;

        for report in reports {
            let is_safe = self.check_report(report, false, false);

            if is_safe {
                safe_amount += 1;
            }
        }

        println!("Task 2, part 1 answer is {safe_amount}");
    }

    fn part2(&self) {
        let reports = self.get_reports();

        let mut safe_amount = 0;

        for report in reports {
            let is_safe = self.check_report(report, true, false);

            if is_safe {
                safe_amount += 1;
            }
        }

        println!("Task 2, part 2 answer is {safe_amount}");
    }

    fn check_report(&self, report: Vec<u8>, use_problem_dampener: bool, problem_dampener_used: bool) -> bool {
        if report.len() == 1 {
            return true;
        }

        let first = report[0];
        let second = report[1];
        let starting_order = first.cmp(&second);

        for n in 0..(report.len() - 1) {
            let current = report[n];
            let next = report[n + 1];
            let order = current.cmp(&next);

            let diff_too_big = current.abs_diff(next) > 3;
            let orders_not_match = current.cmp(&next) == Ordering::Equal || order != starting_order;

            if diff_too_big || orders_not_match {
                if problem_dampener_used || !use_problem_dampener {
                    return false;
                }

                let mut results: Vec<bool> = Vec::new();

                for n in 0..report.len() {
                  let index_to_remove = n;

                  let mut cloned_report = report.clone();
                  cloned_report.remove(index_to_remove);

                  let result = self.check_report(cloned_report, true, true);

                  results.push(result);
                }

                let is_some_safe = results.iter().any(|&r| r == true);

                return is_some_safe;
            }
        }

        return true;
    }

    fn get_reports(&self) -> Vec<Vec<u8>> {
        let input = fs::read_to_string("src/tasks/task02/input.txt").unwrap();

        let mut reports: Vec<Vec<u8>> = Vec::new();

        for line in input.lines() {
            let numbers: Vec<&str> = line.split(" ").collect();

            let mut parsed_number: Vec<u8> = Vec::new();
            for n in numbers {
                let parsed: u8 = n.parse().unwrap();
                parsed_number.push(parsed);
            }

            reports.push(parsed_number);
        }

        reports
    }
}

impl TaskTrait for Task {
    fn run(&self) {
        self.part1();
        self.part2();
    }
}
