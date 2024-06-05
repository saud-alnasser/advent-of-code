aoc::puzzle!("2015:01");

enum Direction {
    Up,
    Down,
    Unknown,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '(' => Direction::Up,
            ')' => Direction::Down,
            _ => Direction::Unknown,
        }
    }
}

impl From<Direction> for i32 {
    fn from(direction: Direction) -> i32 {
        match direction {
            Direction::Up => 1,
            Direction::Down => -1,
            Direction::Unknown => 0,
        }
    }
}

impl Solution for Puzzle {
    type Structure = Vec<Direction>;

    fn parse(input: &str) -> Self::Structure {
        input.chars().map(Direction::from).collect()
    }

    fn solve_part1(directions: Self::Structure) -> Option<String> {
        let mut floor = 0;

        for direction in directions {
            floor += i32::from(direction);
        }

        Some(floor.to_string())
    }

    fn solve_part2(directions: Self::Structure) -> Option<String> {
        let mut floor = 0;
        let mut position = 1;

        for direction in directions {
            floor += i32::from(direction);

            if floor < 0 {
                return Some(position.to_string());
            }

            position += 1;
        }

        Some(floor.to_string())
    }
}
