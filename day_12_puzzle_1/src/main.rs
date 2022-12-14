use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::{fs::File, io};

struct Graph {
    nodes: Vec<Vec<usize>>,
    start: usize,
    destination: usize,
    distances: Vec<usize>,
}

fn calc_height(field: char) -> i32 {
    if field == 'S' {
        return 0;
    }
    if field == 'E' {
        return 'z' as i32 - 'a' as i32;
    }
    return field as i32 - 'a' as i32;
}

impl Graph {
    fn new(height_map: Vec<Vec<char>>) -> Self {
        let n_rows = height_map.len();
        let n_cols = height_map
            .first()
            .expect("Expected at least one row!")
            .len();
        let mut nodes = vec![vec![]; n_rows * n_cols];
        let distances = vec![0; n_rows * n_cols];

        let mut start = 0;
        let mut destination = 0;

        for (row_idx, row) in height_map.iter().enumerate() {
            for (col_idx, field) in row.iter().enumerate() {
                let field_idx = row_idx * n_cols + col_idx;
                if *field == 'S' {
                    start = field_idx;
                }
                if *field == 'E' {
                    destination = field_idx;
                }
                let field_height = calc_height(*field);

                if row_idx > 0 {
                    //up
                    let next_field = height_map[row_idx - 1][col_idx];
                    let height_diff = calc_height(next_field) - field_height;
                    if height_diff.abs() <= 1 {
                        let next_field_idx = (row_idx - 1) * n_cols + col_idx;
                        nodes[field_idx].push(next_field_idx);
                    }
                }

                if row_idx < (n_rows - 1) {
                    //down
                    let next_field = height_map[row_idx + 1][col_idx];
                    let height_diff = calc_height(next_field) - field_height;
                    if height_diff.abs() <= 1 {
                        let next_field_idx = (row_idx + 1) * n_cols + col_idx;
                        nodes[field_idx].push(next_field_idx);
                    }
                }

                if col_idx > 0 {
                    //left
                    let next_field = height_map[row_idx][col_idx - 1];
                    let height_diff = calc_height(next_field) - field_height;
                    if height_diff.abs() <= 1 {
                        let next_field_idx = row_idx * n_cols + col_idx - 1;
                        nodes[field_idx].push(next_field_idx);
                    }
                }

                if col_idx < (n_cols - 1) {
                    //right
                    let next_field = height_map[row_idx][col_idx + 1];
                    let height_diff = calc_height(next_field) - field_height;
                    if height_diff.abs() <= 1 {
                        let next_field_idx = row_idx * n_cols + col_idx + 1;
                        nodes[field_idx].push(next_field_idx);
                    }
                }
            }
        }

        Self {
            nodes,
            start,
            destination,
            distances,
        }
    }

    fn find_shortest_path(&mut self) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back(self.start);

        let mut visited = vec![false; self.nodes.len()];
        visited[self.start] = true;

        while !queue.is_empty() {
            let node_idx = queue.pop_front().unwrap();
            for next_node in &self.nodes[node_idx] {
                if !visited[*next_node] {
                    visited[*next_node] = true;
                    self.distances[*next_node] = self.distances[node_idx] + 1;
                    queue.push_back(*next_node);

                    if *next_node == self.destination {
                        return self.distances[self.destination];
                    }
                }
            }
        }

        let mut visited_map = vec![vec!['.'; 171]; 41];
        for (i, v) in visited.iter().enumerate() {
            let row = i / 171;
            let col = i % 171;
            if *v {
                visited_map[row][col] = '#';
            }
        }

        visited_map[self.start / 171][self.start % 171] = 'S';
        visited_map[self.destination / 171][self.destination % 171] = 'E';

        for row in visited_map {
            println!("{}", row.iter().collect::<String>());
        }

        0
    }
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let height_map = reader
        .lines()
        .map(|row| row.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut graph = Graph::new(height_map);

    println!("Shortest path is {} length", graph.find_shortest_path());

    Ok(())
}
