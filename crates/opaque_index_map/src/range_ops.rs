// Portions of this file are derived from `indexmap`,
// Copyright (c) 2016--2017 The indexmap Developers
// Licensed under either of
//   * Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
//   * MIT license (http://opensource.org/licenses/MIT)
// at your option.
use core::ops;

pub(crate) fn try_simplify_range<R>(range: R, len: usize) -> Option<ops::Range<usize>>
where
    R: ops::RangeBounds<usize>,
{
    let start = match range.start_bound() {
        ops::Bound::Unbounded => 0,
        ops::Bound::Included(&i) if i <= len => i,
        ops::Bound::Excluded(&i) if i < len => i + 1,
        _ => return None,
    };
    let end = match range.end_bound() {
        ops::Bound::Unbounded => len,
        ops::Bound::Excluded(&i) if i <= len => i,
        ops::Bound::Included(&i) if i < len => i + 1,
        _ => return None,
    };

    if start > end {
        return None;
    }

    Some(start..end)
}

#[track_caller]
pub(crate) fn simplify_range<R>(range: R, len: usize) -> ops::Range<usize>
where
    R: ops::RangeBounds<usize>,
{
    let start = match range.start_bound() {
        ops::Bound::Unbounded => 0,
        ops::Bound::Included(&i) if i <= len => i,
        ops::Bound::Excluded(&i) if i < len => i + 1,
        ops::Bound::Included(i) | ops::Bound::Excluded(i) => {
            panic!("range start index {i} out of range for slice of length {len}")
        }
    };
    let end = match range.end_bound() {
        ops::Bound::Unbounded => len,
        ops::Bound::Excluded(&i) if i <= len => i,
        ops::Bound::Included(&i) if i < len => i + 1,
        ops::Bound::Included(i) | ops::Bound::Excluded(i) => {
            panic!("range end index {i} out of range for slice of length {len}")
        }
    };

    if start > end {
        panic!(
            "range start index {:?} should be <= range end index {:?}",
            range.start_bound(),
            range.end_bound()
        );
    }

    start..end
}
