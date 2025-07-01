use crate::hasher_inner::{OpaqueHasherInner, TypedProjHasherInner};

use core::any;
use core::fmt;
use core::marker;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

/// A type-projected hasher.
///
/// Wrapping the hasher like this allows us to type-erase and type-project hashers
/// as **O(1)** time operations. When passing references to type-projected or type-erased hashers
/// around, type-erasure and type-projection are zero-cost operations, since they have identical
/// layout.
///
/// For a given hasher type `H`, the [`TypedProjHasher<H>`] and [`OpaqueHasher`] data types also
/// implement the [`Hasher`] trait, so we can calculate hashes with it just as well as the
/// underlying hasher of type `H`.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Some applications of this include implementing
/// heterogeneous data structures, plugin systems, and managing foreign function interface data.
/// There are two data types that are dual to each other: [`TypedProjHasher`] and [`OpaqueHasher`].
///
/// # Tradeoffs Compared To A Non-Projected Hasher
///
/// There are some tradeoffs to gaining type-erasability and type-projectability. The projected and
/// erased hashers have identical memory layout to ensure that type projection and type erasure are
/// both **O(1)** time operations. Thus, the underlying hasher must be stored in the equivalent
/// of a [`Box`], which carries a small performance penalty. Moreover, the hashers must carry extra
/// metadata about the type of the underlying hasher through its [`TypeId`]. Boxing the hasher
/// imposes a small performance penalty at runtime, and the extra metadata makes the hasher itself
/// a little bigger in memory. This also puts a slight restriction on what kinds of hashers
/// can be held inside the container: the underlying hasher must be [`any::Any`], i.e. it must have
/// a `'static` lifetime.
///
/// # See Also
///
/// - [`OpaqueHasher`]: the type-erased counterpart to [`TypedProjHasher`].
///
/// # Examples
///
/// Using a typed-projected hasher.
///
/// ```
/// # use opaque_hash::TypedProjHasher;
/// # use std::any::TypeId;
/// # use std::hash::DefaultHasher;
/// #
/// let proj_hasher = TypedProjHasher::new(DefaultHasher::new());
///
/// assert_eq!(proj_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
/// ```
#[repr(transparent)]
pub struct TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    inner: TypedProjHasherInner<H>,
}

impl<H> TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    /// Returns the [`TypeId`] of the underlying hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypedProjHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let proj_hasher = TypedProjHasher::new(DefaultHasher::new());
    ///
    /// assert_eq!(proj_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// ```
    #[inline]
    pub const fn hasher_type_id(&self) -> any::TypeId {
        self.inner.hasher_type_id()
    }
}

impl<H> TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    /// Constructs a new type-projected hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypedProjHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let proj_hasher = TypedProjHasher::new(DefaultHasher::new());
    ///
    /// assert_eq!(proj_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// assert_ne!(proj_hasher.hasher_type_id(), TypeId::of::<Box<DefaultHasher>>());
    /// ```
    #[inline]
    pub fn new(hasher: H) -> Self {
        let inner = TypedProjHasherInner::new(hasher);

        Self { inner, }
    }

    /// Constructs a new type-projected hasher from a boxed hasher.
    ///
    /// The underlying type of the type-projected hasher will be the type of the hasher held by
    /// the box.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypedProjHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let proj_hasher = TypedProjHasher::from_boxed_hasher(Box::new(DefaultHasher::new()));
    ///
    /// assert_eq!(proj_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// assert_ne!(proj_hasher.hasher_type_id(), TypeId::of::<Box<DefaultHasher>>());
    ///
    /// // In contrast, a type-projected hasher constructed using `new` will have the boxed hasher
    /// // as the underlying hasher type.
    /// let proj_boxed_hasher = TypedProjHasher::new(Box::new(DefaultHasher::new()));
    ///
    /// assert_eq!(proj_boxed_hasher.hasher_type_id(), TypeId::of::<Box<DefaultHasher>>());
    /// assert_ne!(proj_boxed_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// ```
    #[inline]
    pub fn from_boxed_hasher(hasher: Box<H>) -> Self {
        let inner = TypedProjHasherInner::from_boxed_hasher(hasher);

        Self { inner, }
    }

    /// Returns a reference to the underlying hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypedProjHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let proj_hasher = TypedProjHasher::new(DefaultHasher::new());
    ///
    /// let hasher: &DefaultHasher = proj_hasher.hasher();
    /// ```
    pub fn hasher(&self) -> &H {
        self.inner.hasher_assuming_type()
    }

    /// Converts the type-projected hasher into a boxed hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::TypedProjHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let proj_hasher = TypedProjHasher::new(DefaultHasher::new());
    /// let boxed_hasher: Box<DefaultHasher> = proj_hasher.into_boxed_hasher();
    ///
    /// let new_proj_hasher = TypedProjHasher::from_boxed_hasher(boxed_hasher);
    ///
    /// assert_eq!(new_proj_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// assert_ne!(new_proj_hasher.hasher_type_id(), TypeId::of::<Box<DefaultHasher>>());
    /// ```
    pub fn into_boxed_hasher(self) -> Box<H> {
        self.inner.into_boxed_hasher_assuming_type()
    }
}

