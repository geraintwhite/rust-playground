mod fibo;
mod string;

fn main() {
  fibo::fibo_iter()
    .take(10)
    .for_each(|x| println!("{} {}", x, string::repeat('.', x)));
}
