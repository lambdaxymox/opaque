//! TODO: This is a temporary polyfill, copied from the Rust standard library.
//! Remove this method once the `slice_range` feature has stabilized.
//! Tracking issue: [rust-lang/rust#76393](https://github.com/rust-lang/rust/issues/76393)
use core::ops;

// #[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
// #[cfg_attr(feature = "panic_immediate_abort", inline)]
#[inline(never)]
#[cold]
#[track_caller]
fn slice_end_index_len_fail(index: usize, len: usize) -> ! {
    /*
    const_panic!(
        "slice end index is out of range for slice",
        "range end index {index} out of range for slice of length {len}",
        index: usize,
        len: usize,
    )
    */
    panic!(
        "slice end index is out of range for slice: range end index {} out of range for slice of length {}",
        index, len,
    )
}

// #[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
// #[cfg_attr(feature = "panic_immediate_abort", inline)]
#[inline(never)]
#[cold]
#[track_caller]
fn slice_index_order_fail(index: usize, end: usize) -> ! {
    /*
    const_panic!(
        "slice index start is larger than end",
        "slice index starts at {index} but ends at {end}",
        index: usize,
        end: usize,
    )
    */
    panic!(
        "slice index start is larger than end: slice index starts at {} but ends at {}",
        index, end,
    )
}

// #[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
// #[cfg_attr(feature = "panic_immediate_abort", inline)]
#[inline(never)]
#[cold]
#[track_caller]
const fn slice_start_index_overflow_fail() -> ! {
    panic!("attempted to index slice from after maximum usize");
}

// #[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
// #[cfg_attr(feature = "panic_immediate_abort", inline)]
#[inline(never)]
#[cold]
#[track_caller]
const fn slice_end_index_overflow_fail() -> ! {
    panic!("attempted to index slice up to maximum usize");
}

/// Performs bounds checking of a range.
///
/// This method is similar to [`Index::index`] for slices, but it returns a
/// [`Range`] equivalent to `range`. You can use this method to turn any range
/// into `start` and `end` values.
///
/// `bounds` is the range of the slice to use for bounds checking. It should
/// be a [`RangeTo`] range that ends at the length of the slice.
///
/// The returned [`Range`] is safe to pass to [`slice::get_unchecked`] and
/// [`slice::get_unchecked_mut`] for slices with the given range.
///
/// [`Range`]: ops::Range
/// [`RangeTo`]: ops::RangeTo
/// [`slice::get_unchecked`]: slice::get_unchecked
/// [`slice::get_unchecked_mut`]: slice::get_unchecked_mut
///
/// # Panics
///
/// Panics if `range` would be out of bounds.
///
///
/// TODO: This is a temporary polyfill, copied from the Rust standard library.
/// Remove this method once the `slice_range` feature has stabilized.
/// Tracking issue: [rust-lang/rust#76393](https://github.com/rust-lang/rust/issues/76393)
#[track_caller]
#[must_use]
pub fn range<R>(range: R, bounds: ops::RangeTo<usize>) -> ops::Range<usize>
where
    R: ops::RangeBounds<usize>,
{
    let len = bounds.end;

    let start = match range.start_bound() {
        ops::Bound::Included(&start) => start,
        ops::Bound::Excluded(start) => start.checked_add(1).unwrap_or_else(|| slice_start_index_overflow_fail()),
        ops::Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        ops::Bound::Included(end) => end.checked_add(1).unwrap_or_else(|| slice_end_index_overflow_fail()),
        ops::Bound::Excluded(&end) => end,
        ops::Bound::Unbounded => len,
    };

    if start > end {
        slice_index_order_fail(start, end);
    }
    if end > len {
        slice_end_index_len_fail(end, len);
    }

    ops::Range { start, end }
}
