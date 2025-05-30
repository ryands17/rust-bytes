macro_rules! set {
    ($($s:expr),*) => ({
        HashSet::from([$($s),*])
    });
}

pub(crate) use set;

// assertions macro that only panics in debug builds
macro_rules! c_assert {
  ($cond:expr, $err:expr, $($arg:tt)+) => {{
    debug_assert!($cond, $($arg)+);
    if !$cond {
      return Err($err);
    }
  }};
}

pub(crate) use c_assert;
