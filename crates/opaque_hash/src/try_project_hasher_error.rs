use core::any;
use core::error;
use core::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TryProjectHasherErrorKind {
    Hasher,
}

/// The error type for the [`try_as_proj`], [`try_as_proj_mut`], [`try_into_proj`] on the
/// [`TypeErasedHasher`] type.
///
/// [`try_as_proj`]: TypeErasedHasher::try_as_proj
/// [`try_as_proj_mut`]: TypeErasedHasher::try_as_proj_mut
/// [`try_into_proj`]: TypeErasedHasher::try_into_proj
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryProjectHasherError {
    kind: TryProjectHasherErrorKind,
    expected: any::TypeId,
    result: any::TypeId,
}

impl TryProjectHasherError {
    /// Constructs a new type projection error.
    #[inline]
    pub(crate) const fn new(kind: TryProjectHasherErrorKind, expected: any::TypeId, result: any::TypeId) -> Self {
        Self { kind, expected, result }
    }

    /// Returns which data type did not match.
    #[inline]
    pub const fn kind(&self) -> TryProjectHasherErrorKind {
        self.kind
    }

    /// Returns the [`TypeId`] of the expected type to perform the type projection.
    #[inline]
    pub const fn expected(&self) -> any::TypeId {
        self.expected
    }

    /// Returns the [`TypeId`] of the provided type to perform the type projection.
    #[inline]
    pub const fn result(&self) -> any::TypeId {
        self.result
    }
}

impl fmt::Display for TryProjectHasherError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TryProjectHasherErrorKind::Hasher => write!(
                formatter,
                "Type projection failed for hasher type: expected type with type id `{:?}`, but got `{:?}`",
                self.expected, self.result
            ),
        }
    }
}

impl error::Error for TryProjectHasherError {}
