use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)").unwrap();

    let mut mul_flag = true;

    let result = re
        .find_iter(input)
        .map(|x| {
            let x = x.as_str();

            if x == "do()" {
                mul_flag = true;
                0
            } else if x == "don't()" {
                mul_flag = false;
                0
            } else if mul_flag {
                x.strip_prefix("mul(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(",")
                    .fold(1, |mul, x| match x.to_string().parse::<i32>() {
                        Ok(v) => mul * v,
                        Err(_) => mul,
                    })
            } else {
                0
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
