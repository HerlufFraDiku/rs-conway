use std::{error::Error, fs};

pub struct WorldConfig {
    pub width: usize,
    pub height: usize,
    pub seed: Vec<Vec<u8>>,
}

impl WorldConfig {
    pub fn new(width: usize, height: usize, seed: Vec<Vec<u8>>) -> WorldConfig {
        WorldConfig {
            width,
            height,
            seed,
        }
    }

    pub fn zero(width: usize, height: usize) -> WorldConfig {
        WorldConfig {
            width,
            height,
            seed: vec![vec![0; width]; height],
        }
    }

    pub fn from_file(filename: &str) -> Result<WorldConfig, Box<dyn Error>> {
        // Read file and split it into lines
        let contents = fs::read_to_string(filename)?;
        let lines: Vec<Vec<char>> = contents
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        // Make a set from
        let mut widths: Vec<usize> = lines.iter().map(|line| line.len()).collect();
        widths.sort();
        widths.dedup();
        if widths.len() != 1 {
            return Err("Rows must be uniform length".into());
        }

        // We can now determine dimensions for world config
        let height = lines.len();
        let width = widths[0];

        // Read the seed from file
        let mut seed = vec![vec![0; width]; height];
        for x in 0..height {
            for y in 0..width {
                if ['#', '1'].contains(&lines[x][y]) {
                    seed[x][y] = 1;
                } else {
                    seed[x][y] = 0;
                }
            }
        }

        Ok(WorldConfig {
            height,
            width,
            seed,
        })
    }
}
