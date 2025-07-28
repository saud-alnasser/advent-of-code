use std::collections::{HashMap, HashSet};

crate::puzzle!("2015_09_01");

// AI used to solve this puzzle.

impl Solution for Puzzle {
    type Input = (Vec<String>, HashMap<(String, String), usize>);

    fn parse(input: &str) -> Self::Input {
        let mut cities = HashSet::new();
        let mut distances = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split(" to ");
            let from = parts.next().unwrap().to_string();
            let mut right = parts.next().unwrap().split(" = ");
            let to = right.next().unwrap().to_string();
            let distance = right.next().unwrap().parse::<usize>().unwrap();

            cities.insert(from.clone());
            cities.insert(to.clone());

            distances.insert((from.clone(), to.clone()), distance);
            distances.insert((to, from), distance);
        }

        (cities.into_iter().collect(), distances)
    }

    fn solve((cities, distances): Self::Input) -> Option<String> {
        fn permutations(cities: &[String]) -> Vec<Vec<String>> {
            if cities.len() <= 1 {
                return vec![cities.to_vec()];
            }

            let mut result = Vec::new();

            for i in 0..cities.len() {
                let mut remaining = cities.to_vec();
                let current = remaining.remove(i);

                for mut perm in permutations(&remaining) {
                    perm.insert(0, current.clone());
                    result.push(perm);
                }
            }

            result
        }

        Some(
            permutations(&cities)
                .iter()
                .map(|route| {
                    route
                        .windows(2)
                        .map(|pair| distances[&(pair[0].clone(), pair[1].clone())])
                        .sum::<usize>()
                })
                .min()
                .unwrap()
                .to_string(),
        )
    }
}
