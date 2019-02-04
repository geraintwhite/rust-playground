use std::fmt;
use super::string;

const SIZE: usize = 3;
const CELLS: usize = SIZE * SIZE * SIZE * SIZE;
const ERROR: &str = "Invalid Sudoku string provided";

pub struct Sudoku {
  grid: [u8; CELLS]
}

impl Sudoku {
  pub fn from_str(s: &str) -> Sudoku {
    let mut grid = [0; CELLS];

    if s.len() != CELLS {
      panic!("{}. Expected length: {}, received length: {}.", ERROR, CELLS, s.len());
    }

    for (i, c) in s.chars().enumerate() {
      grid[i] = match c {
        '0'...'9' => c.to_digit(10).unwrap() as u8,
        '.' | '_' => 0,
        _ => panic!("{}. Unknown character received: {}.", ERROR, c)
      };
    }

    Sudoku { grid }
  }
}

impl fmt::Display for Sudoku {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut output: String = "".to_owned();

    for rowgrp in 0..SIZE {
      for row in 0..SIZE {
        for colgrp in 0..SIZE {
          for col in 0..SIZE {
            let index = (rowgrp * SIZE + row) * SIZE * SIZE + (colgrp * SIZE + col);
            output.push_str(&format!(" {} ", self.grid[index].to_string()));
          }
          if colgrp < SIZE-1 {
            output.push_str("|");
          }
        }
        output.push_str("\n");
        if rowgrp < SIZE-1 && row == SIZE-1 {
          let length = (SIZE * SIZE * 3 + SIZE - 3) as i32;
          output.push_str(&format!(" {} \n", string::repeat('-', length)));
        }
      }
    }

    write!(f, "{}", output)
  }
}
