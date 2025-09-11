use color_eyre::eyre::Result;
use std::collections::HashSet;
use ulid::Ulid;

mod bitflags;
mod brands;
mod errors;
mod macros;
mod multithreading;
mod options;
mod typestate;
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

  // Create a new locked password manager
  utils::section("Typestate pattern");

  let pm = typestate::PasswordManager::new();
  match pm.unlock("wrong_password") {
    Ok(_) => unreachable!(),
    Err(e) => println!("Failed to unlock as expected: {}", e),
  }

  // Create another one and unlock it properly
  let pm = typestate::PasswordManager::new();
  let mut pm = pm.unlock("correct_master_password").unwrap();

  // Now we can use the unlocked methods
  pm.set_password("gmail".to_string(), "super_secure_123".to_string());
  pm.set_password("github".to_string(), "another_password".to_string());

  // Get passwords
  if let Some(password) = pm.get_password("gmail") {
    println!("Gmail password: {}", password);
  }

  // List all services
  println!("Stored services: {:?}", pm.list_services());

  // Lock it again
  let pm = pm.lock();

  // These would be compile errors if uncommented:
  // pm.set_password("test".to_string(), "password".to_string()); // Error!
  // pm.get_password("gmail"); // Error!

  let mut pm = pm.unlock("correct_master_password").unwrap();

  // remove a password
  pm.remove_password("github");

  // And the data persists
  if let Some(password) = pm.get_password("gmail") {
    println!("Gmail password after unlock: {}", password);
  }

  utils::section("Filter map to remove None values");
  let nums = vec!["10", "20", "abc", "30", "def", "40"];
  let nums_safe = utils::parse_numbers(&nums);
  dbg!(nums_safe);

  match utils::parse_numbers_fail_fast(&nums) {
    Ok(nums) => println!("Parsed numbers: {:?}", nums),
    Err(e) => println!("Failed to parse numbers: {}", e),
  }

  Ok(())
}