impl<H> hash::Hasher for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    fn finish(&self) -> u64 {
        self.inner.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.inner.write(bytes)
    }
}

impl<H> Clone for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<H> Default for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<H> fmt::Debug for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync + fmt::Debug,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TypedProjHasher")
            .field("inner", self.inner.hasher_assuming_type())
            .finish()
    }
}

impl<H> From<H> for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    fn from(hasher: H) -> Self {
        Self::new(hasher)
    }
}

/// A type-erased hasher.
///
/// Wrapping the hasher like this allows us to type-erase and type-project hashers
/// as **O(1)** time operations. When passing references to type-projected or type-erased hashers
/// around, type-erasure and type-projection are zero-cost operations, since they have identical
/// layout.
///
/// For a given hasher type `H`, the [`TypedProjHasher<H>`] and [`OpaqueHasher`] data types also
/// implement the [`Hasher`] trait, so we can calculate hashes with it just as well as the
/// underlying hasher of type `H`.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Some applications of this include implementing
/// heterogeneous data structures, plugin systems, and managing foreign function interface data.
/// There are two data types that are dual to each other: [`TypedProjHasher`] and [`OpaqueHasher`].
///
/// # Tradeoffs Compared To A Non-Projected Hasher
///
/// There are some tradeoffs to gaining type-erasability and type-projectability. The projected and
/// erased hashers have identical memory layout to ensure that type projection and type erasure are
/// both **O(1)** time operations. Thus, the underlying hasher must be stored in the equivalent
/// of a [`Box`], which carries a small performance penalty. Moreover, the hashers must carry extra
/// metadata about the type of the underlying hasher through its [`TypeId`]. Boxing the hasher
/// imposes a small performance penalty at runtime, and the extra metadata makes the hasher itself
/// a little bigger in memory. This also puts a slight restriction on what kinds of hashers
/// can be held inside the container: the underlying hasher must be [`any::Any`], i.e. it must have
/// a `'static` lifetime.
///
/// # See Also
///
/// - [`TypedProjHasher`]: the type-projected counterpart to [`OpaqueHasher`].
///
/// # Examples
///
/// Using a type-erased hasher.
///
/// ```
/// # use opaque_hash::OpaqueHasher;
/// # use std::any::TypeId;
/// # use std::hash::DefaultHasher;
/// #
/// let opaque_hasher = OpaqueHasher::new::<DefaultHasher>(DefaultHasher::new());
///
/// assert_eq!(opaque_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
/// ```
#[repr(transparent)]
pub struct OpaqueHasher {
    inner: OpaqueHasherInner,
}

impl OpaqueHasher {
    /// Returns the [`TypeId`] of the underlying hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::OpaqueHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let opaque_hasher = OpaqueHasher::new::<DefaultHasher>(DefaultHasher::new());
    ///
    /// assert_eq!(opaque_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// ```
    #[inline]
    pub const fn hasher_type_id(&self) -> any::TypeId {
        self.inner.hasher_type_id()
    }
}

impl OpaqueHasher {
    /// Determines whether the underlying hasher has the given hasher type.
    ///
    /// Returns `true` if `self` has the specified hasher type. Returns `false` otherwise.
    /// 
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::OpaqueHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let opaque_hasher = OpaqueHasher::new::<DefaultHasher>(DefaultHasher::new());
    ///
    /// assert!(opaque_hasher.has_hasher_type::<DefaultHasher>());
    /// ```
    #[inline]
    pub fn has_hasher_type<H>(&self) -> bool
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        self.inner.hasher_type_id() == any::TypeId::of::<H>()
    }

    /// Assert the concrete types underlying a type-erased data type.
    ///
    /// This method's main use case is ensuring the type safety of an operation before projecting
    /// into the type-projected counterpart of the type-erased hasher.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hasher of `self` do not match the requested hasher
    /// type `H`.
    #[inline]
    #[track_caller]
    fn assert_type_safety<H>(&self)
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        #[cold]
        #[cfg_attr(feature = "nightly", optimize(size))]
        #[track_caller]
        fn type_check_failed(type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_hasher_type::<H>() {
            type_check_failed(self.inner.hasher_type_id(), any::TypeId::of::<H>());
        }
    }
}

