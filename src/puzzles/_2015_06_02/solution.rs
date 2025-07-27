crate::puzzle!("2015_06_02");

#[derive(Debug, Clone, Copy)]
struct Light {
    on: bool,
    brightness: u32,
}

struct Grid {
    lights: Vec<Vec<Light>>,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            lights: vec![
                vec![
                    Light {
                        on: false,
                        brightness: 0
                    };
                    cols
                ];
                rows
            ],
        }
    }

    fn apply(&mut self, instruction: Instruction, range: Range) {
        let Range {
            from: (x1, y1),
            to: (x2, y2),
        } = range;

        for x in x1..=x2 {
            for y in y1..=y2 {
                match instruction {
                    Instruction::TurnOn => {
                        self.lights[x][y].on = true;
                        self.lights[x][y].brightness += 1;
                    }
                    Instruction::TurnOff => {
                        self.lights[x][y].on = false;
                        self.lights[x][y].brightness =
                            self.lights[x][y].brightness.saturating_sub(1);
                    }
                    Instruction::Toggle => {
                        self.lights[x][y].on = !self.lights[x][y].on;
                        self.lights[x][y].brightness += 2;
                    }
                }
            }
        }
    }
}

pub struct Range {
    from: (usize, usize),
    to: (usize, usize),
}

pub enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Solution for Puzzle {
    type Input = Vec<(Instruction, Range)>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let mut words = line.split_whitespace();

                let instruction = match words.next().unwrap() {
                    "turn" => match words.next().unwrap() {
                        "on" => Instruction::TurnOn,
                        "off" => Instruction::TurnOff,
                        _ => unreachable!(),
                    },
                    "toggle" => Instruction::Toggle,
                    _ => unreachable!(),
                };

                let from = {
                    let mut from = words.next().unwrap().split(',');
                    (
                        from.next().unwrap().parse().unwrap(),
                        from.next().unwrap().parse().unwrap(),
                    )
                };

                let to = {
                    let mut to = words.last().unwrap().split(',');
                    (
                        to.next().unwrap().parse().unwrap(),
                        to.next().unwrap().parse().unwrap(),
                    )
                };

                (instruction, Range { from, to })
            })
            .collect()
    }

    fn solve(instructions: Self::Input) -> Option<String> {
        let mut grid = Grid::new(1000, 1000);

        for (instruction, range) in instructions {
            grid.apply(instruction, range);
        }

        Some(
            grid.lights
                .iter()
                .flatten()
                .map(|&light| light.brightness)
                .sum::<u32>()
                .to_string(),
        )
    }
}
