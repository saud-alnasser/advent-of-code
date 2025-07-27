crate::puzzle!("2015_02_02");

pub struct Gift {
    l: u32,
    w: u32,
    h: u32,
}

impl Gift {
    fn ribbon(&self) -> u32 {
        let mut sides = [self.l, self.w, self.h];
        sides.sort_unstable();

        2 * sides[0] + 2 * sides[1]
    }

    fn bow(&self) -> u32 {
        self.l * self.w * self.h
    }
}

impl Solution for Puzzle {
    type Input = Vec<Gift>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let mut parts = line.split('x');

                Gift {
                    l: parts.next().unwrap().parse().unwrap(),
                    w: parts.next().unwrap().parse().unwrap(),
                    h: parts.next().unwrap().parse().unwrap(),
                }
            })
            .collect()
    }

    fn solve(gifts: Self::Input) -> Option<String> {
        Some(
            gifts
                .iter()
                .map(|gift| gift.ribbon() + gift.bow())
                .sum::<u32>()
                .to_string(),
        )
    }
}
