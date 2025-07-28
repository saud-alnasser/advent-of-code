crate::puzzle!("2015_05_02");

trait Rule {
    fn accept(&self, value: &str) -> bool;
}

#[derive(PartialEq, Eq)]
enum StringKind {
    Nice,
    Naughty,
}

struct StringQuantifier {
    rules: Vec<Box<dyn Rule>>,
}

impl StringQuantifier {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self { rules }
    }

    pub fn kind(&self, value: &str) -> StringKind {
        match self.rules.iter().all(|rule| rule.accept(value)) {
            true => StringKind::Nice,
            false => StringKind::Naughty,
        }
    }
}

struct PairsRule;

impl Rule for PairsRule {
    fn accept(&self, value: &str) -> bool {
        value
            .chars()
            .zip(value.chars().skip(1))
            .enumerate()
            .any(|(i, (a, b))| {
                value
                    .chars()
                    .skip(i + 2)
                    .zip(value.chars().skip(i + 3))
                    .any(|(c, d)| a == c && b == d)
            })
    }
}

struct RepeatRule;

impl Rule for RepeatRule {
    fn accept(&self, value: &str) -> bool {
        value
            .chars()
            .zip(value.chars().skip(2))
            .any(|(a, b)| a == b)
    }
}

impl Solution for Puzzle {
    type Input = Vec<String>;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(str::to_string).collect()
    }

    fn solve(inputs: Self::Input) -> Option<String> {
        let quantifier = StringQuantifier::new(vec![Box::new(PairsRule), Box::new(RepeatRule)]);

        let nice = inputs
            .iter()
            .filter(|input| quantifier.kind(input) == StringKind::Nice)
            .count();

        Some(nice.to_string())
    }
}
