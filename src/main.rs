mod fibo;
mod string;
mod sudoku;

use sudoku::Sudoku;

fn main() {
  fibo::fibo_iter()
    .take(10)
    .for_each(|x| println!("{} {}", x, string::repeat('.', x)));

  let string = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
  let sudoku = Sudoku::from_str(string);

  println!("\n{}", sudoku);
}
