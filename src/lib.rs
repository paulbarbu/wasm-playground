mod utils;

use std::fmt::{Display, Formatter};

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

use js_sys::Math;

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
    cells: FixedBitSet,
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

#[cfg(feature = "binary")]
impl Universe{
    pub fn rand(width: u32, height: u32) -> Self {
        use rand::prelude::*;
        
        let mut cells = FixedBitSet::with_capacity((width * height) as usize);

        for i in 0..cells.len(){
            
            let v = match rand::random::<bool>() {
                true => true,
                false => false
            };
            cells.set(i, v);
        }
        
        Self {
            width: width,
            height: height,
            cells: cells,
        }
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
                    (true, 0 | 1) => {
                        //underpopulation
                        next.set(i, false);
                    }
                    (true, 2 | 3) => {
                        //continues to live
                        next.set(i, true);
                    }
                    (true, _) => {
                        //overpopulation
                        next.set(i, false);
                    }
                    (false, 3) => {
                        //reproduction
                        next.set(i, true);
                    }
                    _ => {}
                }
            }
        }

        self.cells = next;
    }

    pub fn new(width: u32, height: u32) -> Self {
        // create a line
        let mut cells = FixedBitSet::with_capacity((width * height) as usize);
        let i = (((width * height) / 2) + width / 2) as usize;
        cells.set(i - 1, true);
        cells.set(i, true);
        cells.set(i + 1, true);
        Self {
            width: width,
            height: height,
            cells: cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn cells(&self) -> *const usize{
        self.cells.as_slice().as_ptr()
    }
    
    #[cfg(not(feature = "binary"))]
    pub fn rand(width: u32, height: u32) -> Self {  
        let mut cells = FixedBitSet::with_capacity((width * height) as usize);

        for i in 0..cells.len(){
            
            let v = if Math::random() < 0.5 {
                true
            }
            else {
                false
            };
            cells.set(i, v);

        }
        
        Self {
            width: width,
            height: height,
            cells: cells,
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);

                let cell = self.cells[i];
                let out = match cell {
                    true => "1 ",
                    false => "0 ",
                };
                write!(f, "{}", out)?
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue
{
    wasm_bindgen::memory()
}