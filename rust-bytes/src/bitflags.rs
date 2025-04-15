use enumflags2::{BitFlags, bitflags};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Permission {
  Read = 1,
  Write = 1 << 1,
  Execute = 1 << 2,
}
#[repr(transparent)]
pub(crate) struct FileAccess(BitFlags<Permission>);

impl FileAccess {
  pub(crate) fn new(flag: BitFlags<Permission>) -> Self {
    Self(flag)
  }

  pub(crate) fn set(&mut self, flag: Permission) {
    self.0.insert(flag);
  }

  pub(crate) fn toggle(&mut self, flag: Permission) {
    self.0.toggle(flag);
  }

  pub(crate) fn clear(&mut self, flag: Permission) {
    self.0.remove(flag);
  }

  pub(crate) fn has(&self, flag: Permission) -> bool {
    self.0.contains(flag)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use enumflags2::BitFlags;

  #[test]
  fn test_new_file_access() {
    let access = FileAccess::new(BitFlags::empty());
    assert!(!access.has(Permission::Read));
    assert!(!access.has(Permission::Write));
    assert!(!access.has(Permission::Execute));
  }

  #[test]
  fn test_set_permission() {
    let mut access = FileAccess::new(BitFlags::empty());
    access.set(Permission::Read);
    assert!(access.has(Permission::Read));
    assert!(!access.has(Permission::Write));
  }

  #[test]
  fn test_clear_permission() {
    let mut access = FileAccess::new(BitFlags::all());
    access.clear(Permission::Write);
    assert!(!access.has(Permission::Write));
    assert!(access.has(Permission::Read));
    assert!(access.has(Permission::Execute));
  }

  #[test]
  fn test_toggle_permission_on() {
    let mut access = FileAccess::new(BitFlags::empty());
    access.toggle(Permission::Execute);
    assert!(access.has(Permission::Execute));
  }

  #[test]
  fn test_toggle_permission_off() {
    let mut access = FileAccess::new(BitFlags::all());
    access.toggle(Permission::Execute);
    assert!(!access.has(Permission::Execute));
  }

  #[test]
  fn test_combined_permissions() {
    let mut access = FileAccess::new(BitFlags::empty());
    access.set(Permission::Read);
    access.set(Permission::Write);
    assert!(access.has(Permission::Read));
    assert!(access.has(Permission::Write));
    assert!(!access.has(Permission::Execute));
  }
}
