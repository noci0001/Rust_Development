mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//        _____________________________
//       |                             |
//       |  AUTOMATIC FUNCTION EXPORT  |
//       |_____________________________|
//
// ===> it allows Rust functions to be called from JavaScript and vice versa
// ===> handles conversion of types between Rust and JavaScript
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
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    //RULE 1: Any live cell with fewer than two live neighbors dies bc of
                    // underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,

                    //RULE 2: Any live cell with two or three live neighbors lives on
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,

                    //RULE 3: Any live cell with more than three live neightbors dies bc of overpop
                    (Cell::Alive, x) => Cell::Alive,

                    //RULE 4: Any dead cell with exactly three live neighbors becomes a live cell
                    //due  to reproduction
                    (Cell::Dead, 3) => Cell::Alive,

                    //Otherwise, cells remains in the same state as before
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..height * width)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) ->  String {
        self.to_string()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '▫' } else { '▪' };
                write!(f, "{}", symbol)?;
            }

            write!(f, "\n");
        }
        Ok(())
    }
}