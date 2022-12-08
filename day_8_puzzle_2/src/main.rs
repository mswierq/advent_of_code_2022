use std::io::{BufRead, BufReader};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut forest_map = Vec::<Vec<u8>>::new();
    for result_line in reader.lines() {
        let line = result_line.expect("Couldn't read a row!");
        forest_map.push(
            line.chars()
                .map(|height| height as u8 - '0' as u8)
                .collect(),
        );
    }

    let mut top_score = 0;
    let n_rows = forest_map.len();
    let n_cols = forest_map[0].len();
    for row_idx in 0..n_rows {
        for col_idx in 0..n_cols {
            let tree_size = forest_map[row_idx][col_idx];
            let l_side_score = forest_map[row_idx][0..col_idx]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, tree)| **tree >= tree_size)
                .map_or_else(|| col_idx, |(idx, _)| idx + 1);
            let r_side_score = forest_map[row_idx][col_idx + 1..n_cols]
                .iter()
                .enumerate()
                .find(|(_, tree)| **tree >= tree_size)
                .map_or_else(|| n_cols - 1 - col_idx, |(idx, _)| idx + 1);
            let u_side_score = forest_map[0..row_idx]
                .iter()
                .map(|rows| rows[col_idx])
                .rev()
                .enumerate()
                .find(|(_, tree)| *tree >= tree_size)
                .map_or_else(|| row_idx, |(idx, _)| idx + 1);
            let d_side_score = forest_map[row_idx + 1..n_rows]
                .iter()
                .map(|rows| rows[col_idx])
                .enumerate()
                .find(|(_, tree)| *tree >= tree_size)
                .map_or_else(|| n_rows - 1 - row_idx, |(idx, _)| idx + 1);
            top_score = std::cmp::max(
                top_score,
                l_side_score * r_side_score * u_side_score * d_side_score,
            );
        }
    }

    println!("Top visibility score: {}", top_score);

    Ok(())
}
