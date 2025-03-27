use rust_bytes::options;
use std::collections::HashSet;

fn main() {
  // passing option values as reference
  let mut num = Some(23);
  println!("current value: {:?}", num);
  options::set_value(num.as_mut());
  println!("new value: {:?}", num);

  // creating and using declarative macros across the project
  let set = rust_bytes::set!(1, 2, 3);
  println!("Set from macro: {:?}", set)
}
