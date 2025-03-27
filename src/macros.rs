#[macro_export]
macro_rules! set {
    ($($s:expr),*) => ({
        HashSet::from([$($s),*])
    });
}
