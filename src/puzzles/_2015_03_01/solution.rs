crate::puzzle!("2015_03_01");

use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector(i32, i32);

impl From<char> for Vector {
    fn from(c: char) -> Self {
        match c {
            '^' => Vector(0, 1),
            'v' => Vector(0, -1),
            '>' => Vector(1, 0),
            '<' => Vector(-1, 0),
            _ => panic!("invalid direction char: {}", c),
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Solution for Puzzle {
    type Input = Vec<Vector>;

    fn parse(input: &str) -> Self::Input {
        input.chars().map(Vector::from).collect()
    }

    fn solve(vectors: Self::Input) -> Option<String> {
        let mut visited = std::collections::HashSet::new();

        let mut santa_position = Vector(0, 0);
        visited.insert(santa_position);

        for vector in vectors {
            santa_position += vector;
            visited.insert(santa_position);
        }

        Some(visited.len().to_string())
    }
}