impl OpaqueHasher {
    /// Projects the type-erased hasher reference into a type-projected hasher reference.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hasher of `self` do not match the requested
    /// hasher type `H`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{OpaqueHasher, TypedProjHasher};
    /// # use std::hash::DefaultHasher;
    /// #
    /// let opaque_hasher = OpaqueHasher::new::<DefaultHasher>(DefaultHasher::new());
    /// #
    /// # assert!(opaque_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let proj_hasher: &TypedProjHasher<DefaultHasher> = opaque_hasher.as_proj::<DefaultHasher>();
    /// ```
    #[inline]
    pub fn as_proj<H>(&self) -> &TypedProjHasher<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<H>();

        unsafe { &*(self as *const OpaqueHasher as *const TypedProjHasher<H>) }
    }

    /// Projects the mutable type-erased hasher reference into a mutable type-projected
    /// hasher reference.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hasher of `self` do not match the requested
    /// hasher type `H`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{OpaqueHasher, TypedProjHasher};
    /// # use std::hash::DefaultHasher;
    /// #
    /// let mut opaque_hasher = OpaqueHasher::new::<DefaultHasher>(DefaultHasher::new());
    /// #
    /// # assert!(opaque_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let proj_hasher: &mut TypedProjHasher<DefaultHasher> = opaque_hasher.as_proj_mut::<DefaultHasher>();
    /// ```
    #[inline]
    pub fn as_proj_mut<H>(&mut self) -> &mut TypedProjHasher<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<H>();

        unsafe { &mut *(self as *mut OpaqueHasher as *mut TypedProjHasher<H>) }
    }

    /// Projects the type-erased hasher value into a type-projected hasher value.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hasher of `self` do not match the requested
    /// hasher type `H`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{OpaqueHasher, TypedProjHasher};
    /// # use std::hash::DefaultHasher;
    /// #
    /// let opaque_hasher = OpaqueHasher::new::<DefaultHasher>(DefaultHasher::new());
    /// #
    /// # assert!(opaque_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let proj_hasher: TypedProjHasher<DefaultHasher> = opaque_hasher.into_proj::<DefaultHasher>();
    /// ```
    #[inline]
    pub fn into_proj<H>(self) -> TypedProjHasher<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<H>();

        TypedProjHasher {
            inner: self.inner.into_proj(),
        }
    }

    /// Erases the type-projected hasher value into a type-erased hasher value.
    ///
    /// Unlike the type projection methods [`as_proj`], [`as_proj_mut`], and [`into_proj`], this
    /// method never panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::{OpaqueHasher, TypedProjHasher};
    /// # use std::hash::DefaultHasher;
    /// #
    /// let proj_hasher: TypedProjHasher<DefaultHasher> = TypedProjHasher::new(DefaultHasher::new());
    /// let opaque_hasher: OpaqueHasher = OpaqueHasher::from_proj(proj_hasher);
    /// #
    /// # assert!(opaque_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// ```
    ///
    /// [`as_proj`]: OpaqueHasher::as_proj,
    /// [`as_proj_mut`]: OpaqueHasher::as_proj_mut
    /// [`into_proj`]: OpaqueHasher::into_proj
    #[inline]
    pub fn from_proj<H>(proj_self: TypedProjHasher<H>) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            inner: OpaqueHasherInner::from_proj(proj_self.inner),
        }
    }
}


impl OpaqueHasher {
    /// Constructs a new type-erased hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::OpaqueHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let opaque_hasher = OpaqueHasher::new::<DefaultHasher>(DefaultHasher::new());
    ///
    /// assert_eq!(opaque_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// assert_ne!(opaque_hasher.hasher_type_id(), TypeId::of::<Box<DefaultHasher>>());
    /// ```
    #[inline]
    pub fn new<H>(hasher: H) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_hasher = TypedProjHasher::<H>::new(hasher);

