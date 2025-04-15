use color_eyre::eyre::Result;
use std::collections::HashSet;
use ulid::Ulid;

mod bitflags;
mod brands;
mod errors;
mod macros;
mod multithreading;
mod options;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install()?;

  // passing option values as reference
  utils::section("Option reference");
  let mut num = Some(23);
  println!("current value: {:?}", num);
  options::set_value(num.as_mut());
  println!("new value: {:?}", num);

  // creating and using declarative macros across the project
  utils::section("Declarative macros");
  let set = macros::set!(1, 2, 3);
  println!("Set from macro: {:?}", set);

  // perform CPU intensive ops using Rayon and Tokio
  utils::section("Tokio and Rayon");
  let nums: Vec<usize> = vec![1; 1024 * 1024];
  let total = multithreading::parallel_sum(&nums).await;
  println!("Compute intensive op: {}", total);

  // branded types
  utils::section("Branded types");
  let api_user_id = Ulid::new();
  let user_id = brands::UserId(api_user_id.into());

  // deref ensures we can directly access the type
  println!("User id is: {}", *user_id);

  // simple error handling
  utils::section("Error handling");
  let result = errors::divide(12, 2)?;
  println!("Quotient: {}", result);

  // assertion-type error handling
  utils::section("Assertion-type error handling");
  let result = errors::divide_assert(10, 2)?;
  println!("Quotient: {}", result);

  // bitflags
  utils::section("Bitflags");
  let mut file_access = bitflags::FileAccess::new(bitflags::Permission::Read.into());
  println!(
    "Has read access: {}",
    file_access.has(bitflags::Permission::Read)
  );

  file_access.set(bitflags::Permission::Write);
  println!(
    "Has write access: {} and read access: {}",
    file_access.has(bitflags::Permission::Write),
    file_access.has(bitflags::Permission::Read)
  );

  file_access.toggle(bitflags::Permission::Read);
  println!(
    "Read access present: {}",
    file_access.has(bitflags::Permission::Read)
  );

  file_access.clear(bitflags::Permission::Write);
  println!(
    "Write access present: {}",
    file_access.has(bitflags::Permission::Write)
  );

  Ok(())
}
