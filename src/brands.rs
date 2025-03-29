use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd)]
pub(crate) struct UserId(pub(crate) String);

impl Deref for UserId {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for UserId {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
