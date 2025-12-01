mod utils;

use std::fmt::{Display, Formatter};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(format!("Hello, wasm-game-of-life! {} ", s).as_str());
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn num_alive_neighbours(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighour_row = (row + delta_row) % self.height;
                let neighour_col = (col + delta_col) % self.width;

                count += self.cells[self.get_index(neighour_row, neighour_col)] as u8;
            }
        }
        return count;
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);

                let cell = self.cells[i];
                let alive_neighbours = self.num_alive_neighbours(row, col);
                match (cell, alive_neighbours) {
                    (Cell::Alive, 0 | 1) => {
                        //underpopulation
                        next[i] = Cell::Dead;
                    }
                    (Cell::Alive, 2 | 3) => {
                        //continues to live
                        next[i] = Cell::Alive;
                    }
                    (Cell::Alive, _) => {
                        //overpopulation
                        next[i] = Cell::Dead;
                    }
                    (Cell::Dead, 3) => {
                        //reproduction
                        next[i] = Cell::Alive;
                    }
                    _ => {}
                }
            }
        }

        self.cells = next;
    }

    pub fn new(width: u32, height: u32) -> Self{
        // set the middle cell to 1
        let mut cells   = vec![Cell::Dead; (width*height) as usize];
        let i = (((width * height) / 2) + width/2) as usize;
        cells[i-1] = Cell::Alive;
        cells[i] = Cell::Alive;
        cells[i+1] = Cell::Alive;
        Self {width: width, height: height, cells: cells}
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);

                let cell = self.cells[i];
                match cell {
                    Cell::Alive => write!(f, "1 "),
                    Cell::Dead => write!(f, "0 "),
                }?
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}
