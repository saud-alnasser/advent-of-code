crate::puzzle!("2015_10_02");

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<char> for Digit {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            _ => panic!("invalid digit"),
        }
    }
}

impl From<usize> for Digit {
    fn from(n: usize) -> Self {
        match n {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => panic!("invalid digit"),
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Combination {
    digit: Digit,
    count: usize,
}

impl Combination {
    pub fn spread(&self) -> Vec<Digit> {
        vec![Digit::from(self.count), self.digit.clone()]
    }
}

impl Solution for Puzzle {
    type Input = Vec<Digit>;

    fn parse(input: &str) -> Self::Input {
        input
            .chars()
            .map(|c| Digit::from(c))
            .collect::<Vec<Digit>>()
    }

    fn solve(digits: Self::Input) -> Option<String> {
        fn combine(digits: &[Digit]) -> Vec<Digit> {
            let mut combinations = Vec::<Combination>::new();

            let mut digits = digits.iter().rev().collect::<Vec<&Digit>>();

            let mut latest = digits.pop().unwrap();
            let mut count = 1;

            while let Some(current) = digits.pop() {
                if current == latest {
                    count += 1;
                } else {
                    combinations.push(Combination {
                        digit: latest.clone(),
                        count,
                    });

                    latest = current;
                    count = 1;
                }
            }

            combinations.push(Combination {
                digit: latest.clone(),
                count,
            });

            combinations
                .iter()
                .map(|combination| combination.spread())
                .flatten()
                .collect::<Vec<Digit>>()
        }

        let mut output = digits;

        for _ in 0..50 {
            output = combine(&output);
        }

        Some(output.len().to_string())
    }
}
