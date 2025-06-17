use core::error;
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetDisjointMutError {
    /// An index provided was out of bounds for the index map.
    IndexOutOfBounds,
    /// Two indices provided overlapped.
    OverlappingIndices,
}

impl fmt::Display for GetDisjointMutError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            GetDisjointMutError::IndexOutOfBounds => "an index is out of bounds",
            GetDisjointMutError::OverlappingIndices => "there were overlapping indices",
        };

        core::fmt::Display::fmt(msg, f)
    }
}

#[cfg(feature = "std")]
impl error::Error for GetDisjointMutError {}
