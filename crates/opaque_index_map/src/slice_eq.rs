pub(crate) fn slice_eq<T, U, F>(this: &[T], that: &[U], eq: F) -> bool
where
    F: Fn(&T, &U) -> bool,
{
    if this.len() != that.len() {
        return false;
    }

    // PERFORMANCE: Bounds checks are optimized away.
    for i in 0..this.len() {
        if !eq(&this[i], &that[i]) {
            return false;
        }
    }

    true
}
