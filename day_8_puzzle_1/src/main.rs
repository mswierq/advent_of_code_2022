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

    let mut num_of_visible = 0;
    let n_rows = forest_map.len();
    let n_cols = forest_map[0].len();
    for row_idx in 0..n_rows {
        for col_idx in 0..n_cols {
            if row_idx == 0 || row_idx == n_rows - 1 || col_idx == 0 || col_idx == n_cols - 1 {
                num_of_visible += 1;
            } else {
                let tree_size = forest_map[row_idx][col_idx];
                if forest_map[row_idx][0..col_idx]
                    .iter()
                    .all(|tree| *tree < tree_size) // visible from left?
                    || forest_map[row_idx][col_idx + 1..n_cols]
                        .iter()
                        .all(|tree| *tree < tree_size) // visible from right
                    || forest_map[0..row_idx]
                        .iter()
                        .map(|rows| rows[col_idx])
                        .all(|tree| tree < tree_size) // visible from up
                    || forest_map[row_idx + 1..n_rows]
                        .iter()
                        .map(|rows| rows[col_idx])
                        .all(|tree| tree < tree_size) // visible from down
                {
                    num_of_visible += 1;
                }
            }
        }
    }

    println!("N visible trees: {}", num_of_visible);

    Ok(())
}
