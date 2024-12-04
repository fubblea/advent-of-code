use ndarray::prelude::*;
use rayon::prelude::*;

const TARGET: &str = "MAS";

enum Diag {
    Right,
    Left,
}

impl std::fmt::Debug for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
        }
    }
}

fn check_xmas(root: (i32, i32), diag: Diag, arr: ArrayView2<char>) -> i32 {
    let shape = arr.shape();

    if root.0 < 1
        || root.1 < 1
        || (root.0 + 1) >= shape[0] as i32
        || (root.1 + 1) >= shape[1] as i32
    {
        0
    } else {
        match diag {
            Diag::Right => {
                let v = vec![
                    arr[[(root.0 - 1) as usize, (root.1 + 1) as usize]],
                    arr[[(root.0) as usize, (root.1) as usize]],
                    arr[[(root.0 + 1) as usize, (root.1 - 1) as usize]],
                ];
                let s: String = v.into_iter().collect();

                if s == TARGET || s.chars().rev().collect::<String>() == TARGET {
                    1
                } else {
                    0
                }
            }
            Diag::Left => {
                let v = vec![
                    arr[[(root.0 - 1) as usize, (root.1 - 1) as usize]],
                    arr[[(root.0) as usize, (root.1) as usize]],
                    arr[[(root.0 + 1) as usize, (root.1 + 1) as usize]],
                ];
                let s: String = v.into_iter().collect();

                if s == TARGET || s.chars().rev().collect::<String>() == TARGET {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input_vec = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let shape = (input_vec.len(), input_vec[0].len());
    let input_arr =
        match Array2::from_shape_vec(shape, input_vec.iter().flatten().cloned().collect()) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

    let mut xmas_count = 0;

    for i in 0..shape.0 {
        for j in 0..shape.1 {
            if input_arr[[i, j]] == 'A' {
                let rf_vec = vec![Diag::Right, Diag::Left];
                let res = rf_vec
                    .into_par_iter()
                    .map(|rf| check_xmas((i as i32, j as i32), rf, input_arr.view()))
                    .sum::<i32>();

                if res == 2 {
                    xmas_count += 1;
                }
            }
        }
    }

    Ok(xmas_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_stripped() -> miette::Result<()> {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!("9", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_easy() -> miette::Result<()> {
        let input = "M.S
.A.
M.S";
        assert_eq!("1", process(input)?);
        Ok(())
    }
}
