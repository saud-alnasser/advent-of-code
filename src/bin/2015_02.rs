aoc::puzzle!("2015:02");

struct Gift {
    l: u32,
    w: u32,
    h: u32,
}

impl Gift {
    fn area(&self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn slack(&self) -> u32 {
        let mut sides = [self.l, self.w, self.h];
        sides.sort_unstable();

        sides[0] * sides[1]
    }

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

    fn solve_part1(gifts: Self::Input) -> Option<String> {
        Some(
            gifts
                .iter()
                .map(|gift| gift.area() + gift.slack())
                .sum::<u32>()
                .to_string(),
        )
    }

    fn solve_part2(gifts: Self::Input) -> Option<String> {
        Some(
            gifts
                .iter()
                .map(|gift| gift.ribbon() + gift.bow())
                .sum::<u32>()
                .to_string(),
        )
    }
}
