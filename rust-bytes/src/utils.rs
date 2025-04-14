pub(crate) fn section(name: &str) {
  let header = vec!["*"; 40];
  let line: String = header.join("");

  println!("\n{line}");
  println!("{name}");
  println!("{line}\n");
}
