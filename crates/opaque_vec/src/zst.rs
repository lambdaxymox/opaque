#[inline(always)]
pub(crate) const fn is_zst<T>() -> bool {
    core::mem::size_of::<T>() == 0
}
