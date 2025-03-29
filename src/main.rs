use brands::UserId;
use color_eyre::eyre::Result;
use std::collections::HashSet;
use ulid::Ulid;

mod brands;
mod macros;
mod multithreading;
mod options;

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install()?;

  // passing option values as reference
  let mut num = Some(23);
  println!("current value: {:?}", num);
  options::set_value(num.as_mut());
  println!("new value: {:?}", num);

  // creating and using declarative macros across the project
  let set = macros::set!(1, 2, 3);
  println!("Set from macro: {:?}", set);

  // perform CPU intensive ops using Rayon and Tokio
  let nums: Vec<usize> = vec![1; 1024 * 1024];
  let total = multithreading::parallel_sum(&nums).await;
  println!("Compute intensive op: {}", total);

  // branded types
  let api_user_id = Ulid::new();
  let user_id = UserId(api_user_id.into());

  // deref ensures we can directly access the type
  println!("User id is: {}", *user_id);

  Ok(())
}
