macro_rules! set {
    ($($s:expr),*) => ({
        HashSet::from([$($s),*])
    });
}

pub(crate) use set;