        Self::from_proj(proj_hasher)
    }

    /// Constructs a new type-erased hasher from a boxed hasher.
    ///
    /// The underlying type of the type-erased hasher will be the type of the hasher held by
    /// the box.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::OpaqueHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let opaque_hasher = OpaqueHasher::from_boxed_hasher(Box::new(DefaultHasher::new()));
    ///
    /// assert_eq!(opaque_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// assert_ne!(opaque_hasher.hasher_type_id(), TypeId::of::<Box<DefaultHasher>>());
    ///
    /// // In contrast, a type-projected hasher constructed using `new` will have the boxed hasher
    /// // as the underlying hasher type.
    /// let opaque_boxed_hasher = OpaqueHasher::new(Box::new(DefaultHasher::new()));
    ///
    /// assert_eq!(opaque_boxed_hasher.hasher_type_id(), TypeId::of::<Box<DefaultHasher>>());
    /// assert_ne!(opaque_boxed_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// ```
    #[inline]
    pub fn from_boxed_hasher<H>(hasher: Box<H>) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_hasher = TypedProjHasher::<H>::from_boxed_hasher(hasher);

        Self::from_proj(proj_hasher)
    }

    /// Returns a reference to the underlying hasher.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hasher of `self` do not match the requested
    /// hasher type `H`.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::OpaqueHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let opaque_hasher = OpaqueHasher::new(DefaultHasher::new());
    /// #
    /// # assert!(opaque_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    ///
    /// let hasher: &DefaultHasher = opaque_hasher.hasher::<DefaultHasher>();
    /// ```
    pub fn hasher<H>(&self) -> &H
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<H>();

        proj_self.hasher()
    }

    /// Converts the type-erased hasher into a boxed hasher.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the hasher of `self` do not match the requested
    /// hasher type `H`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opaque_hash::OpaqueHasher;
    /// # use std::any::TypeId;
    /// # use std::hash::DefaultHasher;
    /// #
    /// let opaque_hasher = OpaqueHasher::new(DefaultHasher::new());
    /// #
    /// # assert!(opaque_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    /// let boxed_hasher: Box<DefaultHasher> = opaque_hasher.into_boxed_hasher::<DefaultHasher>();
    ///
    /// let new_opaque_hasher = OpaqueHasher::from_boxed_hasher(boxed_hasher);
    /// #
    /// # assert!(new_opaque_hasher.has_hasher_type::<DefaultHasher>());
    /// #
    ///
    /// assert_eq!(new_opaque_hasher.hasher_type_id(), TypeId::of::<DefaultHasher>());
    /// assert_ne!(new_opaque_hasher.hasher_type_id(), TypeId::of::<Box<DefaultHasher>>());
    /// ```
    pub fn into_boxed_hasher<H>(self) -> Box<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.into_proj::<H>();

        proj_self.into_boxed_hasher()
    }
}

impl hash::Hasher for OpaqueHasher {
    fn finish(&self) -> u64 {
        self.inner.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.inner.write(bytes)
    }
}

impl fmt::Debug for OpaqueHasher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("OpaqueHasher").finish()
    }
}

mod dummy {
    use super::*;

    #[allow(dead_code)]
    pub(super) struct DummyHasher {
        _do_not_construct: marker::PhantomData<()>,
    }

    impl hash::Hasher for DummyHasher {
        #[inline]
        fn finish(&self) -> u64 {
            panic!("[`DummyHasher::finish`] should never actually be called. Its purpose is to test struct layouts.");
        }

        #[inline]
        fn write(&mut self, _bytes: &[u8]) {
            panic!("[`DummyHasher::write`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }

    #[allow(dead_code)]
    pub(super) struct DummyBuildHasher {
        _do_not_construct: marker::PhantomData<()>,
    }

    impl hash::BuildHasher for DummyBuildHasher {
        type Hasher = DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            panic!("[`DummyBuildHasher::build_hasher`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }
}

#[cfg(test)]
mod hasher_layout_tests {
    use super::*;
    use std::hash;

    fn run_test_opaque_hasher_match_sizes<H>()
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = core::mem::size_of::<TypedProjHasher<H>>();
        let result = core::mem::size_of::<OpaqueHasher>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_hasher_match_alignments<H>()
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = core::mem::align_of::<TypedProjHasher<H>>();
        let result = core::mem::align_of::<OpaqueHasher>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_hasher_match_offsets<H>()
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        assert_eq!(
            core::mem::offset_of!(TypedProjHasher<H>, inner),
            core::mem::offset_of!(OpaqueHasher, inner),
            "Opaque and Typed Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $hasher_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_hasher_layout_match_sizes() {
                    run_test_opaque_hasher_match_sizes::<$hasher_typ>();
                }

                #[test]
                fn test_hasher_layout_match_alignments() {
                    run_test_opaque_hasher_match_alignments::<$hasher_typ>();
                }

                #[test]
                fn test_hasher_layout_match_offsets() {
                    run_test_opaque_hasher_match_offsets::<$hasher_typ>();
                }
            }
        };
    }

    #[cfg(feature = "std")]
    layout_tests!(default_hasher, hash::DefaultHasher);

    layout_tests!(dummy_hasher, dummy::DummyHasher);
}

#[cfg(test)]
mod assert_send_sync {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjHasher<hash::DefaultHasher>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjHasher<dummy::DummyHasher>>();
    }
}
