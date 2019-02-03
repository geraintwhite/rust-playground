pub fn repeat(c: char, n: i32) -> String {
  std::iter::repeat(c).take(n as usize).collect::<String>()
}
