use super::WorldConfig;
use std::{error::Error, fmt, fs};

pub struct World {
    config: WorldConfig,
    generation: usize,
    cells: Vec<Vec<u8>>,
}

impl World {
    pub fn new(config: WorldConfig) -> World {
        let cells = config.seed.clone();
        World {
            config,
            generation: 0,
            cells,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) -> Result<(), &'static str> {
        if x > self.config.width {
            return Err("cannot set for x that is out of bounds");
        }

        if y > self.config.height {
            return Err("cannot set for y that is out of bounds");
        }

        if alive {
            self.cells[x][y] = 1;
        } else {
            self.cells[x][y] = 0;
        }
        Ok(())
    }

    pub fn tick(&mut self) {
        let mut new_cells = vec![vec![0 as u8; self.config.width]; self.config.height];

        for x in 0..self.config.width {
            for y in 0..self.config.height {
                // Make signed versions of indicies
                let ix = isize::try_from(x).unwrap();
                let iy = isize::try_from(y).unwrap();

                let adjecent_cells = [
                    (ix - 1, iy - 1),
                    (ix, iy - 1),
                    (ix + 1, iy - 1),
                    (ix - 1, iy),
                    (ix + 1, iy),
                    (ix - 1, iy + 1),
                    (ix, iy + 1),
                    (ix + 1, iy + 1),
                ];

                // Count alive adjecent cells.
                let num_friends: u8 = adjecent_cells
                    .into_iter()
                    .map(|(x, y)| -> u8 {
                        match self.cells.get(x as usize) {
                            None => 0,
                            Some(row) => match row.get(y as usize) {
                                None => 0,
                                Some(v) => *v,
                            },
                        }
                    })
                    .sum();

                let was_alive = self.cells[x][y] == 1;
                let mut cell: u8 = 0;

                // Any live cell with two or three live neighbours survives.
                if was_alive && (num_friends == 2 || num_friends == 3) {
                    cell = 1;
                }

                // Any dead cell with three live neighbours becomes a live cell.
                if !was_alive && num_friends == 3 {
                    cell = 1;
                }

                new_cells[x][y] = cell
            }
        }
        self.cells = new_cells;
        self.generation += 1;
    }

    pub fn to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        fs::write(filename, self.to_string()).or_else(|e| Err(e.into()))
    }

    pub fn from_file(filename: &str) -> Result<World, Box<dyn Error>> {
        Ok(World::new(WorldConfig::from_file(filename)?))
    }

    pub fn to_string(&self) -> String {
        let mut string = String::with_capacity(self.config.width * self.config.height);
        for row in self.cells.iter() {
            for cell in row.iter() {
                match *cell {
                    1 => string.push('#'),
                    _ => string.push('.'),
                }
            }
            string.push_str("\n");
        }
        string
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::with_capacity(self.config.width * self.config.height);

        for row in self.cells.iter() {
            for cell in row.iter() {
                match *cell {
                    1 => string.push('#'),
                    _ => string.push('.'),
                }
            }
            string.push_str("\n");
        }

        write!(f, "{}", string)
    }
}
