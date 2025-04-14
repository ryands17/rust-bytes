use deref_macro::{DerefImpl, DerefMutImpl};

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, DerefImpl, DerefMutImpl)]
pub(crate) struct UserId(pub(crate) String);
