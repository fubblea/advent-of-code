#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for (i, value) in input.split_ascii_whitespace().enumerate() {
        if i % 2 == 0 {
            left.push(value.parse::<i32>().unwrap());
        } else {
            right.push(value.parse::<i32>().unwrap());
        }
    }

    let result = left.iter().fold(0, |acc, x| {
        acc + (x * right.iter().filter(|y| y == &x).count() as i32)
    });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
