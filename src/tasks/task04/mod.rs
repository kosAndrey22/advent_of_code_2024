use std::fs;

use super::TaskTrait;

pub struct Task;

struct Point(i16, i16);

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRECTIONS_ARRAY: [Direction; 8] = [
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];

impl Task {
    fn part1(&self) {
        let symbol_matrix = self.get_input();

        let mut total_count = 0;

        for x in 0..symbol_matrix.len() {
            let line = &symbol_matrix[x];

            for y in 0..line.len() {
                let element = line[y];

                if element != WORD[0] {
                    continue;
                }

                for direction in DIRECTIONS_ARRAY {
                    let is_match =
                        self.check_for_word_by_direction(&Point(x as i16, y as i16), &direction, &symbol_matrix, 1);
                    if is_match {
                        total_count += 1;
                    }
                }
            }
        }

        println!("Task 4, part 1 answer is {total_count}");
    }

    fn check_for_word_by_direction(
        &self,
        point: &Point,
        direction: &Direction,
        symbol_matrix: &Vec<Vec<char>>,
        next_symbol_index: i32,
    ) -> bool {
        let next_cords = self.get_next_cords(point, &direction, &symbol_matrix);
        if next_cords.is_none() {
            return false;
        }
        let Point(next_x, next_y) = next_cords.unwrap();

        let next_symbol = symbol_matrix[next_x as usize][next_y as usize];
        if next_symbol != WORD[next_symbol_index as usize] {
            return false;
        }

        let is_last = next_symbol_index == (WORD.len() as i32) - 1;
        if is_last {
            return true;
        }

        return self.check_for_word_by_direction(
            &Point(next_x, next_y),
            direction,
            &symbol_matrix,
            next_symbol_index + 1,
        );
    }

    fn get_next_cords(
        &self,
        point: &Point,
        direction: &Direction,
        symbol_matrix: &Vec<Vec<char>>,
    ) -> Option<Point> {
        let &Point(x, y) = point;

        let (next_x, next_y) = match direction {
            Direction::Up => (x - 1, y),
            Direction::UpRight => (x - 1, y + 1),
            Direction::Right => (x, y + 1),
            Direction::DownRight => (x + 1, y + 1),
            Direction::Down => (x + 1, y),
            Direction::DownLeft => (x + 1, y - 1),
            Direction::Left => (x, y - 1),
            Direction::UpLeft => (x - 1, y - 1),
        };

        let next_x_valid = next_x >= 0 && next_x < symbol_matrix.len() as i16;
        let next_y_valid = next_y >= 0 && next_y < symbol_matrix[x as usize].len() as i16;

        if !next_x_valid || !next_y_valid {
            return None;
        }

        return Option::Some(Point(next_x, next_y));
    }

    fn get_input(&self) -> Vec<Vec<char>> {
        let input = fs::read_to_string("src/tasks/task04/input.txt").unwrap();
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            let mut symbols: Vec<char> = Vec::new();

            for c in line.chars() {
                symbols.push(c)
            }

            matrix.push(symbols);
        }

        matrix
    }
}

impl TaskTrait for Task {
    fn run(&self) {
        self.part1();
        // Task::part2();
    }
}
