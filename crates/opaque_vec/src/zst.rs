use core::mem;

#[inline(always)]
pub(crate) const fn is_zst<T>() -> bool {
    mem::size_of::<T>() == 0
}
