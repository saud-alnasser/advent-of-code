crate::puzzle!("2015_05_01");

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

struct VowelRule;

impl Rule for VowelRule {
    fn accept(&self, value: &str) -> bool {
        value.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
    }
}

struct DoubleRule;

impl Rule for DoubleRule {
    fn accept(&self, value: &str) -> bool {
        value
            .chars()
            .zip(value.chars().skip(1))
            .any(|(a, b)| a == b)
    }
}

struct ForbiddenRule;

impl Rule for ForbiddenRule {
    fn accept(&self, value: &str) -> bool {
        !value.contains("ab")
            && !value.contains("cd")
            && !value.contains("pq")
            && !value.contains("xy")
    }
}

impl Solution for Puzzle {
    type Input = Vec<String>;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(str::to_string).collect()
    }

    fn solve(inputs: Self::Input) -> Option<String> {
        let quantifier = StringQuantifier::new(vec![
            Box::new(VowelRule),
            Box::new(DoubleRule),
            Box::new(ForbiddenRule),
        ]);

        let nice = inputs
            .iter()
            .filter(|input| quantifier.kind(input) == StringKind::Nice)
            .count();

        Some(nice.to_string())
    }
}
