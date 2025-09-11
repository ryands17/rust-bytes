use color_eyre::eyre::Result;

pub(crate) fn section(name: &str) {
  let header = vec!["*"; 40];
  let line: String = header.join("");

  println!("\n{line}");
  println!("{name}");
  println!("{line}\n");
}

pub(crate) fn parse_numbers(input: &[&str]) -> Vec<i32> {
  input.iter().filter_map(|s| s.parse::<i32>().ok()).collect()
}

// input type can also be Vec<Result<&str, _>>
pub(crate) fn parse_numbers_fail_fast(strings: &[&str]) -> Result<Vec<i32>> {
  strings
    .into_iter()
    .map(|s| s.parse::<i32>().map_err(Into::into))
    .collect()
}
