use std::fs;

use super::TaskTrait;

pub struct Task {
    symbol_matrix: Vec<Vec<char>>,
}

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

const XMAS_WORD_CHARS: [char; 4] = ['X', 'M', 'A', 'S'];
const X_MAS_MARK_CHARS: [char; 3] = ['M', 'A', 'S'];

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
    pub fn create() -> Self {
        return Task {
            symbol_matrix: Vec::new(),
        };
    }

    fn part1(&self) {
        let mut total_count = 0;

        for x in 0..self.symbol_matrix.len() {
            let line = &self.symbol_matrix[x];

            for y in 0..line.len() {
                let element = line[y];

                if element != XMAS_WORD_CHARS[0] {
                    continue;
                }

                for direction in DIRECTIONS_ARRAY {
                    let is_match =
                        self.check_for_word_by_direction(&Point(x as i16, y as i16), &direction, 1);
                    if is_match {
                        total_count += 1;
                    }
                }
            }
        }

        println!("Task 4, part 1 answer is {total_count}");
    }

    fn part2(&self) {
        let mut total_count = 0;

        for x in 0..self.symbol_matrix.len() {
            let line = &self.symbol_matrix[x];

            for y in 0..line.len() {
                let is_match = self.check_for_mark(&Point(x as i16, y as i16));
                if is_match {
                    total_count += 1;
                }
            }
        }

        println!("Task 4, part 2 answer is {total_count}");
    }

    fn check_for_word_by_direction(
        &self,
        point: &Point,
        direction: &Direction,
        next_symbol_index: i32,
    ) -> bool {
        let next_cords = self.get_next_cords(point, &direction);
        if next_cords.is_none() {
            return false;
        }

        let next_cords = next_cords.unwrap();
        let next_symbol = self.get_symbol_by_point(&next_cords);

        if next_symbol != XMAS_WORD_CHARS[next_symbol_index as usize] {
            return false;
        }

        let is_last = next_symbol_index == (XMAS_WORD_CHARS.len() as i32) - 1;
        if is_last {
            return true;
        }

        let Point(next_x, next_y) = next_cords;

        return self.check_for_word_by_direction(
            &Point(next_x, next_y),
            direction,
            next_symbol_index + 1,
        );
    }


    fn check_for_mark(&self, point: &Point) -> bool {
        let start_symbol = self.get_symbol_by_point(point);
        if start_symbol != X_MAS_MARK_CHARS[0] && start_symbol != X_MAS_MARK_CHARS[2] {
            return false;
        }

        let central_cord = self.get_next_cords(point, &Direction::DownRight);
        if central_cord.is_none() {
            return false;
        }
        let central_cord = central_cord.unwrap();
        let central_symbol = self.get_symbol_by_point(&central_cord);
        if central_symbol != X_MAS_MARK_CHARS[1] {
            return false;
        }

        let directions_from_central_cord = vec![
            &Direction::UpRight,
            &Direction::DownRight,
            &Direction::DownLeft,
        ];

        let mut first_line_symbols: Vec<char> = vec![start_symbol];
        let mut second_line_symbols: Vec<char> = Vec::new();

        for direction in directions_from_central_cord {
            let cord = self.get_next_cords(&central_cord, &direction);
            if cord.is_none() {
                return false;
            }
            let cord = cord.unwrap();
            let symbol = self.get_symbol_by_point(&cord);

            if symbol != X_MAS_MARK_CHARS[0] && symbol != X_MAS_MARK_CHARS[2] {
                return false;
            }

            match direction {
                &Direction::DownRight => {
                    first_line_symbols.push(symbol);
                }
                _ => {
                    second_line_symbols.push(symbol);
                }
            }
        }

        let symbols_different = first_line_symbols[0] != first_line_symbols[1]
            && second_line_symbols[0] != second_line_symbols[1];

        return symbols_different;
    }

    fn get_symbol_by_point(&self, point: &Point) -> char {
        let &Point(x, y) = point;
        let symbol = self.symbol_matrix[x as usize][y as usize];

        return symbol;
    }

    fn get_next_cords(&self, point: &Point, direction: &Direction) -> Option<Point> {
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

        let next_x_valid = next_x >= 0 && next_x < self.symbol_matrix.len() as i16;
        let next_y_valid = next_y >= 0 && next_y < self.symbol_matrix[x as usize].len() as i16;

        if !next_x_valid || !next_y_valid {
            return None;
        }

        return Option::Some(Point(next_x, next_y));
    }

    fn init(&mut self) {
        let input = fs::read_to_string("src/tasks/task04/input.txt").unwrap();
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            let mut symbols: Vec<char> = Vec::new();

            for c in line.chars() {
                symbols.push(c)
            }

            matrix.push(symbols);
        }

        self.symbol_matrix = matrix;
    }
}

impl TaskTrait for Task {
    fn run(&mut self) {
        self.init();

        self.part1();
        self.part2();
    }
}
