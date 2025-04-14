use color_eyre::eyre::{Ok, Result, eyre};

use crate::macros;

pub(crate) fn divide(a: usize, b: usize) -> Result<usize> {
  if b == 0 {
    Err(eyre!("Cannot divide by zero"))
  } else {
    Ok(a / b)
  }
}

pub(crate) fn divide_assert(a: usize, b: usize) -> Result<usize> {
  let err_msg = "divisor must be positive";
  macros::c_assert!(b > 0, eyre!(err_msg), "{err_msg}");
  Ok(a / b)
}
