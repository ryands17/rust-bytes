pub fn set_value(val: Option<&mut i32>) {
  if let Some(v) = val {
    *v += 2;
  }
}
