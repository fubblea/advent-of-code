use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result = re
        .find_iter(input)
        .map(|x| {
            x.as_str()
                .strip_prefix("mul(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(",")
                .fold(1, |mul, x| match x.to_string().parse::<i32>() {
                    Ok(v) => mul * v,
                    Err(_) => mul,
                })
        })
        .sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
