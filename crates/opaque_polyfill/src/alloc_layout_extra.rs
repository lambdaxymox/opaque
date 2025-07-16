use core::alloc;
use core::num::NonZero;
use core::ptr::NonNull;

/// Creates a `NonNull` that is dangling, but well-aligned for this Layout.
///
/// Note that the pointer value may potentially represent a valid pointer,
/// which means this must not be used as a "not yet initialized"
/// sentinel value. Types that lazily allocate must track initialization by
/// some other means.
#[must_use]
#[inline]
pub const fn dangling(slf: &alloc::Layout) -> NonNull<u8> {
    NonNull::without_provenance(unsafe { NonZero::new_unchecked(slf.align()) })
}
