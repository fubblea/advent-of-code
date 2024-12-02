fn check_row(row: &mut Vec<i32>, one_ok: &mut bool) -> bool {
    let allow_one = |row: &mut Vec<i32>, one_ok: &mut bool, idx: &usize| -> bool {
        if *one_ok {
            *one_ok = false;

            let check_1 = {
                let mut row_1 = row.clone();
                row_1.remove(*idx);
                check_row(&mut row_1, one_ok)
            };
            let check_2 = {
                let mut row_2 = row.clone();
                row_2.remove(*idx - 1);
                check_row(&mut row_2, one_ok)
            };
            check_1 || check_2
        } else {
            false
        }
    };

    let mut factor = None;

    for idx in 1..row.len() {
        let diff = row[idx] - row[idx - 1];

        if diff == 0 {
            return allow_one(row, one_ok, &idx);
        }

        if factor.is_none() {
            factor = Some(diff.signum())
        } else if factor.unwrap() != diff.signum() {
            return allow_one(row, one_ok, &idx);
        }

        if diff.abs() > 3 {
            return allow_one(row, one_ok, &idx);
        }
    }
    true
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input
        .split("\n")
        .map(|row| {
            let mut clean_row = row
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut one_ok = true;

            let val: i32 = check_row(&mut clean_row, &mut one_ok).into();
            val
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
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
