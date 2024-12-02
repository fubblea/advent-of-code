#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input
        .split("\n")
        .map(|row| {
            let clean_row = row
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let mut factor = None;

            for idx in 1..clean_row.len() {
                let diff = clean_row[idx] - clean_row[idx - 1];

                if diff == 0 {
                    return 0;
                }

                if factor.is_none() {
                    factor = Some(diff.signum())
                } else if factor.unwrap() != diff.signum() {
                    return 0;
                }

                if diff.abs() > 3 {
                    return 0;
                }
            }
            1
        })
        .sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
