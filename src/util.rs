

// string to char array
pub fn stc(s: &str) -> Vec<char> {
  s.chars().collect::<Vec<char>>()
}

// char array to string
pub fn cts(c: &[char]) -> String {
  c.iter().collect::<String>()
}