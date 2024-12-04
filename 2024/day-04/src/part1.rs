use ndarray::prelude::*;
use rayon::prelude::*;

const TARGET: &str = "XMAS";

#[non_exhaustive]
struct SearchFactors;

impl SearchFactors {
    pub const UP: (i32, i32) = (-1, 0);
    pub const DOWN: (i32, i32) = (1, 0);
    pub const RIGHT: (i32, i32) = (0, 1);
    pub const LEFT: (i32, i32) = (0, -1);
    pub const TOPRIGHT: (i32, i32) = (-1, 1);
    pub const BOTTOMRIGHT: (i32, i32) = (1, 1);
    pub const TOPLEFT: (i32, i32) = (-1, -1);
    pub const BOTTOMLEFT: (i32, i32) = (1, -1);
}

fn check_xmas(root: (i32, i32), rf: (i32, i32), arr: ArrayView2<char>) -> i32 {
    let shape = arr.shape();

    if (root.0 + rf.0 * 3) < 0
        || (root.1 + rf.1 * 3) < 0
        || (root.0 + rf.0 * 3) >= shape[0] as i32
        || (root.1 + rf.1 * 3) >= shape[1] as i32
    {
        0
    } else {
        let v = vec![
            arr[[root.0 as usize, root.1 as usize]],
            arr[[(root.0 + rf.0) as usize, (root.1 + rf.1) as usize]],
            arr[[(root.0 + rf.0 * 2) as usize, (root.1 + rf.1 * 2) as usize]],
            arr[[(root.0 + rf.0 * 3) as usize, (root.1 + rf.1 * 3) as usize]],
        ];
        let s: String = v.into_iter().collect();

        if s == TARGET {
            1
        } else {
            0
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
            if input_arr[[i, j]] == 'X' {
                let rf_vec = vec![
                    SearchFactors::UP,
                    SearchFactors::DOWN,
                    SearchFactors::RIGHT,
                    SearchFactors::LEFT,
                    SearchFactors::TOPRIGHT,
                    SearchFactors::BOTTOMRIGHT,
                    SearchFactors::TOPLEFT,
                    SearchFactors::BOTTOMLEFT,
                ];

                let result = rf_vec
                    .into_par_iter()
                    .map(|rf| check_xmas((i as i32, j as i32), rf, input_arr.view()))
                    .sum::<i32>();

                xmas_count += result;
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
        assert_eq!("18", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_stripped() -> miette::Result<()> {
        let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_easy() -> miette::Result<()> {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
