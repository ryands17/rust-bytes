use color_eyre::eyre::{Result, eyre};
use std::collections::HashMap;
use std::marker::PhantomData;

pub(crate) struct PasswordManager<State> {
  passwords: HashMap<String, String>,
  state: PhantomData<State>,
}

pub(crate) struct Locked;
pub(crate) struct Unlocked;

impl PasswordManager<Locked> {
  /// Create a new locked password manager
  pub(crate) fn new() -> PasswordManager<Locked> {
    PasswordManager {
      passwords: HashMap::new(),
      state: PhantomData,
    }
  }

  /// Unlock the password manager with a master password
  pub(crate) fn unlock(self, master_password: &str) -> Result<PasswordManager<Unlocked>> {
    // In a real implementation, you'd verify the master password
    if master_password == "correct_master_password" {
      eprintln!("Password manager unlocked!");
      Ok(PasswordManager {
        passwords: self.passwords,
        state: PhantomData,
      })
    } else {
      Err(eyre!("Invalid master password"))
    }
  }
}

impl PasswordManager<Unlocked> {
  /// Set a password for a given name/service
  pub(crate) fn set_password(&mut self, name: String, password: String) {
    println!("Setting password for: {}", name);
    self.passwords.insert(name, password);
  }

  /// Get a password for a given name/service
  pub(crate) fn get_password(&self, name: &str) -> Option<&String> {
    self.passwords.get(name)
  }

  /// List all stored service names
  pub(crate) fn list_services(&self) -> Vec<&String> {
    self.passwords.keys().collect()
  }

  /// Lock the password manager
  pub(crate) fn lock(self) -> PasswordManager<Locked> {
    println!("Password manager locked!");
    PasswordManager {
      passwords: self.passwords,
      state: PhantomData,
    }
  }

  /// Remove a password entry
  pub(crate) fn remove_password(&mut self, name: &str) -> Option<String> {
    self.passwords.remove(name)
  }
}
