use std::cmp::Ordering;
use std::collections::HashMap;

use rayon::prelude::*;
use tracing::warn;

fn line_is_good(line: &[i32], rule_map: &HashMap<i32, Vec<i32>>) -> bool {
    let mut good_vals: Vec<bool> = Vec::new();
    for (idx, value) in line.iter().enumerate() {
        let all_good = line[(idx + 1)..].iter().all(|x| match rule_map.get(value) {
            Some(a_vec) => a_vec.contains(x),
            None => false,
        });

        if all_good {
            good_vals.push(true)
        } else {
            good_vals.push(false)
        }
    }

    good_vals.into_iter().all(|x| x)
}

fn order_bad(line: &mut [i32], rule_map: &HashMap<i32, Vec<i32>>) {
    line.sort_by(|a, b| match rule_map.get(a) {
        Some(a_vec) => {
            if a_vec.contains(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        None => Ordering::Greater,
    });
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();

    rules.lines().for_each(|line| {
        let (b, a) = line.split_once("|").unwrap();
        let (b, a) = (b.parse::<i32>().unwrap(), a.parse::<i32>().unwrap());

        match rule_map.get(&b) {
            Some(a_vec) => {
                let mut new_vec = a_vec.clone();
                new_vec.push(a);
                rule_map.insert(b, new_vec);
            }
            None => {
                let a_vec = vec![a];
                rule_map.insert(b, a_vec);
            }
        }
    });

    let result = updates
        .par_lines()
        .map(|line| {
            let mut line: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

            if line_is_good(&line, &rule_map) {
                0
            } else {
                order_bad(&mut line, &rule_map);
                line[line.len() / 2]
            }
        })
        .sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
