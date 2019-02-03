fn repeat(c: char, n: i32) -> String {
  std::iter::repeat(c).take(n as usize).collect::<String>()
}

fn fibo(n: i32) -> (i32, i32) {
  match n {
    n if n == 0 => (0, 1),
    _ => {
      let (a, b) = fibo(n / 2);
      let c = a * (b * 2 - a);
      let d = a * a + b * b;
      if n % 2 == 0 { (c, d) } else { (d, c + d) }
    }
  }
}

fn fibo_iter() -> Box<Iterator<Item = i32>> {
  Box::new((0..).map(|x| fibo(x).0))
}

fn main() {
  fibo_iter()
    .take(25)
    .for_each(|x| println!("{} {}", x, repeat('.', x)));
}
