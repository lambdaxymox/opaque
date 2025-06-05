use crate::into_iter::IntoIter;
use crate::drain::Drain;
use crate::extract_if::ExtractIf;
use crate::splice::Splice;
use crate::vec_inner::{OpaqueVecInner, TypedProjVecInner};

use core::any;
use core::cmp;
use core::hash;
use core::mem;
use core::ops;
use core::slice;
use core::fmt;
use core::ptr::NonNull;
use core::mem::{
    ManuallyDrop,
    MaybeUninit,
};
use alloc_crate::alloc;
use alloc_crate::borrow;
use alloc_crate::boxed::Box;
use alloc_crate::vec::Vec;

use opaque_alloc::TypedProjAlloc;
use opaque_error::TryReserveError;

/// A type-projected contiguous growable array type. This is similar to the [`Vec`] data type
/// from [`std`], except that the generic parameters can be type-erased.
#[repr(transparent)]
pub struct TypedProjVec<T, A = alloc::Global>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: TypedProjVecInner<T, A>,
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = TypedProjVecInner::new_proj_in(proj_alloc);

        Self { inner, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = TypedProjVecInner::with_capacity_proj_in(capacity, proj_alloc);

        Self { inner, }
    }

    #[inline]
    pub fn try_with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Result<Self, TryReserveError> {
        let inner = TypedProjVecInner::try_with_capacity_proj_in(capacity, proj_alloc)?;

        Ok(Self { inner, })
    }

    #[inline]
    pub unsafe fn from_raw_parts_proj_in(ptr: *mut T, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_raw_parts_proj_in(ptr, length, capacity, proj_alloc)
        };

        Self { inner, }
    }

    #[inline]
    pub unsafe fn from_parts_proj_in(ptr: NonNull<T>, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_parts_proj_in(ptr, length, capacity, proj_alloc)
        };

        Self { inner, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in(alloc: A) -> Self {
        let inner = TypedProjVecInner::new_in(alloc);

        Self { inner, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let inner = TypedProjVecInner::with_capacity_in(capacity, alloc);

        Self { inner, }
    }

    #[inline]
    pub fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, TryReserveError> {
        let inner = TypedProjVecInner::try_with_capacity_in(capacity, alloc)?;

        Ok(Self { inner, })
    }

    #[inline]
    pub unsafe fn from_raw_parts_in(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_raw_parts_in(ptr, length, capacity, alloc)
        };

        Self { inner, }
    }

    #[inline]
    pub unsafe fn from_parts_in(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_parts_in(ptr, length, capacity, alloc)
        };

        Self { inner, }
    }
}

impl<T> TypedProjVec<T, alloc::Global>
where
    T: any::Any,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new() -> Self {
        let inner = TypedProjVecInner::new();

        Self { inner, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity(capacity: usize) -> Self {
        let inner = TypedProjVecInner::with_capacity(capacity);

        Self { inner, }
    }

    #[inline]
    pub fn try_with_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        let inner = TypedProjVecInner::try_with_capacity(capacity)?;

        Ok(Self { inner, })
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_raw_parts(ptr, length, capacity)
        };

        Self { inner, }
    }

    #[inline]
    pub unsafe fn from_parts(ptr: NonNull<T>, length: usize, capacity: usize) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_parts(ptr, length, capacity)
        };

        Self { inner, }
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        self.inner.allocator()
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        unsafe {
            self.inner.set_len(new_len)
        }
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &<I as slice::SliceIndex<[T]>>::Output
    where
        I: slice::SliceIndex<[T]>,
    {
        unsafe {
            self.inner.get_unchecked(index)
        }
    }

    #[inline]
    #[must_use]
    pub unsafe fn get_mut_unchecked<I>(&mut self, index: I) -> &mut <I as slice::SliceIndex<[T]>>::Output
    where
        I: slice::SliceIndex<[T]>,
    {
        unsafe {
            self.inner.get_mut_unchecked(index)
        }
    }

    #[inline]
    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&<I as slice::SliceIndex<[T]>>::Output>
    where
        I: slice::SliceIndex<[T]>,
    {
        self.inner.get(index)
    }

    #[inline]
    #[must_use]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as slice::SliceIndex<[T]>>::Output>
    where
        I: slice::SliceIndex<[T]>,
    {
        self.inner.get_mut(index)
    }

    #[inline]
    #[track_caller]
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    pub fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
        self.inner.push_within_capacity(value)
    }

    #[track_caller]
    pub fn replace_insert(&mut self, index: usize, value: T) {
        self.inner.replace_insert(index, value);
    }

    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, value: T) {
        self.inner.shift_insert(index, value);
    }

    #[track_caller]
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.inner.swap_remove(index)
    }

    #[track_caller]
    pub fn shift_remove(&mut self, index: usize) -> T {
        self.inner.shift_remove(index)
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.inner.contains(value)
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.inner.iter_mut()
    }

    #[inline]
    #[track_caller]
    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner)
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
    where
        R: ops::RangeBounds<usize>,
    {
        self.inner.drain(range)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr()
    }

    #[inline]
    pub fn as_non_null(&mut self) -> NonNull<T> {
        self.inner.as_non_null()
    }

    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }

    #[must_use]
    pub fn into_raw_parts(self) -> (*mut T, usize, usize) {
        self.inner.into_raw_parts()
    }

    #[must_use]
    pub fn into_parts(self) -> (NonNull<T>, usize, usize) {
        self.inner.into_parts()
    }

    #[must_use]
    pub fn into_raw_parts_with_alloc(self) -> (*mut T, usize, usize, TypedProjAlloc<A>) {
        self.inner.into_raw_parts_with_alloc()
    }

    #[must_use]
    pub fn into_parts_with_alloc(self) -> (NonNull<T>, usize, usize, TypedProjAlloc<A>) {
        self.inner.into_parts_with_alloc()
    }

    #[track_caller]
    pub fn into_boxed_slice(self) -> Box<[T], TypedProjAlloc<A>> {
        self.inner.into_boxed_slice()
    }

    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    #[track_caller]
    pub fn split_off(&mut self, at: usize) -> Self
    where
        A: Clone,
    {
        let inner = self.inner.split_off(at);

        Self { inner, }
    }

    #[track_caller]
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T,
    {
        self.inner.resize_with(new_len, f)
    }

    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        self.inner.spare_capacity_mut()
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    #[track_caller]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    #[track_caller]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    #[track_caller]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    where
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        self.inner.splice::<R, I>(range, replace_with)
    }

    pub fn extract_if<F, R>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    where
        T: any::Any,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        self.inner.extract_if::<F, R>(range, filter)
    }

    /*
    #[track_caller]
    fn extend_with(&mut self, count: usize, value: T)
    where
        T: Clone,
    {
        self.inner.extend_with(count, value);
    }

    #[track_caller]
    fn extend_from_iter<I>(&mut self, iterator: I)
    where
        T: Clone,
        I: Iterator<Item = T>,
    {
        self.inner.extend_from_iter::<I>(iterator)
    }
    */

    #[track_caller]
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.inner.extend_from_slice(other);
    }

    #[track_caller]
    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        self.inner.resize(new_len, value);
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.inner.retain(|elem| f(elem));
    }

    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.inner.retain_mut(f)
    }

    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.inner.dedup_by_key(key)
    }

    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.inner.dedup_by(same_bucket)
    }

    #[inline]
    pub fn dedup(&mut self)
    where
        T: PartialEq,
    {
        self.inner.dedup()
    }
}

impl<T, A> ops::Deref for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, A> ops::DerefMut for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

/*
unsafe impl<T, A> ops::DerefPure for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}
*/

impl<T, A> Clone for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        let cloned_inner = self.inner.clone();

        Self {
            inner: cloned_inner,
        }
    }
}

impl<T, A> hash::Hash for TypedProjVec<T, A>
where
    T: any::Any + hash::Hash,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash(self.as_slice(), state)
    }
}

impl<T, I, A> ops::Index<I> for TypedProjVec<T, A>
where
    T: any::Any,
    I: slice::SliceIndex<[T]>,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        ops::Index::index(self.as_slice(), index)
    }
}

impl<T, I, A> ops::IndexMut<I> for TypedProjVec<T, A>
where
    T: any::Any,
    I: slice::SliceIndex<[T]>,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        ops::IndexMut::index_mut(self.as_mut_slice(), index)
    }
}

impl<T> FromIterator<T> for TypedProjVec<T, alloc::Global>
where
    T: any::Any,
{
    #[inline]
    #[track_caller]
    fn from_iter<I>(iter: I) -> TypedProjVec<T, alloc::Global>
    where
        I: IntoIterator<Item = T>,
    {
        let inner = TypedProjVecInner::from_iter(iter);

        Self { inner, }
    }
}

impl<T, A> IntoIterator for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = T;
    type IntoIter = IntoIter<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let mut me = ManuallyDrop::new(self);
            let alloc = ManuallyDrop::new(core::ptr::read(me.allocator()));
            let inner = me.as_non_null();
            let begin = inner.as_ptr();
            let end = if crate::zst::is_zst::<T>() {
                begin.wrapping_byte_add(me.len())
            } else {
                begin.add(me.len()) as *const T
            };
            let cap = me.inner.capacity();

            IntoIter::from_parts(inner, cap, alloc, inner, end)
        }
    }
}

impl<'a, T, A> IntoIterator for &'a TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, A> IntoIterator for &'a mut TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, A> Extend<T> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    #[track_caller]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.inner.extend(iter)
    }

    /*
    #[inline]
    #[track_caller]
    fn extend_one(&mut self, item: T) {
        self.inner.push(item);
    }
    */
    /*
    #[inline]
    #[track_caller]
    fn extend_reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }
    */
    /*
    #[inline]
    unsafe fn extend_one_unchecked(&mut self, item: T) {
        // SAFETY: Our preconditions ensure the space has been reserved, and `extend_reserve` is implemented correctly.
        unsafe {
            let len = self.len();
            core::ptr::write(self.as_mut_ptr().add(len), item);
            self.set_len(len + 1);
        }
    }
    */
}

impl<'a, T, A> Extend<&'a T> for TypedProjVec<T, A>
where
    T: any::Any + Copy + 'a,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[track_caller]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.inner.extend(iter.into_iter().copied())
    }

    /*
    #[inline]
    #[track_caller]
    fn extend_one(&mut self, &item: &'a T) {
        self.push(item);
    }
    */
    /*
    #[inline]
    #[track_caller]
    fn extend_reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }
    */
    /*
    #[inline]
    unsafe fn extend_one_unchecked(&mut self, &item: &'a T) {
        // SAFETY: Our preconditions ensure the space has been reserved, and `extend_reserve` is implemented correctly.
        unsafe {
            let len = self.len();
            core::ptr::write(self.as_mut_ptr().add(len), item);
            self.set_len(len + 1);
        }
    }
    */
}

impl<T, A1, A2> PartialEq<TypedProjVec<T, A2>> for TypedProjVec<T, A1>
where
    T: any::Any + PartialEq,
    A1: any::Any + alloc::Allocator + Send + Sync,
    A2: any::Any + alloc::Allocator + Send + Sync,
{
    fn eq(&self, other: &TypedProjVec<T, A2>) -> bool {
        PartialEq::eq(self.as_slice(), other.as_slice())
    }
}

impl<T, A1, A2> PartialOrd<TypedProjVec<T, A2>> for TypedProjVec<T, A1>
where
    T: any::Any + PartialOrd,
    A1: any::Any + alloc::Allocator + Send + Sync,
    A2: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn partial_cmp(&self, other: &TypedProjVec<T, A2>) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(self.as_slice(), other.as_slice())
    }
}

impl<T, A> Eq for TypedProjVec<T, A>
where
    T: any::Any + Eq,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, A> Ord for TypedProjVec<T, A>
where
    T: any::Any + Ord,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Ord::cmp(self.as_slice(), other.as_slice())
    }
}

impl<T, A> Default for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> TypedProjVec<T, A> {
        TypedProjVec::new_in(Default::default())
    }
}

impl<T, A> fmt::Debug for TypedProjVec<T, A>
where
    T: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl<T, A> AsRef<TypedProjVec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn as_ref(&self) -> &TypedProjVec<T, A> {
        self
    }
}

impl<T, A> AsMut<TypedProjVec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn as_mut(&mut self) -> &mut TypedProjVec<T, A> {
        self
    }
}

impl<T, A> AsRef<[T]> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, A> AsMut<[T]> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> From<&[T]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &[T]) -> TypedProjVec<T, alloc::Global> {
        let inner = TypedProjVecInner::from(slice);

        Self { inner, }
    }
}

impl<T> From<&mut [T]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &mut [T]) -> TypedProjVec<T, alloc::Global> {
        let inner = TypedProjVecInner::from(slice);

        Self { inner, }
    }
}

impl<T, const N: usize> From<&[T; N]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &[T; N]) -> TypedProjVec<T, alloc::Global> {
        Self::from(slice.as_slice())
    }
}

impl<T, const N: usize> From<&mut [T; N]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &mut [T; N]) -> TypedProjVec<T, alloc::Global> {
        Self::from(slice.as_mut_slice())
    }
}

impl<T, const N: usize> From<[T; N]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any,
{
    #[track_caller]
    fn from(slice: [T; N]) -> TypedProjVec<T, alloc::Global> {
        let inner = TypedProjVecInner::from(slice);

        Self { inner, }
    }
}

impl<'a, T> From<borrow::Cow<'a, [T]>> for TypedProjVec<T, alloc::Global>
where
    T: any::Any,
    [T]: borrow::ToOwned<Owned = TypedProjVec<T, alloc::Global>>,
{
    #[track_caller]
    fn from(slice: borrow::Cow<'a, [T]>) -> TypedProjVec<T, alloc::Global> {
        slice.into_owned()
    }
}

impl<T, A> From<Box<[T], A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn from(slice: Box<[T], A>) -> Self {
        let inner = TypedProjVecInner::from(slice);

        Self { inner, }
    }
}

impl<T, A> From<Vec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[track_caller]
    fn from(vec: Vec<T, A>) -> Self {
        let inner = TypedProjVecInner::from(vec);

        Self { inner, }
    }
}

impl<T, A> From<&Vec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    #[track_caller]
    fn from(vec: &Vec<T, A>) -> Self {
        let inner = TypedProjVecInner::from(vec);

        Self { inner, }
    }
}

impl<T, A> From<&mut Vec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    #[track_caller]
    fn from(vec: &mut Vec<T, A>) -> Self {
        let inner = TypedProjVecInner::from(vec);

        Self { inner, }
    }
}

impl<T, A> From<TypedProjVec<T, A>> for Box<[T], TypedProjAlloc<A>>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[track_caller]
    fn from(vec: TypedProjVec<T, A>) -> Self {
        vec.into_boxed_slice()
    }
}

impl From<&str> for TypedProjVec<u8, alloc::Global> {
    #[track_caller]
    fn from(st: &str) -> TypedProjVec<u8, alloc::Global> {
        From::from(st.as_bytes())
    }
}

impl<T, A, const N: usize> TryFrom<TypedProjVec<T, A>> for [T; N]
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Error = TypedProjVec<T, A>;

    fn try_from(mut vec: TypedProjVec<T, A>) -> Result<[T; N], TypedProjVec<T, A>> {
        if vec.len() != N {
            return Err(vec);
        }

        // SAFETY: `.set_len(0)` is always sound.
        unsafe { vec.set_len(0) };

        // SAFETY: A `Vec`'s pointer is always aligned properly, and
        // the alignment the array needs is the same as the items.
        // We checked earlier that we have sufficient items.
        // The items will not double-drop as the `set_len`
        // tells the `Vec` not to also drop them.
        let array = unsafe { core::ptr::read(vec.as_ptr() as *const [T; N]) };
        Ok(array)
    }
}

/// A type-erased contiguous growable array type. This is similar to the [`Vec`] data type
/// from [`std`] except that the concrete type of the generic parameters are type-erased.
#[repr(transparent)]
pub struct OpaqueVec {
    inner: OpaqueVecInner,
}

impl OpaqueVec {
    /// Returns the [`TypeId`] of the elements of contained in an [`OpaqueVec`].
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// # use std::any::TypeId;
    /// #
    /// let opaque_vec = OpaqueVec::new_in::<i32, Global>(Global);
    /// let expected = TypeId::of::<i32>();
    /// let result = opaque_vec.element_type_id();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub const fn element_type_id(&self) -> any::TypeId {
        self.inner.element_type_id()
    }

    /// Returns the [`TypeId`] of the memory allocator of an [`OpaqueVec`].
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// # use std::any::TypeId;
    /// #
    /// let opaque_vec = OpaqueVec::new_in::<i32, Global>(Global);
    /// let expected = TypeId::of::<Global>();
    /// let result = opaque_vec.allocator_type_id();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }

    /// Determine whether an [`OpaqueVec`] has a specific element type.
    ///
    /// Returns `true` if `self` has the specified element type. Returns `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_vec = OpaqueVec::new_in::<i32, Global>(Global);
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// ```
    #[inline]
    pub fn has_element_type<T>(&self) -> bool
    where
        T: any::Any,
    {
        self.inner.element_type_id() == any::TypeId::of::<T>()
    }

    /// Determine whether an [`OpaqueVec`] has a specific memory allocator type.
    ///
    /// Returns `true` if `self` has the specified memory allocator type. Returns `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_vec = OpaqueVec::new_in::<i32, Global>(Global);
    ///
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// ```
    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.inner.allocator_type_id() == any::TypeId::of::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<T, A>(&self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(st: &str, type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("{:?} type mismatch. Need `{:?}`, got `{:?}`", st, type_id_self, type_id_other);
        }

        if !self.has_element_type::<T>() {
            type_check_failed("Element", self.inner.element_type_id(), any::TypeId::of::<T>());
        }

        if !self.has_allocator_type::<A>() {
            type_check_failed("Allocator", self.inner.allocator_type_id(), any::TypeId::of::<A>());
        }
    }
}

impl OpaqueVec {
    /// Projects the type-erased [`OpaqueVec`] reference into a type-projected
    /// [`TypedProjVec`] reference.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::{OpaqueVec, TypedProjVec};
    /// # use std::alloc::Global;
    /// #
    /// let opaque_vec = OpaqueVec::new_in::<i32, Global>(Global);
    /// let proj_vec: &TypedProjVec<i32, Global> = opaque_vec.as_proj::<i32, Global>();
    /// ```
    #[inline]
    pub fn as_proj<T, A>(&self) -> &TypedProjVec<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<T, A>();

        unsafe { &*(self as *const OpaqueVec as *const TypedProjVec<T, A>) }
    }

    /// Projects the type-erased [`OpaqueVec`] mutable reference into a type-projected
    /// [`TypedProjVec`] mutable reference.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::{OpaqueVec, TypedProjVec};
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = OpaqueVec::new_in::<i32, Global>(Global);
    /// let proj_vec: &mut TypedProjVec<i32, Global> = opaque_vec.as_proj_mut::<i32, Global>();
    /// ```
    #[inline]
    pub fn as_proj_mut<T, A>(&mut self) -> &mut TypedProjVec<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<T, A>();

        unsafe { &mut *(self as *mut OpaqueVec as *mut TypedProjVec<T, A>) }
    }

    /// Projects a type-erased [`OpaqueVec`] value into a type-projected
    /// [`TypedProjVec`] value.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::{OpaqueVec, TypedProjVec};
    /// # use std::alloc::Global;
    /// #
    /// let opaque_vec = OpaqueVec::new_in::<i32, Global>(Global);
    /// let proj_vec: TypedProjVec<i32, Global> = opaque_vec.into_proj::<i32, Global>();
    /// ```
    #[inline]
    pub fn into_proj<T, A>(self) -> TypedProjVec<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<T, A>();

        TypedProjVec {
            inner: self.inner.into_proj_assuming_type::<T, A>(),
        }
    }

    /// Erases the type-projected [`TypedProjVec`] value into a type-erased
    /// [`OpaqueVec`] value.
    ///
    /// Unlike the type projection methods [`as_proj`], [`as_proj_mut`], and [`into_proj`], this
    /// method never panics.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::{OpaqueVec, TypedProjVec};
    /// # use std::alloc::Global;
    /// #
    /// let proj_vec: TypedProjVec<i32, Global> = TypedProjVec::new_in(Global);
    /// let opaque_vec: OpaqueVec = OpaqueVec::from_proj(proj_vec);
    /// ```
    ///
    /// [`as_proj`]: OpaqueVec::as_proj,
    /// [`as_proj_mut`]: OpaqueVec::as_proj_mut
    /// [`into_proj`]: OpaqueVec::into_proj
    #[inline]
    pub fn from_proj<T, A>(proj_self: TypedProjVec<T, A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        Self {
            inner: OpaqueVecInner::from_proj(proj_self.inner),
        }
    }
}

impl OpaqueVec {
    /// Constructs a new empty [`OpaqueVec`] using a specific type-projected memory allocator.
    ///
    /// The vector will not allocate until elements are pushed into it. In particular, the
    /// vector has zero capacity until elements are pushed into it.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let opaque_vec = OpaqueVec::new_proj_in::<i32, Global>(proj_alloc);
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(!opaque_vec.has_allocator_type::<TypedProjAlloc<Global>>());
    /// assert!(opaque_vec.is_empty());
    ///
    /// assert_eq!(opaque_vec.capacity(), 0);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_proj_in<T, A>(proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = TypedProjVec::<T, A>::new_proj_in(proj_alloc);

        Self::from_proj(proj_vec)
    }

    /// Constructs a new empty [`OpaqueVec`] using a specific type-projected memory allocator
    /// and a specific capacity.
    ///
    /// The vector will be able to hold at least `capacity` elements without reallocating. The
    /// method _can_ allocate more than `capacity` elements. If `capacity` is zero, the
    /// constructor does not allocate memory, i.e. it is equivalent to [`new_proj_in`] when
    /// `capacity` is zero.
    ///
    /// Note that while the returned vector will have a **capacity** of at least `capacity`, it will
    /// have a **length** of zero, because no elements have been pushed to the vector yet.
    ///
    /// # Panics
    ///
    /// This method panics if the new capacity exceeds `isize::MAX` _bytes_.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 32;
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let opaque_vec = OpaqueVec::with_capacity_proj_in::<i32, Global>(capacity, proj_alloc.clone());
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(!opaque_vec.has_allocator_type::<TypedProjAlloc<Global>>());
    /// assert!(opaque_vec.capacity() >= capacity);
    /// assert!(opaque_vec.is_empty());
    ///
    /// let empty_vec = OpaqueVec::with_capacity_proj_in::<i32, Global>(0, proj_alloc.clone());
    ///
    /// assert_eq!(empty_vec.capacity(), 0);
    /// assert!(empty_vec.is_empty());
    /// ```
    ///
    /// [`new_proj_in`]: OpaqueVec::new_proj_in
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_proj_in<T, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = TypedProjVec::<T, A>::with_capacity_proj_in(capacity, proj_alloc);

        Self::from_proj(proj_vec)
    }

    /// Constructs a new empty [`OpaqueVec`] using a specific type-projected memory allocator
    /// and a specific capacity.
    ///
    /// The vector will be able to hold at least `capacity` elements without reallocating. The
    /// method _can_ allocate more than `capacity` elements. If `capacity` is zero, the
    /// constructor does not allocate memory, i.e. it is equivalent to [`new_proj_in`] when
    /// `capacity` is zero.
    ///
    /// Note that while the returned vector will have a **capacity** of at least `capacity`, it will
    /// have a **length** of zero, because no elements have been pushed to the vector yet.
    ///
    /// # Errors
    ///
    /// This method returns an error if the capacity `capacity` exceeds `isize::MAX` bytes, or if the
    /// allocator reports an allocation failure.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 32;
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let opaque_vec = OpaqueVec::try_with_capacity_proj_in::<i32, Global>(capacity, proj_alloc.clone());
    ///
    /// assert!(opaque_vec.is_ok());
    ///
    /// let opaque_vec = opaque_vec.unwrap();
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(!opaque_vec.has_allocator_type::<TypedProjAlloc<Global>>());
    /// assert!(opaque_vec.capacity() >= capacity);
    /// assert!(opaque_vec.is_empty());
    ///
    /// let empty_vec = OpaqueVec::with_capacity_proj_in::<i32, Global>(0, proj_alloc.clone());
    ///
    /// assert_eq!(empty_vec.capacity(), 0);
    /// assert!(empty_vec.is_empty());
    /// ```
    ///
    /// [`new_proj_in`]: OpaqueVec::new_proj_in
    #[inline]
    pub fn try_with_capacity_proj_in<T, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Result<Self, TryReserveError>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = TypedProjVec::<T, A>::try_with_capacity_proj_in(capacity, proj_alloc)?;

        Ok(Self::from_proj(proj_vec))
    }

    /// Constructs an [`OpaqueVec`] directly from a pointer, a length, a capacity, and a
    /// type-projected allocator.
    ///
    /// # Safety
    ///
    /// This method is highly unsafe. A safe use of it must satisfy the following invariants:
    ///
    /// * The pointer `ptr` must be non-null.
    /// * The allocation referred to by `ptr` must have been allocated using the supplied
    ///   allocator.
    /// * The element type `T` must have the same alignment that `ptr` was allocated with.
    ///   The type `T` cannot have a less strict alignment is not sufficient; the alignment really
    ///   must be equal to satisfy the [`dealloc`] requirement that memory must be allocated and
    ///   deallocated with the same layout.
    /// * The allocation size in bytes (`mem::size_of::<T>() * capacity`) must
    ///   be the same size as the pointer was allocated with. Similar to alignment, [`dealloc`] must
    ///   be called with the same layout `size`.
    /// * The length `length` of the elements inside the allocation must be less than or equal to
    ///   the capacity `capacity`.
    /// * The first `length` values must be properly initialized values of type `T`.
    /// * `capacity` must be the capacity that the pointer was allocated with.
    /// * The allocated size in bytes must be no larger than `isize::MAX`.
    ///   See the safety documentation of [`pointer::offset`].
    ///
    /// These requirements always hold for any `ptr` that has been allocated via [`OpaqueVec`].
    ///
    /// The ownership of `ptr` is effectively transferred to the
    /// [`OpaqueVec`] which may then deallocate, reallocate or change the
    /// contents of memory pointed to by the pointer at will. The caller must ensure
    /// that nothing else uses the pointer `ptr` after calling this method.
    ///
    /// # Examples
    ///
    /// Using memory that was allocated by an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// # use std::ptr;
    /// # use std::mem;
    /// #
    /// let opaque_vec = {
    ///     let array: [i32; 3] = [1, 2, 3];
    ///     OpaqueVec::from(array)
    /// };
    ///
    /// // Prevent running `opaque_vec`'s destructor to completely control the allocation.
    /// let mut opaque_vec = mem::ManuallyDrop::new(opaque_vec);
    ///
    /// // Destructure `opaque_vec` into its constituent parts.
    /// let ptr: *mut i32 = opaque_vec.as_mut_ptr::<i32, Global>();
    /// let length = opaque_vec.len();
    /// let capacity = opaque_vec.capacity();
    /// let proj_alloc: TypedProjAlloc<Global> = unsafe { ptr::read(opaque_vec.allocator::<i32, Global>()) };
    ///
    /// let expected = {
    ///     let array: [i32; 3] = [4, 5, 6];
    ///     OpaqueVec::from(array)
    /// };
    /// let result = unsafe {
    ///     // Mutate the values directly in memory.
    ///     for i in 0..length {
    ///         ptr::write(ptr.add(i), 4 + i as i32);
    ///     }
    ///
    ///     // Rebuild the vector.
    ///     OpaqueVec::from_raw_parts_proj_in::<i32, Global>(ptr, length, capacity, proj_alloc)
    /// };
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert_eq!(result.capacity(), expected.capacity());
    /// # assert!(!result.is_empty());
    /// # assert_eq!(result.len(), length);
    /// # assert_eq!(result.capacity(), capacity);
    /// # assert!(result.len() <= result.capacity());
    /// # assert!(!expected.is_empty());
    /// # assert_eq!(expected.len(), length);
    /// # assert_eq!(expected.capacity(), capacity);
    /// # assert!(expected.len() <= expected.capacity());
    ///
    /// let mut result = result;
    /// let new_capacity = 16;
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// for _ in 0..(new_capacity - length) {
    ///     result.push::<i32, Global>(i32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     4,        5,        6,        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ///     i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert!(result.len() <= result.capacity());
    /// # assert_eq!(result.len(), new_capacity);
    /// # assert!(result.capacity() >= new_capacity);
    /// ```
    ///
    /// Using memory that was allocated outside an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Allocator;
    /// # use std::alloc::Global;
    /// # use std::alloc::Layout;
    /// # use std::ptr::NonNull;
    /// #
    /// let value = 1_000_000;
    /// let layout = Layout::array::<u32>(16).unwrap();
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let length = 1;
    /// let capacity = 16;
    /// let opaque_vec = unsafe {
    ///     let mut memory: NonNull<u32> = proj_alloc.allocate(layout).unwrap().cast::<u32>();
    ///     memory.write(value);
    ///
    ///     OpaqueVec::from_raw_parts_proj_in::<u32, Global>(memory.as_mut() as *mut u32, length, capacity, proj_alloc)
    /// };
    ///
    /// assert_eq!(opaque_vec.as_slice::<u32, Global>(), &[value]);
    /// assert_eq!(opaque_vec.len(), length);
    /// assert_eq!(opaque_vec.capacity(), capacity);
    /// # assert!(!opaque_vec.is_empty());
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// let mut result = opaque_vec;
    /// for _ in 0..(capacity - length) {
    ///     result.push::<u32, Global>(u32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     value,     u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    ///     u32::MAX,  u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<u32, Global>(), expected.as_slice::<u32, Global>());
    /// assert_eq!(result.len(), result.capacity());
    /// # assert_eq!(result.len(), capacity);
    /// # assert_eq!(result.capacity(), capacity);
    /// ```
    ///
    /// [`dealloc`]: std::alloc::Allocator::dealloc
    #[inline]
    pub unsafe fn from_raw_parts_proj_in<T, A>(ptr: *mut T, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, A>::from_raw_parts_proj_in(ptr, length, capacity, proj_alloc)
        };

        Self::from_proj(proj_vec)
    }

    /// Constructs an [`OpaqueVec`] directly from a non-null pointer, a length, a capacity, and a
    /// type-projected allocator.
    ///
    /// # Safety
    ///
    /// This method is highly unsafe. A safe use of it must satisfy the following invariants:
    ///
    /// * The pointer `ptr` must be non-null.
    /// * The allocation referred to by `ptr` must have been allocated using the supplied
    ///   allocator.
    /// * The element type `T` must have the same alignment that `ptr` was allocated with.
    ///   The type `T` cannot have a less strict alignment is not sufficient; the alignment really
    ///   must be equal to satisfy the [`dealloc`] requirement that memory must be allocated and
    ///   deallocated with the same layout.
    /// * The allocation size in bytes (`mem::size_of::<T>() * capacity`) must
    ///   be the same size as the pointer was allocated with. Similar to alignment, [`dealloc`] must
    ///   be called with the same layout `size`.
    /// * The length `length` of the elements inside the allocation must be less than or equal to
    ///   the capacity `capacity`.
    /// * The first `length` values must be properly initialized values of type `T`.
    /// * `capacity` must be the capacity that the pointer was allocated with.
    /// * The allocated size in bytes must be no larger than `isize::MAX`.
    ///   See the safety documentation of [`pointer::offset`].
    ///
    /// These requirements always hold for any `ptr` that has been allocated via [`OpaqueVec`].
    ///
    /// The ownership of `ptr` is effectively transferred to the
    /// [`OpaqueVec`] which may then deallocate, reallocate or change the
    /// contents of memory pointed to by the pointer at will. The caller must ensure
    /// that nothing else uses the pointer `ptr` after calling this method.
    ///
    /// # Examples
    ///
    /// Using memory that was allocated by an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// # use std::ptr::NonNull;
    /// # use std::ptr;
    /// # use std::mem;
    /// #
    /// let opaque_vec = {
    ///     let array: [i32; 3] = [1, 2, 3];
    ///     OpaqueVec::from(array)
    /// };
    ///
    /// // Prevent running `opaque_vec`'s destructor to completely control the allocation.
    /// let mut opaque_vec = mem::ManuallyDrop::new(opaque_vec);
    ///
    /// // Destructure `opaque_vec` into its constituent parts.
    /// let ptr: NonNull<i32> = opaque_vec.as_non_null::<i32, Global>();
    /// let length = opaque_vec.len();
    /// let capacity = opaque_vec.capacity();
    /// let proj_alloc: TypedProjAlloc<Global> = unsafe { ptr::read(opaque_vec.allocator::<i32, Global>()) };
    ///
    /// let expected = {
    ///     let array: [i32; 3] = [4, 5, 6];
    ///     OpaqueVec::from(array)
    /// };
    /// let result = unsafe {
    ///     // Mutate the values directly in memory.
    ///     for i in 0..length {
    ///         ptr::write(ptr.as_ptr().add(i), 4 + i as i32);
    ///     }
    ///
    ///     // Rebuild the vector.
    ///     OpaqueVec::from_parts_proj_in::<i32, Global>(ptr, length, capacity, proj_alloc)
    /// };
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert_eq!(result.capacity(), expected.capacity());
    /// # assert!(!result.is_empty());
    /// # assert_eq!(result.len(), length);
    /// # assert_eq!(result.capacity(), capacity);
    /// # assert!(result.len() <= result.capacity());
    /// # assert!(!expected.is_empty());
    /// # assert_eq!(expected.len(), length);
    /// # assert_eq!(expected.capacity(), capacity);
    /// # assert!(expected.len() <= expected.capacity());
    ///
    /// let mut result = result;
    /// let new_capacity = 16;
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// for _ in 0..(new_capacity - length) {
    ///     result.push::<i32, Global>(i32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     4,        5,        6,        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ///     i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert!(result.len() <= result.capacity());
    /// # assert_eq!(result.len(), new_capacity);
    /// # assert!(result.capacity() >= new_capacity);
    /// ```
    ///
    /// Using memory that was allocated outside an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Allocator;
    /// # use std::alloc::Global;
    /// # use std::alloc::Layout;
    /// # use std::ptr::NonNull;
    /// #
    /// let value = 1_000_000;
    /// let layout = Layout::array::<u32>(16).unwrap();
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let length = 1;
    /// let capacity = 16;
    /// let opaque_vec = unsafe {
    ///     let mut memory: NonNull<u32> = proj_alloc.allocate(layout).unwrap().cast::<u32>();
    ///     memory.write(value);
    ///
    ///     OpaqueVec::from_parts_proj_in::<u32, Global>(memory, length, capacity, proj_alloc)
    /// };
    ///
    /// assert_eq!(opaque_vec.as_slice::<u32, Global>(), &[value]);
    /// assert_eq!(opaque_vec.len(), length);
    /// assert_eq!(opaque_vec.capacity(), capacity);
    /// # assert!(!opaque_vec.is_empty());
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// let mut result = opaque_vec;
    /// for _ in 0..(capacity - length) {
    ///     result.push::<u32, Global>(u32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     value,     u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    ///     u32::MAX,  u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<u32, Global>(), expected.as_slice::<u32, Global>());
    /// assert_eq!(result.len(), result.capacity());
    /// # assert_eq!(result.len(), capacity);
    /// # assert_eq!(result.capacity(), capacity);
    /// ```
    ///
    /// [`dealloc`]: std::alloc::Allocator::dealloc
    #[inline]
    pub unsafe fn from_parts_proj_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, A>::from_parts_proj_in(ptr, length, capacity, proj_alloc)
        };

        Self::from_proj(proj_vec)
    }

    /// Constructs a new empty [`OpaqueVec`] using a specific memory allocator.
    ///
    /// The vector will not allocate until elements are pushed into it. In particular, the
    /// vector has zero capacity until elements are pushed into it.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_vec = OpaqueVec::new_in::<i32, Global>(Global);
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(opaque_vec.is_empty());
    /// assert_eq!(opaque_vec.capacity(), 0);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in<T, A>(alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = TypedProjVec::<T, A>::new_in(alloc);

        Self::from_proj(proj_vec)
    }

    /// Constructs a new empty [`OpaqueVec`] using a specific memory allocator and a
    /// specific capacity.
    ///
    /// The vector will be able to hold at least `capacity` elements without reallocating. The
    /// method _can_ allocate more than `capacity` elements. If `capacity` is zero, the
    /// constructor does not allocate memory, i.e. it is equivalent to [`new_in`] when `capacity` is
    /// zero.
    ///
    /// Note that while the returned vector will have a **capacity** of at least `capacity`, it will
    /// have a **length** of zero, because no elements have been pushed to the vector yet.
    ///
    /// # Panics
    ///
    /// This method panics if the new capacity exceeds `isize::MAX` _bytes_.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 32;
    /// let opaque_vec = OpaqueVec::with_capacity_in::<i32, Global>(capacity, Global);
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(opaque_vec.capacity() >= capacity);
    /// assert!(opaque_vec.is_empty());
    ///
    /// let empty_vec = OpaqueVec::with_capacity_in::<i32, Global>(0, Global);
    ///
    /// assert_eq!(empty_vec.capacity(), 0);
    /// assert!(empty_vec.is_empty());
    /// ```
    ///
    /// [`new_in`]: OpaqueVec::new_in`
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in<T, A>(capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = TypedProjVec::<T, A>::with_capacity_in(capacity, alloc);

        Self::from_proj(proj_vec)
    }

    /// Constructs a new empty [`OpaqueVec`] using a specific memory allocator and a
    /// specific capacity.
    ///
    /// The vector will be able to hold at least `capacity` elements without reallocating. The
    /// method _can_ allocate more than `capacity` elements. If `capacity` is zero, the
    /// constructor does not allocate memory, i.e. it is equivalent to [`new_in`] when `capacity` is
    /// zero.
    ///
    /// Note that while the returned vector will have a **capacity** of at least `capacity`, it will
    /// have a **length** of zero, because no elements have been pushed to the vector yet.
    ///
    /// # Errors
    ///
    /// This method returns an error if the capacity `capacity` exceeds `isize::MAX` bytes, or if the
    /// allocator reports an allocation failure.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 32;
    /// let opaque_vec = OpaqueVec::try_with_capacity_in::<i32, Global>(capacity, Global);
    ///
    /// assert!(opaque_vec.is_ok());
    ///
    /// let opaque_vec = opaque_vec.unwrap();
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(opaque_vec.capacity() >= capacity);
    /// assert!(opaque_vec.is_empty());
    ///
    /// let empty_vec = OpaqueVec::try_with_capacity_in::<i32, Global>(0, Global).unwrap();
    ///
    /// assert_eq!(empty_vec.capacity(), 0);
    /// assert!(empty_vec.is_empty());
    /// ```
    ///
    /// [`new_in`]: OpaqueVec::new_in
    #[inline]
    pub fn try_with_capacity_in<T, A>(capacity: usize, alloc: A) -> Result<Self, TryReserveError>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = TypedProjVec::<T, A>::try_with_capacity_in(capacity, alloc)?;

        Ok(Self::from_proj(proj_vec))
    }

    /// Constructs an [`OpaqueVec`] directly from a pointer, a length, a capacity, and a
    /// memory allocator.
    ///
    /// # Safety
    ///
    /// This method is highly unsafe. A safe use of it must satisfy the following invariants:
    ///
    /// * The pointer `ptr` must be non-null.
    /// * The allocation referred to by `ptr` must have been allocated using the supplied
    ///   allocator.
    /// * The element type `T` must have the same alignment that `ptr` was allocated with.
    ///   The type `T` cannot have a less strict alignment is not sufficient; the alignment really
    ///   must be equal to satisfy the [`dealloc`] requirement that memory must be allocated and
    ///   deallocated with the same layout.
    /// * The allocation size in bytes (`mem::size_of::<T>() * capacity`) must
    ///   be the same size as the pointer was allocated with. Similar to alignment, [`dealloc`] must
    ///   be called with the same layout `size`.
    /// * The length `length` of the elements inside the allocation must be less than or equal to
    ///   the capacity `capacity`.
    /// * The first `length` values must be properly initialized values of type `T`.
    /// * `capacity` must be the capacity that the pointer was allocated with.
    /// * The allocated size in bytes must be no larger than `isize::MAX`.
    ///   See the safety documentation of [`pointer::offset`].
    ///
    /// These requirements always hold for any `ptr` that has been allocated via [`OpaqueVec`].
    ///
    /// The ownership of `ptr` is effectively transferred to the
    /// [`OpaqueVec`] which may then deallocate, reallocate or change the
    /// contents of memory pointed to by the pointer at will. The caller must ensure
    /// that nothing else uses the pointer `ptr` after calling this method.
    ///
    /// # Examples
    ///
    /// Using memory that was allocated by an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// # use std::ptr;
    /// # use std::mem;
    /// #
    /// let opaque_vec = {
    ///     let array: [i32; 3] = [1, 2, 3];
    ///     OpaqueVec::from(array)
    /// };
    ///
    /// // Prevent running `opaque_vec`'s destructor to completely control the allocation.
    /// let mut opaque_vec = mem::ManuallyDrop::new(opaque_vec);
    ///
    /// // Destructure `opaque_vec` into its constituent parts.
    /// let ptr: *mut i32 = opaque_vec.as_mut_ptr::<i32, Global>();
    /// let length = opaque_vec.len();
    /// let capacity = opaque_vec.capacity();
    /// let alloc: Global = Global;
    ///
    /// let expected = {
    ///     let array: [i32; 3] = [4, 5, 6];
    ///     OpaqueVec::from(array)
    /// };
    /// let result = unsafe {
    ///     // Mutate the values directly in memory.
    ///     for i in 0..length {
    ///         ptr::write(ptr.add(i), 4 + i as i32);
    ///     }
    ///
    ///     // Rebuild the vector.
    ///     OpaqueVec::from_raw_parts_in::<i32, Global>(ptr, length, capacity, alloc)
    /// };
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert_eq!(result.capacity(), expected.capacity());
    /// # assert!(!result.is_empty());
    /// # assert_eq!(result.len(), length);
    /// # assert_eq!(result.capacity(), capacity);
    /// # assert!(result.len() <= result.capacity());
    /// # assert!(!expected.is_empty());
    /// # assert_eq!(expected.len(), length);
    /// # assert_eq!(expected.capacity(), capacity);
    /// # assert!(expected.len() <= expected.capacity());
    ///
    /// let mut result = result;
    /// let new_capacity = 16;
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// for _ in 0..(new_capacity - length) {
    ///     result.push::<i32, Global>(i32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     4,        5,        6,        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ///     i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert!(result.len() <= result.capacity());
    /// # assert_eq!(result.len(), new_capacity);
    /// # assert!(result.capacity() >= new_capacity);
    /// ```
    ///
    /// Using memory that was allocated outside an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Allocator;
    /// # use std::alloc::Global;
    /// # use std::alloc::Layout;
    /// # use std::ptr::NonNull;
    /// #
    /// let value = 1_000_000;
    /// let layout = Layout::array::<u32>(16).unwrap();
    /// let alloc: Global = Global;
    /// let length = 1;
    /// let capacity = 16;
    /// let opaque_vec = unsafe {
    ///     let mut memory: NonNull<u32> = alloc.allocate(layout).unwrap().cast::<u32>();
    ///     memory.write(value);
    ///
    ///     OpaqueVec::from_raw_parts_in::<u32, Global>(memory.as_mut() as *mut u32, length, capacity, alloc)
    /// };
    ///
    /// assert_eq!(opaque_vec.as_slice::<u32, Global>(), &[value]);
    /// assert_eq!(opaque_vec.len(), length);
    /// assert_eq!(opaque_vec.capacity(), capacity);
    /// # assert!(!opaque_vec.is_empty());
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// let mut result = opaque_vec;
    /// for _ in 0..(capacity - length) {
    ///     result.push::<u32, Global>(u32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     value,     u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    ///     u32::MAX,  u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<u32, Global>(), expected.as_slice::<u32, Global>());
    /// assert_eq!(result.len(), result.capacity());
    /// # assert_eq!(result.len(), capacity);
    /// # assert_eq!(result.capacity(), capacity);
    /// ```
    ///
    /// [`dealloc`]: std::alloc::Allocator::dealloc
    #[inline]
    pub unsafe fn from_raw_parts_in<T, A>(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, A>::from_raw_parts_in(ptr, length, capacity, alloc)
        };

        Self::from_proj(proj_vec)
    }

    /// Constructs an [`OpaqueVec`] directly from a pointer, a length, a capacity, and a
    /// memory allocator.
    ///
    /// # Safety
    ///
    /// This method is highly unsafe. A safe use of it must satisfy the following invariants:
    ///
    /// * The pointer `ptr` must be non-null.
    /// * The allocation referred to by `ptr` must have been allocated using the supplied
    ///   allocator.
    /// * The element type `T` must have the same alignment that `ptr` was allocated with.
    ///   The type `T` cannot have a less strict alignment is not sufficient; the alignment really
    ///   must be equal to satisfy the [`dealloc`] requirement that memory must be allocated and
    ///   deallocated with the same layout.
    /// * The allocation size in bytes (`mem::size_of::<T>() * capacity`) must
    ///   be the same size as the pointer was allocated with. Similar to alignment, [`dealloc`] must
    ///   be called with the same layout `size`.
    /// * The length `length` of the elements inside the allocation must be less than or equal to
    ///   the capacity `capacity`.
    /// * The first `length` values must be properly initialized values of type `T`.
    /// * `capacity` must be the capacity that the pointer was allocated with.
    /// * The allocated size in bytes must be no larger than `isize::MAX`.
    ///   See the safety documentation of [`pointer::offset`].
    ///
    /// These requirements always hold for any `ptr` that has been allocated via [`OpaqueVec`].
    ///
    /// The ownership of `ptr` is effectively transferred to the
    /// [`OpaqueVec`] which may then deallocate, reallocate or change the
    /// contents of memory pointed to by the pointer at will. The caller must ensure
    /// that nothing else uses the pointer `ptr` after calling this method.
    ///
    /// # Examples
    ///
    /// Using memory that was allocated by an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// # use std::ptr::NonNull;
    /// # use std::ptr;
    /// # use std::mem;
    /// #
    /// let opaque_vec = {
    ///     let array: [i32; 3] = [1, 2, 3];
    ///     OpaqueVec::from(array)
    /// };
    ///
    /// // Prevent running `opaque_vec`'s destructor to completely control the allocation.
    /// let mut opaque_vec = mem::ManuallyDrop::new(opaque_vec);
    ///
    /// // Destructure `opaque_vec` into its constituent parts.
    /// let ptr: NonNull<i32> = opaque_vec.as_non_null::<i32, Global>();
    /// let length = opaque_vec.len();
    /// let capacity = opaque_vec.capacity();
    /// let alloc: Global = Global;
    ///
    /// let expected = {
    ///     let array: [i32; 3] = [4, 5, 6];
    ///     OpaqueVec::from(array)
    /// };
    /// let result = unsafe {
    ///     // Mutate the values directly in memory.
    ///     for i in 0..length {
    ///         ptr::write(ptr.as_ptr().add(i), 4 + i as i32);
    ///     }
    ///
    ///     // Rebuild the vector.
    ///     OpaqueVec::from_parts_in::<i32, Global>(ptr, length, capacity, alloc)
    /// };
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert_eq!(result.capacity(), expected.capacity());
    /// # assert!(!result.is_empty());
    /// # assert_eq!(result.len(), length);
    /// # assert_eq!(result.capacity(), capacity);
    /// # assert!(result.len() <= result.capacity());
    /// # assert!(!expected.is_empty());
    /// # assert_eq!(expected.len(), length);
    /// # assert_eq!(expected.capacity(), capacity);
    /// # assert!(expected.len() <= expected.capacity());
    ///
    /// let mut result = result;
    /// let new_capacity = 16;
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// for _ in 0..(new_capacity - length) {
    ///     result.push::<i32, Global>(i32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     4,        5,        6,        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ///     i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert!(result.len() <= result.capacity());
    /// # assert_eq!(result.len(), new_capacity);
    /// # assert!(result.capacity() >= new_capacity);
    /// ```
    ///
    /// Using memory that was allocated outside an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Allocator;
    /// # use std::alloc::Global;
    /// # use std::alloc::Layout;
    /// # use std::ptr::NonNull;
    /// #
    /// let value = 1_000_000;
    /// let layout = Layout::array::<u32>(16).unwrap();
    /// let alloc: Global = Global;
    /// let length = 1;
    /// let capacity = 16;
    /// let opaque_vec = unsafe {
    ///     let mut memory: NonNull<u32> = alloc.allocate(layout).unwrap().cast::<u32>();
    ///     memory.write(value);
    ///
    ///     OpaqueVec::from_parts_in::<u32, Global>(memory, length, capacity, alloc)
    /// };
    ///
    /// assert_eq!(opaque_vec.as_slice::<u32, Global>(), &[value]);
    /// assert_eq!(opaque_vec.len(), length);
    /// assert_eq!(opaque_vec.capacity(), capacity);
    /// # assert!(!opaque_vec.is_empty());
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// let mut result = opaque_vec;
    /// for _ in 0..(capacity - length) {
    ///     result.push::<u32, Global>(u32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     value,     u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    ///     u32::MAX,  u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<u32, Global>(), expected.as_slice::<u32, Global>());
    /// assert_eq!(result.len(), result.capacity());
    /// # assert_eq!(result.len(), capacity);
    /// # assert_eq!(result.capacity(), capacity);
    /// ```
    ///
    /// [`dealloc`]: std::alloc::Allocator::dealloc
    #[inline]
    pub unsafe fn from_parts_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, A>::from_parts_in(ptr, length, capacity, alloc)
        };

        Self::from_proj(proj_vec)
    }
}

impl OpaqueVec {
    /// Constructs a new empty [`OpaqueVec`].
    ///
    /// The vector will not allocate until elements are pushed into it. In particular, the
    /// vector has zero capacity until elements are pushed into it.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_vec = OpaqueVec::new::<i32>();
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(opaque_vec.is_empty());
    /// assert_eq!(opaque_vec.capacity(), 0);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new<T>() -> Self
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, alloc::Global>::new();

        Self::from_proj(proj_vec)
    }

    /// Constructs a new empty [`OpaqueVec`] using a specific capacity.
    ///
    /// The vector will be able to hold at least `capacity` elements without reallocating. The
    /// method _can_ allocate more than `capacity` elements. If `capacity` is zero, the
    /// constructor does not allocate memory, i.e. it is equivalent to [`new`] when `capacity` is
    /// zero.
    ///
    /// Note that while the returned vector will have a **capacity** of at least `capacity`, it will
    /// have a **length** of zero, because no elements have been pushed to the vector yet.
    ///
    /// # Panics
    ///
    /// This method panics if the new capacity exceeds `isize::MAX` _bytes_.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 32;
    /// let opaque_vec = OpaqueVec::with_capacity::<i32>(capacity);
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(opaque_vec.capacity() >= capacity);
    /// assert!(opaque_vec.is_empty());
    ///
    /// let empty_vec = OpaqueVec::with_capacity::<i32>(0);
    ///
    /// assert_eq!(empty_vec.capacity(), 0);
    /// assert!(empty_vec.is_empty());
    /// ```
    ///
    /// [`new`]: OpaqueVec::new
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity<T>(capacity: usize) -> Self
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, alloc::Global>::with_capacity(capacity);

        Self::from_proj(proj_vec)
    }

    /// Constructs a new empty [`OpaqueVec`] using a specific capacity.
    ///
    /// The vector will be able to hold at least `capacity` elements without reallocating. The
    /// method _can_ allocate more than `capacity` elements. If `capacity` is zero, the
    /// constructor does not allocate memory, i.e. it is equivalent to [`new`] when `capacity` is
    /// zero.
    ///
    /// Note that while the returned vector will have a **capacity** of at least `capacity`, it will
    /// have a **length** of zero, because no elements have been pushed to the vector yet.
    ///
    /// # Errors
    ///
    /// This method returns an error if the capacity `capacity` exceeds `isize::MAX` bytes, or if the
    /// allocator reports an allocation failure.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 32;
    /// let opaque_vec = OpaqueVec::try_with_capacity::<i32>(capacity);
    ///
    /// assert!(opaque_vec.is_ok());
    ///
    /// let opaque_vec = opaque_vec.unwrap();
    ///
    /// assert!(opaque_vec.has_element_type::<i32>());
    /// assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(opaque_vec.capacity() >= capacity);
    /// assert!(opaque_vec.is_empty());
    ///
    /// let empty_vec = OpaqueVec::try_with_capacity::<i32>(0).unwrap();
    ///
    /// assert_eq!(empty_vec.capacity(), 0);
    /// assert!(empty_vec.is_empty());
    /// ```
    ///
    /// [`new`]: OpaqueVec::new
    #[inline]
    pub fn try_with_capacity<T>(capacity: usize) -> Result<Self, TryReserveError>
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, alloc::Global>::try_with_capacity(capacity)?;

        Ok(Self::from_proj(proj_vec))
    }

    /// Constructs an [`OpaqueVec`] directly from a pointer, a length, and a capacity.
    ///
    /// # Safety
    ///
    /// This method is highly unsafe. A safe use of it must satisfy the following invariants:
    ///
    /// * The pointer `ptr` must be non-null.
    /// * The allocation referred to by `ptr` must have been allocated using the global allocator.
    /// * The element type `T` must have the same alignment that `ptr` was allocated with.
    ///   The type `T` cannot have a less strict alignment is not sufficient; the alignment really
    ///   must be equal to satisfy the [`dealloc`] requirement that memory must be allocated and
    ///   deallocated with the same layout.
    /// * The allocation size in bytes (`mem::size_of::<T>() * capacity`) must
    ///   be the same size as the pointer was allocated with. Similar to alignment, [`dealloc`] must
    ///   be called with the same layout `size`.
    /// * The length `length` of the elements inside the allocation must be less than or equal to
    ///   the capacity `capacity`.
    /// * The first `length` values must be properly initialized values of type `T`.
    /// * `capacity` must be the capacity that the pointer was allocated with.
    /// * The allocated size in bytes must be no larger than `isize::MAX`.
    ///   See the safety documentation of [`pointer::offset`].
    ///
    /// These requirements always hold for any `ptr` that has been allocated via [`OpaqueVec`].
    ///
    /// The ownership of `ptr` is effectively transferred to the
    /// [`OpaqueVec`] which may then deallocate, reallocate or change the
    /// contents of memory pointed to by the pointer at will. The caller must ensure
    /// that nothing else uses the pointer `ptr` after calling this method.
    ///
    /// # Examples
    ///
    /// Using memory that was allocated by an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// # use std::ptr;
    /// # use std::mem;
    /// #
    /// let opaque_vec = {
    ///     let array: [i32; 3] = [1, 2, 3];
    ///     OpaqueVec::from(array)
    /// };
    ///
    /// // Prevent running `opaque_vec`'s destructor to completely control the allocation.
    /// let mut opaque_vec = mem::ManuallyDrop::new(opaque_vec);
    ///
    /// // Destructure `opaque_vec` into its constituent parts.
    /// let ptr: *mut i32 = opaque_vec.as_mut_ptr::<i32, Global>();
    /// let length = opaque_vec.len();
    /// let capacity = opaque_vec.capacity();
    ///
    /// let expected = {
    ///     let array: [i32; 3] = [4, 5, 6];
    ///     OpaqueVec::from(array)
    /// };
    /// let result = unsafe {
    ///     // Mutate the values directly in memory.
    ///     for i in 0..length {
    ///         ptr::write(ptr.add(i), 4 + i as i32);
    ///     }
    ///
    ///     // Rebuild the vector.
    ///     OpaqueVec::from_raw_parts::<i32>(ptr, length, capacity)
    /// };
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert_eq!(result.capacity(), expected.capacity());
    /// # assert!(!result.is_empty());
    /// # assert_eq!(result.len(), length);
    /// # assert_eq!(result.capacity(), capacity);
    /// # assert!(result.len() <= result.capacity());
    /// # assert!(!expected.is_empty());
    /// # assert_eq!(expected.len(), length);
    /// # assert_eq!(expected.capacity(), capacity);
    /// # assert!(expected.len() <= expected.capacity());
    ///
    /// let mut result = result;
    /// let new_capacity = 16;
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// for _ in 0..(new_capacity - length) {
    ///     result.push::<i32, Global>(i32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     4,        5,        6,        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ///     i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert!(result.len() <= result.capacity());
    /// # assert_eq!(result.len(), new_capacity);
    /// # assert!(result.capacity() >= new_capacity);
    /// ```
    ///
    /// Using memory that was allocated outside an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Allocator;
    /// # use std::alloc::Global;
    /// # use std::alloc::Layout;
    /// # use std::ptr::NonNull;
    /// #
    /// let value = 1_000_000;
    /// let layout = Layout::array::<u32>(16).unwrap();
    /// let length = 1;
    /// let capacity = 16;
    /// let opaque_vec = unsafe {
    ///     let mut memory: NonNull<u32> = Global.allocate(layout).unwrap().cast::<u32>();
    ///     memory.write(value);
    ///
    ///     OpaqueVec::from_raw_parts::<u32>(memory.as_mut() as *mut u32, length, capacity)
    /// };
    ///
    /// assert_eq!(opaque_vec.as_slice::<u32, Global>(), &[value]);
    /// assert_eq!(opaque_vec.len(), length);
    /// assert_eq!(opaque_vec.capacity(), capacity);
    /// # assert!(!opaque_vec.is_empty());
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// let mut result = opaque_vec;
    /// for _ in 0..(capacity - length) {
    ///     result.push::<u32, Global>(u32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     value,     u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    ///     u32::MAX,  u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<u32, Global>(), expected.as_slice::<u32, Global>());
    /// assert_eq!(result.len(), result.capacity());
    /// # assert_eq!(result.len(), capacity);
    /// # assert_eq!(result.capacity(), capacity);
    /// ```
    ///
    /// [`dealloc`]: std::alloc::Allocator::dealloc
    #[inline]
    pub unsafe fn from_raw_parts<T>(ptr: *mut T, length: usize, capacity: usize) -> Self
    where
        T: any::Any,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, alloc::Global>::from_raw_parts(ptr, length, capacity)
        };

        Self::from_proj(proj_vec)
    }

    /// Constructs an [`OpaqueVec`] directly from a pointer, a length, and a capacity.
    ///
    /// # Safety
    ///
    /// This method is highly unsafe. A safe use of it must satisfy the following invariants:
    ///
    /// * The pointer `ptr` must be non-null.
    /// * The allocation referred to by `ptr` must have been allocated using the global allocator.
    /// * The element type `T` must have the same alignment that `ptr` was allocated with.
    ///   The type `T` cannot have a less strict alignment is not sufficient; the alignment really
    ///   must be equal to satisfy the [`dealloc`] requirement that memory must be allocated and
    ///   deallocated with the same layout.
    /// * The allocation size in bytes (`mem::size_of::<T>() * capacity`) must
    ///   be the same size as the pointer was allocated with. Similar to alignment, [`dealloc`] must
    ///   be called with the same layout `size`.
    /// * The length `length` of the elements inside the allocation must be less than or equal to
    ///   the capacity `capacity`.
    /// * The first `length` values must be properly initialized values of type `T`.
    /// * `capacity` must be the capacity that the pointer was allocated with.
    /// * The allocated size in bytes must be no larger than `isize::MAX`.
    ///   See the safety documentation of [`pointer::offset`].
    ///
    /// These requirements always hold for any `ptr` that has been allocated via [`OpaqueVec`].
    ///
    /// The ownership of `ptr` is effectively transferred to the
    /// [`OpaqueVec`] which may then deallocate, reallocate or change the
    /// contents of memory pointed to by the pointer at will. The caller must ensure
    /// that nothing else uses the pointer `ptr` after calling this method.
    ///
    /// # Examples
    ///
    /// Using memory that was allocated by an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// # use std::ptr::NonNull;
    /// # use std::ptr;
    /// # use std::mem;
    /// #
    /// let opaque_vec = {
    ///     let array: [i32; 3] = [1, 2, 3];
    ///     OpaqueVec::from(array)
    /// };
    ///
    /// // Prevent running `opaque_vec`'s destructor to completely control the allocation.
    /// let mut opaque_vec = mem::ManuallyDrop::new(opaque_vec);
    ///
    /// // Destructure `opaque_vec` into its constituent parts.
    /// let ptr: NonNull<i32> = opaque_vec.as_non_null::<i32, Global>();
    /// let length = opaque_vec.len();
    /// let capacity = opaque_vec.capacity();
    ///
    /// let expected = {
    ///     let array: [i32; 3] = [4, 5, 6];
    ///     OpaqueVec::from(array)
    /// };
    /// let result = unsafe {
    ///     // Mutate the values directly in memory.
    ///     for i in 0..length {
    ///         ptr::write(ptr.as_ptr().add(i), 4 + i as i32);
    ///     }
    ///
    ///     // Rebuild the vector.
    ///     OpaqueVec::from_parts::<i32>(ptr, length, capacity)
    /// };
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert_eq!(result.capacity(), expected.capacity());
    /// # assert!(!result.is_empty());
    /// # assert_eq!(result.len(), length);
    /// # assert_eq!(result.capacity(), capacity);
    /// # assert!(result.len() <= result.capacity());
    /// # assert!(!expected.is_empty());
    /// # assert_eq!(expected.len(), length);
    /// # assert_eq!(expected.capacity(), capacity);
    /// # assert!(expected.len() <= expected.capacity());
    ///
    /// let mut result = result;
    /// let new_capacity = 16;
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// for _ in 0..(new_capacity - length) {
    ///     result.push::<i32, Global>(i32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     4,        5,        6,        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ///     i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// assert_eq!(result.len(), expected.len());
    /// assert!(result.len() <= result.capacity());
    /// # assert_eq!(result.len(), new_capacity);
    /// # assert!(result.capacity() >= new_capacity);
    /// ```
    ///
    /// Using memory that was allocated outside an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Allocator;
    /// # use std::alloc::Global;
    /// # use std::alloc::Layout;
    /// # use std::ptr::NonNull;
    /// #
    /// let value = 1_000_000;
    /// let layout = Layout::array::<u32>(16).unwrap();
    /// let length = 1;
    /// let capacity = 16;
    /// let opaque_vec = unsafe {
    ///     let mut memory: NonNull<u32> = Global.allocate(layout).unwrap().cast::<u32>();
    ///     memory.write(value);
    ///
    ///     OpaqueVec::from_parts::<u32>(memory, length, capacity)
    /// };
    ///
    /// assert_eq!(opaque_vec.as_slice::<u32, Global>(), &[value]);
    /// assert_eq!(opaque_vec.len(), length);
    /// assert_eq!(opaque_vec.capacity(), capacity);
    /// # assert!(!opaque_vec.is_empty());
    ///
    /// // It is safe to work further with the vector since it satisfies the required invariants.
    /// let mut result = opaque_vec;
    /// for _ in 0..(capacity - length) {
    ///     result.push::<u32, Global>(u32::MAX);
    /// }
    ///
    /// let expected = OpaqueVec::from([
    ///     value,     u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    ///     u32::MAX,  u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX,
    /// ]);
    ///
    /// assert_eq!(result.as_slice::<u32, Global>(), expected.as_slice::<u32, Global>());
    /// assert_eq!(result.len(), result.capacity());
    /// # assert_eq!(result.len(), capacity);
    /// # assert_eq!(result.capacity(), capacity);
    /// ```
    ///
    /// [`dealloc`]: std::alloc::Allocator::dealloc
    #[inline]
    pub unsafe fn from_parts<T>(ptr: NonNull<T>, length: usize, capacity: usize) -> Self
    where
        T: any::Any,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, alloc::Global>::from_parts(ptr, length, capacity)
        };

        Self::from_proj(proj_vec)
    }
}

impl OpaqueVec {
    /// Returns the memory layout of the elements inside an [`OpaqueVec`].
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::{Global, Layout};
    /// #
    /// struct Rgb { r: u8, g: u8, b: u8, }
    ///
    /// impl Rgb {
    ///     fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b, }}
    /// }
    ///
    /// let capacity = 32;
    /// let mut opaque_vec = OpaqueVec::with_capacity_in::<Rgb, Global>(capacity, Global);
    /// #
    /// # assert!(opaque_vec.has_element_type::<Rgb>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    ///
    /// let expected = Layout::new::<Rgb>();
    /// let result = opaque_vec.element_layout();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub const fn element_layout(&self) -> alloc::Layout {
        self.inner.element_layout()
    }

    /// Returns the capacity of an [`OpaqueVec`].
    ///
    /// The **capacity** of an [`OpaqueVec`] is the number of elements the vector can hold
    /// without reallocating memory.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 32;
    /// let mut opaque_vec = OpaqueVec::with_capacity_in::<i32, Global>(capacity, Global);
    ///
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(opaque_vec.capacity() >= capacity);
    /// assert_eq!(opaque_vec.len(), 0);
    ///
    /// for i in 0..capacity {
    ///     opaque_vec.push::<i32, Global>(i as i32);
    /// }
    ///
    /// assert!(opaque_vec.capacity() >= capacity);
    /// assert_eq!(opaque_vec.len(), capacity);
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Determine whether an [`OpaqueVec`] is empty or not.
    ///
    /// An [`OpaqueVec`] is **empty** if it contains no elements, i.e.
    /// its length is zero.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = OpaqueVec::with_capacity_in::<i32, Global>(1, Global);
    ///
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert!(opaque_vec.is_empty());
    ///
    /// opaque_vec.push::<i32, Global>(1);
    ///
    /// assert!(!opaque_vec.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the length of an [`OpaqueVec`].
    ///
    /// The **length** of an [`OpaqueVec`] is the number of elements stored inside it.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let len = 32;
    /// let mut opaque_vec = OpaqueVec::with_capacity_in::<i32, Global>(len, Global);
    ///
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// assert_eq!(opaque_vec.len(), 0);
    ///
    /// for i in 0..len {
    ///     opaque_vec.push::<i32, Global>(i as i32);
    /// }
    ///
    /// assert_eq!(opaque_vec.len(), len);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }
}

impl OpaqueVec {
    /// Returns a reference to the type-projected memory allocator from the vector.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_vec = OpaqueVec::new::<i32>();
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    ///
    /// assert!(opaque_vec.is_empty());
    ///
    /// let alloc: &TypedProjAlloc<Global> = opaque_vec.allocator::<i32, Global>();
    /// ```
    #[inline]
    pub fn allocator<T, A>(&self) -> &TypedProjAlloc<A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.allocator()
    }
}

impl OpaqueVec {
    /// Forces the length of and [`OpaqueVec`] to be set to `new_len`.
    ///
    /// This is a low-level operation that does not maintain the invariants of the [`OpaqueVec`].
    /// Normally one changes the length of the collection using operations such as [`truncate`],
    /// [`extend`], [`resize`], or [`clear`].
    ///
    /// Note that reducing the length of an [`OpaqueVec`] using this method will not drop the truncated
    /// elements. If those elements own heap-allocated memory or other resources (such as `Box`, `Vec`,
    /// or custom types with destructors), this will result in a memory leak.
    ///
    /// # Safety
    ///
    /// This method is safe to call if the following conditions hold:
    /// * The length `new_len` is less than or equal to `self.capacity()`.
    /// * The elements in the subslice `self.len()..new_len` must be initialized.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Safely reducing the length of an [`OpaqueVec`] with this method.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// # use std::ptr;
    /// #
    /// struct DropCounter {}
    ///
    /// static mut DROP_COUNT: u32 = 0;
    ///
    /// impl Drop for DropCounter {
    ///     fn drop(&mut self) {
    ///         unsafe { DROP_COUNT += 1; }
    ///     }
    /// }
    ///
    /// let capacity = 4;
    /// let mut opaque_vec = OpaqueVec::with_capacity::<Box<DropCounter>>(capacity);
    /// #
    /// # assert!(opaque_vec.has_element_type::<Box<DropCounter>>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// opaque_vec.push::<Box<DropCounter>, Global>(Box::new(DropCounter {}));
    /// opaque_vec.push::<Box<DropCounter>, Global>(Box::new(DropCounter {}));
    /// opaque_vec.push::<Box<DropCounter>, Global>(Box::new(DropCounter {}));
    ///
    /// assert_eq!(opaque_vec.len(), 3);
    /// assert!(opaque_vec.capacity() >= capacity);
    /// unsafe {
    ///     let ptr = opaque_vec.as_mut_ptr::<Box<DropCounter>, Global>();
    ///     // Read, then drop the last two elements.
    ///     let _: Box<DropCounter> = ptr::read(ptr.add(2));
    ///     let _: Box<DropCounter> = ptr::read(ptr.add(1));
    ///     opaque_vec.set_len::<Box<DropCounter>, Global>(1);
    /// }
    ///
    /// assert_eq!(opaque_vec.len(), 1);
    /// assert!(opaque_vec.capacity() >= capacity);
    ///
    /// // No data leaks because we dropped then shrank the length.
    /// assert_eq!(unsafe { DROP_COUNT }, 2);
    /// ```
    ///
    /// Safely extending the length of an [`OpaqueVec`] with this method without leaking memory.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// # use std::ptr;
    /// #
    /// struct DropCounter {}
    ///
    /// static mut DROP_COUNT: u32 = 0;
    ///
    /// impl Drop for DropCounter {
    ///     fn drop(&mut self) {
    ///         unsafe { DROP_COUNT += 1; }
    ///     }
    /// }
    ///
    /// let capacity = 4;
    /// let mut opaque_vec = OpaqueVec::with_capacity::<Box<DropCounter>>(capacity);
    /// #
    /// # assert!(opaque_vec.has_element_type::<Box<DropCounter>>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert_eq!(opaque_vec.len(), 0);
    /// assert!(opaque_vec.capacity() >= capacity);
    /// unsafe {
    ///     let ptr = opaque_vec.as_mut_ptr::<Box<DropCounter>, Global>();
    ///     // Write the elements into the allocation directly.
    ///     ptr::write(ptr.add(0), Box::new(DropCounter {}));
    ///     ptr::write(ptr.add(1), Box::new(DropCounter {}));
    ///     ptr::write(ptr.add(2), Box::new(DropCounter {}));
    ///     opaque_vec.set_len::<Box<DropCounter>, Global>(3);
    /// }
    ///
    /// assert_eq!(opaque_vec.len(), 3);
    /// assert!(opaque_vec.capacity() >= capacity);
    ///
    /// // Not data leaks after writing directly into the allocation.
    /// assert_eq!(unsafe { DROP_COUNT }, 0);
    /// ```
    ///
    /// Safely extending the length of an [`OpaqueVec`] with this method.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// # use std::ptr;
    /// #
    /// let capacity = 4;
    /// let mut opaque_vec = OpaqueVec::with_capacity::<i32>(capacity);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert_eq!(opaque_vec.len(), 0);
    /// assert!(opaque_vec.capacity() >= capacity);
    /// unsafe {
    ///     let ptr = opaque_vec.as_mut_ptr::<i32, Global>();
    ///     // Write the elements into the allocation directly.
    ///     ptr::write(ptr.add(0), 1);
    ///     ptr::write(ptr.add(1), 2);
    ///     ptr::write(ptr.add(2), 3);
    ///     opaque_vec.set_len::<i32, Global>(3);
    /// }
    ///
    /// assert_eq!(opaque_vec.len(), 3);
    /// assert!(opaque_vec.capacity() >= capacity);
    ///
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1, 2, 3]);
    /// ```
    ///
    /// [`truncate`]: Vec::truncate
    /// [`resize`]: Vec::resize
    /// [`extend`]: Extend::extend
    /// [`clear`]: Vec::clear
    #[inline]
    pub unsafe fn set_len<T, A>(&mut self, new_len: usize)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        unsafe {
            proj_self.set_len(new_len);
        }
    }

    /// Returns a reference to an element or subslice of an [`OpaqueVec`], if it exists at the
    /// given index or inside the given subslice.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions holds:
    /// * The [`TypeId`] of the elements of `self` and the [`TypeId`] of the memory allocator of
    ///   `self` do not match the requested element type `T` and allocator type `A`, respectively.
    /// * If `index` is a scalar index, and `index` is out of bounds.
    /// * If `index` is a slice range, and a subslice of `index` falls out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 3] = [10, 40, 30];
    /// let opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    ///
    /// unsafe {
    ///     assert_eq!(opaque_vec.get_unchecked::<_, i32, Global>(0), &10);
    ///     assert_eq!(opaque_vec.get_unchecked::<_, i32, Global>(1), &40);
    ///     assert_eq!(opaque_vec.get_unchecked::<_, i32, Global>(2), &30);
    ///
    ///     assert_eq!(opaque_vec.get_unchecked::<_, i32, Global>(0..2), &[10, 40][..]);
    ///     assert_eq!(opaque_vec.get_unchecked::<_, i32, Global>(1..3), &[40, 30][..]);
    ///     assert_eq!(opaque_vec.get_unchecked::<_, i32, Global>(..), &[10, 40, 30][..]);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked<I, T, A>(&self, index: I) -> &<I as slice::SliceIndex<[T]>>::Output
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        I: slice::SliceIndex<[T]>,
    {
        let proj_self = self.as_proj::<T, A>();
        unsafe {
            proj_self.get_unchecked(index)
        }
    }

    /// Returns a mutable reference to an element or subslice of an [`OpaqueVec`], if it exists at the
    /// given index or inside the given subslice.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions holds:
    /// * The [`TypeId`] of the elements of `self` and the [`TypeId`] of the memory allocator of
    ///   `self` do not match the requested element type `T` and allocator type `A`, respectively.
    /// * If `index` is a scalar index, and `index` is out of bounds.
    /// * If `index` is a slice range, and a subslice of `index` falls out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 3] = [10, 40, 30];
    /// let mut opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    ///
    /// unsafe {
    ///     assert_eq!(opaque_vec.get_mut_unchecked::<_, i32, Global>(0), &10);
    ///     assert_eq!(opaque_vec.get_mut_unchecked::<_, i32, Global>(1), &40);
    ///     assert_eq!(opaque_vec.get_mut_unchecked::<_, i32, Global>(2), &30);
    ///
    ///     assert_eq!(opaque_vec.get_mut_unchecked::<_, i32, Global>(0..2), &[10, 40][..]);
    ///     assert_eq!(opaque_vec.get_mut_unchecked::<_, i32, Global>(1..3), &[40, 30][..]);
    ///     assert_eq!(opaque_vec.get_mut_unchecked::<_, i32, Global>(..), &[10, 40, 30][..]);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub unsafe fn get_mut_unchecked<I, T, A>(&mut self, index: I) -> &mut <I as slice::SliceIndex<[T]>>::Output
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        I: slice::SliceIndex<[T]>,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        unsafe {
            proj_self.get_mut_unchecked(index)
        }
    }

    /// Returns a reference to an element or subslice of an [`OpaqueVec`], if it exists at the
    /// given index or inside the given subslice.
    ///
    /// The method returns `None` from `self` under the following conditions:
    /// * If `index` is a scalar index, and `index` is out of bounds.
    /// * If `index` is a slice range, and a subslice of `index` falls out of bounds.
    /// The method returns some value or range of values otherwise.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 3] = [10, 40, 30];
    /// let opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    ///
    /// assert_eq!(opaque_vec.get::<_, i32, Global>(0), Some(&10));
    /// assert_eq!(opaque_vec.get::<_, i32, Global>(1), Some(&40));
    /// assert_eq!(opaque_vec.get::<_, i32, Global>(2), Some(&30));
    /// assert_eq!(opaque_vec.get::<_, i32, Global>(3), None);
    ///
    /// assert_eq!(opaque_vec.get::<_, i32, Global>(0..2), Some(&[10, 40][..]));
    /// assert_eq!(opaque_vec.get::<_, i32, Global>(1..3), Some(&[40, 30][..]));
    /// assert_eq!(opaque_vec.get::<_, i32, Global>(..), Some(&[10, 40, 30][..]));
    /// assert_eq!(opaque_vec.get::<_, i32, Global>(0..4), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get<I, T, A>(&self, index: I) -> Option<&<I as slice::SliceIndex<[T]>>::Output>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        I: slice::SliceIndex<[T]>,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.get(index)
    }

    /// Returns a mutable reference to an element or subslice of an [`OpaqueVec`], if it exists at the
    /// given index or inside the given subslice.
    ///
    /// The method returns `None` from `self` under the following conditions:
    /// * If `index` is a scalar index, and `index` is out of bounds.
    /// * If `index` is a slice range, and a subslice of `index` falls out of bounds.
    /// The method returns some value or range of values otherwise.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 3] = [10, 40, 30];
    /// let mut opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    ///
    /// assert_eq!(opaque_vec.get_mut::<_, i32, Global>(0), Some(&mut 10));
    /// assert_eq!(opaque_vec.get_mut::<_, i32, Global>(1), Some(&mut 40));
    /// assert_eq!(opaque_vec.get_mut::<_, i32, Global>(2), Some(&mut 30));
    /// assert_eq!(opaque_vec.get_mut::<_, i32, Global>(3), None);
    ///
    /// assert_eq!(opaque_vec.get_mut::<_, i32, Global>(0..2), Some(&mut [10, 40][..]));
    /// assert_eq!(opaque_vec.get_mut::<_, i32, Global>(1..3), Some(&mut [40, 30][..]));
    /// assert_eq!(opaque_vec.get_mut::<_, i32, Global>(..), Some(&mut [10, 40, 30][..]));
    /// assert_eq!(opaque_vec.get_mut::<_, i32, Global>(0..4), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_mut<I, T, A>(&mut self, index: I) -> Option<&mut <I as slice::SliceIndex<[T]>>::Output>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        I: slice::SliceIndex<[T]>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.get_mut(index)
    }

    /// Appends a new element to the end of an [`OpaqueVec`].
    ///
    /// # Panics
    ///
    /// This method panics if either condition occurs:
    /// * The [`TypeId`] of the elements of `self` and the [`TypeId`] of the memory allocator of
    ///   `self` do not match the requested element type `T` and allocator type `A`, respectively.
    /// * The new capacity exceeds `isize::MAX` _bytes_ if the [`OpaqueVec`] reallocates.
    /// Otherwise, the method succeeds.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 2] = [1, 2];
    /// let mut opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// opaque_vec.push::<i32, Global>(3);
    ///
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1, 2, 3]);
    /// ```
    #[inline]
    #[track_caller]
    pub fn push<T, A>(&mut self, value: T)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.push(value);
    }

    /// Returns the last element in an [`OpaqueVec`] if the vector is non-empty,
    /// and `None` if it is empty.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 3] = [1, 2, 3];
    /// let mut opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert!(!opaque_vec.is_empty());
    ///
    /// assert_eq!(opaque_vec.pop::<i32, Global>(), Some(3));
    /// assert_eq!(opaque_vec.pop::<i32, Global>(), Some(2));
    /// assert_eq!(opaque_vec.pop::<i32, Global>(), Some(1));
    ///
    /// assert!(opaque_vec.is_empty());
    ///
    /// assert_eq!(opaque_vec.pop::<i32, Global>(), None);
    /// ```
    #[inline]
    pub fn pop<T, A>(&mut self) -> Option<T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.pop()
    }

    /// Appends an element to an [`OpaqueVec`] if there is sufficient spare capacity. Otherwise, an
    /// error is returned with the element.
    ///
    /// Unlike [`push`], this method will not reallocate when there's insufficient
    /// capacity. The caller should use [`reserve`] or [`try_reserve`] to ensure that
    /// there is enough capacity.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// Pushing elements to the vector within the capacity of the vector.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let min_capacity = 4;
    /// let mut opaque_vec = OpaqueVec::with_capacity::<i32>(min_capacity);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// for i in 0..min_capacity{
    ///     let result = opaque_vec.push_within_capacity::<i32, Global>((i + 1) as i32);
    ///     assert!(result.is_ok());
    /// }
    /// assert!(opaque_vec.capacity() >= min_capacity);
    /// assert_eq!(opaque_vec.len(), min_capacity);
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1, 2, 3, 4]);
    /// ```
    ///
    /// Trying to push elements past the capacity of the vector.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let min_capacity = 4;
    /// let mut opaque_vec = OpaqueVec::with_capacity::<i32>(min_capacity);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert!(opaque_vec.capacity() >= min_capacity);
    /// let actual_capacity = opaque_vec.capacity();
    /// for i in 0..actual_capacity {
    ///     let result = opaque_vec.push_within_capacity::<i32, Global>((i + 1) as i32);
    ///     assert!(result.is_ok());
    ///     assert_eq!(opaque_vec.capacity(), actual_capacity);
    /// }
    ///
    /// let result = opaque_vec.push_within_capacity::<i32, Global>(i32::MAX);
    /// assert!(result.is_err());
    /// assert_eq!(opaque_vec.capacity(), actual_capacity);
    /// ```
    ///
    /// [`push`]: OpaqueVec::push
    /// [`reserve`]: OpaqueVec::reserve
    /// [`try_reserve`]: OpaqueVec::try_reserve
    #[inline]
    pub fn push_within_capacity<T, A>(&mut self, value: T) -> Result<(), T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.push_within_capacity(value)
    }

    /// Inserts a new value into an [`OpaqueVec`], replacing the old value.
    ///
    /// This method behaves with respect to `index` as follows:
    /// * If `index < self.len()`, it replaces the existing value at `index`.
    /// * If `index == self.len()`, it pushes `value` to the end of the collection.
    /// * If `index > self.len()`, it panics.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions occurs:
    /// * The [`TypeId`] of the elements of `self` and the [`TypeId`] of the memory allocator
    ///   of `self` do not match the requested element type `T` and allocator type `A`, respectively.
    /// * The index `index` is larger than the length of the collection.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = OpaqueVec::new::<i32>();
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert!(opaque_vec.is_empty());
    ///
    /// opaque_vec.replace_insert::<i32, Global>(0, 1);
    ///
    /// assert_eq!(opaque_vec.len(), 1);
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1]);
    ///
    /// opaque_vec.replace_insert::<i32, Global>(0, 2);
    ///
    /// assert_eq!(opaque_vec.len(), 1);
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[2]);
    /// ```
    #[track_caller]
    pub fn replace_insert<T, A>(&mut self, index: usize, value: T)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.replace_insert(index, value);
    }

    /// Inserts a new value into an [`OpaqueVec`], shifting the old value and all values after it to
    /// the right in the collection..
    ///
    /// This method behaves with respect to `index` as follows:
    /// * If `index < self.len()`, it shifts the current value at `index` and all successive values
    ///   in the collection to the right in the collection, reallocating if needed.
    /// * If `index == self.len()`, it pushes `value` to the end of the collection.
    /// * If `index > self.len()`, it panics.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions occurs:
    /// * The [`TypeId`] of the elements of `self` and the [`TypeId`] of the memory allocator
    ///   of `self` do not match the requested element type `T` and allocator type `A`, respectively.
    /// * The index `index` is larger than the length of the collection.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = OpaqueVec::new::<i32>();
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert!(opaque_vec.is_empty());
    ///
    /// opaque_vec.shift_insert::<i32, Global>(0, 1);
    ///
    /// assert_eq!(opaque_vec.len(), 1);
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1]);
    ///
    /// opaque_vec.shift_insert::<i32, Global>(0, 2);
    ///
    /// assert_eq!(opaque_vec.len(), 2);
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[2, 1]);
    /// ```
    #[track_caller]
    pub fn shift_insert<T, A>(&mut self, index: usize, value: T)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.shift_insert(index, value);
    }

    /// Removes a value from an [`OpaqueVec`], moving the last value in the collection to the index
    /// where the removed value occupies the collection.
    ///
    /// This method behaves with respect to `index` as follows:
    /// * If `index < self.len() - 1`, it moves the last value in the collection to the slot at
    ///   `index`, leaving the rest of the values in place.
    /// * If `index == self.len() - 1`, it removes the value from end of the collection with no
    ///   reordering of the remaining values in the collection.
    /// * If `index >= self.len()`, it panics.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions occurs:
    /// * The [`TypeId`] of the elements of `self` and the [`TypeId`] of the memory allocator
    ///   of `self` do not match the requested element type `T` and allocator type `A`, respectively.
    /// * The index `index` is larger than the length of the collection. In particular, the method
    ///   panics when `self` is empty.
    ///
    /// # Example
    ///
    /// Showing how swap removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 4] = [1, 2, 3, i32::MAX];
    /// let opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// {
    ///     let mut cloned = opaque_vec.clone::<i32, Global>();
    ///     cloned.swap_remove::<i32, Global>(3);
    ///     assert_eq!(cloned.as_slice::<i32, Global>(), &[1, 2, 3]);
    /// }
    /// {
    ///     let mut cloned = opaque_vec.clone::<i32, Global>();
    ///     cloned.swap_remove::<i32, Global>(2);
    ///     assert_eq!(cloned.as_slice::<i32, Global>(), &[1, 2, i32::MAX]);
    /// }
    /// {
    ///     let mut cloned = opaque_vec.clone::<i32, Global>();
    ///     cloned.swap_remove::<i32, Global>(1);
    ///     assert_eq!(cloned.as_slice::<i32, Global>(), &[1, i32::MAX, 3]);
    /// }
    /// {
    ///     let mut cloned = opaque_vec.clone::<i32, Global>();
    ///     cloned.swap_remove::<i32, Global>(0);
    ///     assert_eq!(cloned.as_slice::<i32, Global>(), &[i32::MAX, 2, 3]);
    /// }
    /// ```
    #[track_caller]
    pub fn swap_remove<T, A>(&mut self, index: usize) -> T
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.swap_remove(index)
    }

    /// Removes a value from an [`OpaqueVec`], shifting every successive value in the collection to
    /// the left one index to fill where the removed value occupies the collection.
    ///
    /// This method behaves with respect to `index` as follows:
    /// * If `index < self.len()`, it moves the every successive value in the collection to
    ///   the slot at `index` to the left one unit. Every value preceding the slot at `index` remains
    ///   in the same location. In particular, the method acts like a [`pop`] when the last value in
    ///   the collection is shift-removed, because the sub-collection of successor values is empty.
    /// * If `index >= self.len()`, it panics.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions occurs:
    /// * The [`TypeId`] of the elements of `self` and the [`TypeId`] of the memory allocator
    ///   of `self` do not match the requested element type `T` and allocator type `A`, respectively.
    /// * The index `index` is larger than the length of the collection. In particular, the method
    ///   panics when `self` is empty.
    ///
    /// # Example
    ///
    /// Showing how shift removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 4] = [1, 2, 3, i32::MAX];
    /// let opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// {
    ///     let mut cloned = opaque_vec.clone::<i32, Global>();
    ///     cloned.swap_remove::<i32, Global>(3);
    ///     assert_eq!(cloned.as_slice::<i32, Global>(), &[1, 2, 3]);
    /// }
    /// {
    ///     let mut cloned = opaque_vec.clone::<i32, Global>();
    ///     cloned.swap_remove::<i32, Global>(2);
    ///     assert_eq!(cloned.as_slice::<i32, Global>(), &[1, 2, i32::MAX]);
    /// }
    /// {
    ///     let mut cloned = opaque_vec.clone::<i32, Global>();
    ///     cloned.swap_remove::<i32, Global>(1);
    ///     assert_eq!(cloned.as_slice::<i32, Global>(), &[1, i32::MAX, 3]);
    /// }
    /// {
    ///     let mut cloned = opaque_vec.clone::<i32, Global>();
    ///     cloned.swap_remove::<i32, Global>(0);
    ///     assert_eq!(cloned.as_slice::<i32, Global>(), &[i32::MAX, 2, 3]);
    /// }
    /// ```
    ///
    /// [`pop`]: OpaqueVec::pop
    #[track_caller]
    pub fn shift_remove<T, A>(&mut self, index: usize) -> T
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.shift_remove(index)
    }

    /// Determines whether an [`OpaqueVec`] contains a value.
    ///
    /// The method returns `true` if `self` contains the value `value`. Returns `false` otherwise.
    /// In particular, the method always returns `false` when `self` is empty.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 9] = [92, 8, 40, 9, 8, 34, 59, 34, 5];
    /// let opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert!(opaque_vec.contains::<i32, Global>(&92));
    /// assert!(opaque_vec.contains::<i32, Global>(&8));
    /// assert!(opaque_vec.contains::<i32, Global>(&40));
    /// assert!(opaque_vec.contains::<i32, Global>(&9));
    /// assert!(opaque_vec.contains::<i32, Global>(&34));
    /// assert!(opaque_vec.contains::<i32, Global>(&5));
    ///
    /// assert!(!opaque_vec.contains::<i32, Global>(&100));
    /// assert!(!opaque_vec.contains::<i32, Global>(&91));
    /// assert!(!opaque_vec.contains::<i32, Global>(&93));
    /// assert!(!opaque_vec.contains::<i32, Global>(&7));
    /// assert!(!opaque_vec.contains::<i32, Global>(&10));
    /// assert!(!opaque_vec.contains::<i32, Global>(&33));
    /// assert!(!opaque_vec.contains::<i32, Global>(&35));
    /// assert!(!opaque_vec.contains::<i32, Global>(&4));
    /// assert!(!opaque_vec.contains::<i32, Global>(&6));
    /// ```
    pub fn contains<T, A>(&self, value: &T) -> bool
    where
        T: any::Any + PartialEq,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.contains(value)
    }

    /// Constructs an iterator over the elements of the [`OpaqueVec`].
    ///
    /// The iterator will yield all elements in the collection from start to end.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 6] = [92, 8, 40, 9, 8, 34];
    /// let opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// let mut iterator = opaque_vec.iter::<i32, Global>();
    /// assert_eq!(iterator.next(), Some(&92));
    /// assert_eq!(iterator.next(), Some(&8));
    /// assert_eq!(iterator.next(), Some(&40));
    /// assert_eq!(iterator.next(), Some(&9));
    /// assert_eq!(iterator.next(), Some(&8));
    /// assert_eq!(iterator.next(), Some(&34));
    /// assert_eq!(iterator.next(), None);
    ///
    /// // Every successive call to `iterator.next()` should yield a `None` value.
    /// for _ in 0..100 {
    ///     assert!(iterator.next().is_none());
    /// }
    /// ```
    pub fn iter<T, A>(&self) -> slice::Iter<'_, T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.iter()
    }

    /// Constructs a mutable iterator over the elements of the [`OpaqueVec`].
    ///
    /// The iterator will yield all elements in the collection from start to end.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 6] = [92, 8, 40, 9, 8, 34];
    /// let mut opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// let mut iterator = opaque_vec.iter_mut::<i32, Global>();
    /// assert_eq!(iterator.next(), Some(&mut 92));
    /// assert_eq!(iterator.next(), Some(&mut 8));
    /// assert_eq!(iterator.next(), Some(&mut 40));
    /// assert_eq!(iterator.next(), Some(&mut 9));
    /// assert_eq!(iterator.next(), Some(&mut 8));
    /// assert_eq!(iterator.next(), Some(&mut 34));
    /// assert_eq!(iterator.next(), None);
    ///
    /// // Every successive call to `iterator.next()` should yield a `None` value.
    /// for _ in 0..100 {
    ///     assert!(iterator.next().is_none());
    /// }
    /// ```
    pub fn iter_mut<T, A>(&mut self) -> slice::IterMut<'_, T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.iter_mut()
    }

    /// Constructs a consuming iterator for an [`OpaqueVec`]. A consuming iterator is an iterator
    /// that moves each value out of the collection from beginning to end.
    ///
    /// The method takes the [`OpaqueVec`], so that it cannot be used again.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [&'static str; 10] = [
    ///     "spam",
    ///     "eggs",
    ///     "sausage",
    ///     "spam",
    ///     "baked beans",
    ///     "spam",
    ///     "Lobster Thermidor aux Crevettes with a Mornay sauce, garnished with truffle pâté, brandy, with a fried egg on top, and spam",
    ///     "bacon",
    ///     "spam",
    ///     "I DON’T WANT SPAM!"
    /// ];
    /// let mut opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<&'static str>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// let mut iterator = opaque_vec.into_iter::<&'static str, Global>();
    ///
    /// assert_eq!(iterator.next(), Some("spam"));
    /// assert_eq!(iterator.next(), Some("eggs"));
    /// assert_eq!(iterator.next(), Some("sausage"));
    /// assert_eq!(iterator.next(), Some("spam"));
    /// assert_eq!(iterator.next(), Some("baked beans"));
    /// assert_eq!(iterator.next(), Some("spam"));
    /// assert_eq!(iterator.next(), Some("Lobster Thermidor aux Crevettes with a Mornay sauce, garnished with truffle pâté, brandy, with a fried egg on top, and spam"));
    /// assert_eq!(iterator.next(), Some("bacon"));
    /// assert_eq!(iterator.next(), Some("spam"));
    /// assert_eq!(iterator.next(), Some("I DON’T WANT SPAM!"));
    /// assert_eq!(iterator.next(), None);
    ///
    /// // Every successive call to `iterator.next()` should yield a `None` value.
    /// for _ in 0..100 {
    ///     assert_eq!(iterator.next(), None);
    /// }
    /// ```
    pub fn into_iter<T, A>(self) -> IntoIter<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.into_proj::<T, A>();

        proj_self.into_iter()
    }

    /// Appends one [`OpaqueVec`] to another [`OpaqueVec`], emptying the latter collection.
    ///
    /// This method drains `other` into `self`, i.e. every element of `other` will be appended
    /// to `self`, and `other` will be empty after the operation finishes.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively. Similarly, the method panics if the [`TypeId`] of
    /// the elements of `self` and `other` do not match, or the [`TypeId`] of the allocators of
    /// `self` and `other` do not match.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut result = {
    ///     let array: [i32; 4] = [1, 2, 3, 4];
    ///     OpaqueVec::from(array)
    /// };
    /// #
    /// # assert!(result.has_element_type::<i32>());
    /// # assert!(result.has_allocator_type::<Global>());
    /// #
    /// let mut appended = {
    ///     let array: [i32; 5] = [5, 6, 7, 8, 9];
    ///     OpaqueVec::from(array)
    /// };
    /// #
    /// # assert!(appended.has_element_type::<i32>());
    /// # assert!(appended.has_allocator_type::<Global>());
    /// #
    /// let expected = {
    ///     let array: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    ///     OpaqueVec::from(array)
    /// };
    /// #
    /// # assert!(expected.has_element_type::<i32>());
    /// # assert!(expected.has_allocator_type::<Global>());
    /// #
    /// result.append::<i32, Global>(&mut appended);
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// # assert_eq!(result.len(), 9);
    /// ```
    #[inline]
    #[track_caller]
    pub fn append<T, A>(&mut self, other: &mut Self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        let proj_other = other.as_proj_mut::<T, A>();

        proj_self.append(proj_other)
    }

    /// Removes the subslice indicated by the given range from the vector,
    /// returning a double-ended iterator over the removed subslice.
    ///
    /// If the iterator is dropped before being fully consumed,
    /// it drops the remaining removed elements.
    ///
    /// The returned iterator keeps a mutable borrow on the vector to optimize
    /// its implementation.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions holds:
    /// * The [`TypeId`] of the elements of `self` and the [`TypeId`] of the memory allocator of
    ///   `self` do not match the requested element type `T` and allocator type `A`, respectively.
    /// * If the range of the subslice falls outside the bounds of the collection. That is, if the
    ///   starting point of the subslice being removed starts after the end of `self`, or if the
    ///   ending point is larger than the length of the vector.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (due to
    /// [`mem::forget`], for example), the vector may have lost and leaked
    /// elements arbitrarily, including elements outside the range.
    ///
    /// # Examples
    ///
    /// Draining part of an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = {
    ///     let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    ///     OpaqueVec::from(array)
    /// };
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert_eq!(opaque_vec.len(), 6);
    ///
    /// let drained_vec: OpaqueVec = opaque_vec.drain::<_, i32, Global>(2..).collect();
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert_eq!(opaque_vec.len(), 2);
    /// assert_eq!(drained_vec.len(), 4);
    ///
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1, 2]);
    /// assert_eq!(drained_vec.as_slice::<i32, Global>(), &[3, 4, 5, 6]);
    /// ```
    ///
    /// Draining an entire [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = {
    ///     let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    ///     OpaqueVec::from(array)
    /// };
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert_eq!(opaque_vec.len(), 6);
    ///
    /// let drained_vec: OpaqueVec = opaque_vec.drain::<_, i32, Global>(..).collect();
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert_eq!(opaque_vec.len(), 0);
    /// assert_eq!(drained_vec.len(), 6);
    ///
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[]);
    /// assert_eq!(drained_vec.as_slice::<i32, Global>(), &[1, 2, 3, 4, 5, 6]);
    /// ```
    ///
    /// Draining no part of an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = {
    ///     let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    ///     OpaqueVec::from(array)
    /// };
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert_eq!(opaque_vec.len(), 6);
    ///
    /// let drained_vec: OpaqueVec = opaque_vec.drain::<_, i32, Global>(0..0).collect();
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert_eq!(opaque_vec.len(), 6);
    /// assert_eq!(drained_vec.len(), 0);
    ///
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1, 2, 3, 4, 5, 6]);
    /// assert_eq!(drained_vec.as_slice::<i32, Global>(), &[]);
    /// ```
    pub fn drain<R, T, A>(&mut self, range: R) -> Drain<'_, T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.drain(range)
    }

    /// Returns a raw pointer to the vector's buffer, or a dangling raw pointer
    /// valid for zero sized reads if the vector didn't allocate.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up dangling.
    /// Modifying the vector may cause its buffer to be reallocated,
    /// which would also make any pointers to it invalid.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an `UnsafeCell`) using this pointer or any pointer
    /// derived from it. If you need to mutate the contents of the slice, use
    /// [`as_mut_ptr`].
    ///
    /// This method guarantees that for the purpose of the aliasing model, this method
    /// does not materialize a reference to the underlying slice, and thus the returned pointer
    /// will remain valid when mixed with other calls to [`as_ptr`], [`as_mut_ptr`],
    /// and [`as_non_null`].
    ///
    /// Note that calling other methods that materialize mutable references to the slice,
    /// or mutable references to specific elements you are planning on accessing through this pointer,
    /// as well as writing to those elements, may still invalidate this pointer.
    /// See the second example below for how this guarantee can be used.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_vec = {
    ///     let array: [i32; 4] = [1, 2, 4, 8];
    ///     OpaqueVec::from(array)
    /// };
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// let ptr = opaque_vec.as_ptr::<i32, Global>();
    ///
    /// unsafe {
    ///     for i in 0..opaque_vec.len() {
    ///         assert_eq!(*ptr.add(i), 1 << i);
    ///     }
    /// }
    ///
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1, 2, 4, 8]);
    /// ```
    ///
    /// Due to the aliasing guarantee, the following code is legal:
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = {
    ///     let array: [i32; 3] = [0, 1, 2];
    ///     OpaqueVec::from(array)
    /// };
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// unsafe {
    ///     let ptr1 = opaque_vec.as_ptr::<i32, Global>();
    ///     let _ = ptr1.read();
    ///     let ptr2 = opaque_vec.as_mut_ptr::<i32, Global>().offset(2);
    ///     ptr2.write(2);
    ///     // Notably, the write to `ptr2` did **not** invalidate `ptr1`
    ///     // because it mutated a different element:
    ///     let _ = ptr1.read();
    /// }
    /// ```
    ///
    /// [`as_mut_ptr`]: OpaqueVec::as_mut_ptr
    /// [`as_ptr`]: OpaqueVec::as_ptr
    /// [`as_non_null`]: OpaqueVec::as_non_null
    #[inline]
    pub fn as_ptr<T, A>(&self) -> *const T
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.as_ptr()
    }

    /// Returns a raw mutable pointer to the vector's buffer, or a dangling
    /// raw pointer valid for zero sized reads if the vector didn't allocate.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up dangling.
    /// Modifying the vector may cause its buffer to be reallocated,
    /// which would also make any pointers to it invalid.
    ///
    /// This method guarantees that for the purpose of the aliasing model, this method
    /// does not materialize a reference to the underlying slice, and thus the returned pointer
    /// will remain valid when mixed with other calls to [`as_ptr`], [`as_mut_ptr`],
    /// and [`as_non_null`].
    /// Note that calling other methods that materialize references to the slice,
    /// or references to specific elements you are planning on accessing through this pointer,
    /// may still invalidate this pointer.
    /// See the second example below for how this guarantee can be used.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// // Allocate vector big enough for 4 elements.
    /// let length = 4;
    /// let mut opaque_vec: OpaqueVec = OpaqueVec::with_capacity::<i32>(length);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// let ptr = opaque_vec.as_mut_ptr::<i32, Global>();
    ///
    /// // Initialize elements via raw pointer writes, then set the length.
    /// unsafe {
    ///     for i in 0..length {
    ///         *ptr.add(i) = (i + 1) as i32;
    ///     }
    ///     opaque_vec.set_len::<i32, Global>(length);
    /// }
    ///
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1, 2, 3, 4]);
    /// ```
    ///
    /// Due to the aliasing guarantee, the following code is legal:
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = OpaqueVec::with_capacity::<i32>(4);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// opaque_vec.push::<i32, Global>(0);
    ///
    /// unsafe {
    ///     let ptr1 = opaque_vec.as_mut_ptr::<i32, Global>();
    ///     ptr1.write(1);
    ///     let ptr2 = opaque_vec.as_mut_ptr::<i32, Global>();
    ///     ptr2.write(2);
    ///     // Notably, the write to `ptr2` did **not** invalidate `ptr1`:
    ///     ptr1.write(3);
    /// }
    /// ```
    ///
    /// [`as_mut_ptr`]: OpaqueVec::as_mut_ptr
    /// [`as_ptr`]: OpaqueVec::as_ptr
    /// [`as_non_null`]: OpaqueVec::as_non_null
    #[inline]
    pub fn as_mut_ptr<T, A>(&mut self) -> *mut T
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.as_mut_ptr()
    }

    /// Returns a [`NonNull`] pointer to the vector's buffer, or a dangling
    /// [`NonNull`] pointer valid for zero sized reads if the vector didn't allocate.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up dangling.
    /// Modifying the vector may cause its buffer to be reallocated,
    /// which would also make any pointers to it invalid.
    ///
    /// This method guarantees that for the purpose of the aliasing model, this method
    /// does not materialize a reference to the underlying slice, and thus the returned pointer
    /// will remain valid when mixed with other calls to [`as_ptr`], [`as_mut_ptr`],
    /// and [`as_non_null`].
    /// Note that calling other methods that materialize references to the slice,
    /// or references to specific elements you are planning on accessing through this pointer,
    /// may still invalidate this pointer.
    /// See the second example below for how this guarantee can be used.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// // Allocate vector big enough for 4 elements.
    /// let length = 4;
    /// let mut opaque_vec = OpaqueVec::with_capacity::<i32>(length);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// let ptr = opaque_vec.as_non_null::<i32, Global>();
    ///
    /// // Initialize elements via raw pointer writes, then set length.
    /// unsafe {
    ///     for i in 0..length {
    ///         ptr.add(i).write((i + 1) as i32);
    ///     }
    ///     opaque_vec.set_len::<i32, Global>(length);
    /// }
    ///
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), &[1, 2, 3, 4]);
    /// ```
    ///
    /// Due to the aliasing guarantee, the following code is legal:
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_vec = OpaqueVec::with_capacity::<i32>(4);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// unsafe {
    ///     let ptr1 = opaque_vec.as_non_null::<i32, Global>();
    ///     ptr1.write(1);
    ///     let ptr2 = opaque_vec.as_non_null::<i32, Global>();
    ///     ptr2.write(2);
    ///     // Notably, the write to `ptr2` did **not** invalidate `ptr1`:
    ///     ptr1.write(3);
    /// }
    /// ```
    ///
    /// [`as_mut_ptr`]: Vec::as_mut_ptr
    /// [`as_ptr`]: Vec::as_ptr
    /// [`as_non_null`]: Vec::as_non_null
    #[inline]
    pub fn as_non_null<T, A>(&mut self) -> NonNull<T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.as_non_null()
    }

    /// Returns an immutable slice of the elements of the [`OpaqueVec`].
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 3] = [9, 28, 37];
    /// let opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// let expected = array.as_slice();
    /// let result = opaque_vec.as_slice::<i32, Global>();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.len(), opaque_vec.len());
    /// ```
    pub fn as_slice<T, A>(&self) -> &[T]
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.as_slice()
    }

    /// Returns n mutable slice of the elements of the [`OpaqueVec`].
    ///
    /// # Example
    ///
    /// Getting a mutable slice of an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut array: [i32; 3] = [9, 28, 37];
    /// let mut opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// let expected = array.as_mut_slice();
    /// let result = opaque_vec.as_mut_slice::<i32, Global>();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.len(), opaque_vec.len());
    /// ```
    ///
    /// Getting and mutating a mutable slice of an [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut array: [i32; 3] = [9, 28, 37];
    /// let mut opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// {
    ///     let slice = opaque_vec.as_mut_slice::<i32, Global>();
    ///     for i in 0..slice.len() {
    ///         slice[i] = 2 * slice[i];
    ///     }
    /// }
    ///
    /// let expected_array = [18, 56, 74];
    /// let expected = expected_array.as_slice();
    /// let result = opaque_vec.as_slice::<i32, Global>();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.len(), opaque_vec.len());
    /// ```
    pub fn as_mut_slice<T, A>(&mut self) -> &mut [T]
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.as_mut_slice()
    }

    #[must_use]
    pub fn into_raw_parts<T>(self) -> (*mut T, usize, usize)
    where
        T: any::Any,
    {
        let proj_self = self.into_proj::<T, alloc::Global>();

        proj_self.into_raw_parts()
    }

    #[must_use]
    pub fn into_parts<T>(self) -> (NonNull<T>, usize, usize)
    where
        T: any::Any,
    {
        let proj_self = self.into_proj::<T, alloc::Global>();

        proj_self.into_parts()
    }

    #[must_use]
    pub fn into_raw_parts_with_alloc<T, A>(self) -> (*mut T, usize, usize, TypedProjAlloc<A>)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.into_proj::<T, A>();

        proj_self.into_raw_parts_with_alloc()
    }

    #[must_use]
    pub fn into_parts_with_alloc<T, A>(self) -> (NonNull<T>, usize, usize, TypedProjAlloc<A>)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.into_proj::<T, A>();

        proj_self.into_parts_with_alloc()
    }

    #[track_caller]
    pub fn into_boxed_slice<T, A>(self) -> Box<[T], TypedProjAlloc<A>>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.into_proj::<T, A>();

        proj_self.into_boxed_slice()
    }

    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    #[track_caller]
    pub fn split_off<T, A>(&mut self, at: usize) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        let proj_split_off = proj_self.split_off(at);

        Self::from_proj(proj_split_off)
    }

    #[track_caller]
    pub fn resize_with<F, T, A>(&mut self, new_len: usize, f: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut() -> T,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.resize_with(new_len, f)
    }

    #[inline]
    pub fn spare_capacity_mut<T, A>(&mut self) -> &mut [MaybeUninit<T>]
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.spare_capacity_mut()
    }
}

impl OpaqueVec {
    pub fn try_reserve<T, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.try_reserve(additional)
    }

    pub fn try_reserve_exact<T, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.try_reserve_exact(additional)
    }

    #[track_caller]
    pub fn reserve<T, A>(&mut self, additional: usize)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.reserve(additional);
    }

    #[track_caller]
    pub fn reserve_exact<T, A>(&mut self, additional: usize)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.reserve_exact(additional);
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit<T, A>(&mut self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.shrink_to_fit();
    }

    #[track_caller]
    pub fn shrink_to<T, A>(&mut self, min_capacity: usize)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.shrink_to(min_capacity);
    }

    pub fn clear<T, A>(&mut self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.clear();
    }
}

impl OpaqueVec {
    #[inline]
    pub fn splice<R, I, T, A>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.splice(range, replace_with)
    }

    pub fn extract_if<F, R, T, A>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extract_if(range, filter)
    }

    /*
    #[track_caller]
    fn extend_with<T, A>(&mut self, count: usize, value: T)
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extend_with(count, value);
    }

    #[track_caller]
    fn extend_from_iter<I, T, A>(&mut self, iterator: I)
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator + Send + Sync,
        I: Iterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extend_from_iter(iterator);
    }
    */

    #[track_caller]
    pub fn extend_from_slice<T, A>(&mut self, other: &[T])
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extend_from_slice(other);
    }

    #[track_caller]
    pub fn resize<T, A>(&mut self, new_len: usize, value: T)
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.resize(new_len, value);
    }

    #[inline]
    pub fn truncate<T, A>(&mut self, len: usize)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.truncate(len);
    }
}

impl OpaqueVec {
    pub fn retain<F, T, A>(&mut self, f: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.retain(f);
    }

    pub fn retain_mut<F, T, A>(&mut self, f: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&mut T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.retain_mut(f);
    }

    #[inline]
    pub fn dedup<T, A>(&mut self)
    where
        T: any::Any + PartialEq,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.dedup();
    }

    #[inline]
    pub fn dedup_by_key<F, K, T, A>(&mut self, mut key: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.dedup_by_key(&mut key);
    }

    pub fn dedup_by<F, T, A>(&mut self, same_bucket: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.dedup_by(same_bucket);
    }
}

impl OpaqueVec {
    /// Appends all elements from an iterator to the [`OpaqueVec`].
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    /// let extension: [i32; 4] = [7, 8, 9, 10];
    /// let combined: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let expected = OpaqueVec::from(combined);
    /// #
    /// # assert!(expected.has_element_type::<i32>());
    /// # assert!(expected.has_allocator_type::<Global>());
    /// #
    /// let mut result = OpaqueVec::from(array);
    /// #
    /// # assert!(result.has_element_type::<i32>());
    /// # assert!(result.has_allocator_type::<Global>());
    /// #
    /// result.extend::<_, i32, Global>(extension.iter().cloned());
    ///
    /// assert_eq!(result.len(), array.len() + extension.len());
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// ```
    #[inline]
    pub fn extend<I, T, A>(&mut self, iter: I)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
        I: IntoIterator<Item=T>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extend(iter);
    }

    /// Mutably reverses an [`OpaqueVec`] in place.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// Examples
    ///
    /// Reversing a sequence with no repeating values.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    /// let array_rev: [i32; 6] = [6, 5, 4, 3, 2, 1];
    /// let expected = OpaqueVec::from(array_rev);
    /// #
    /// # assert!(expected.has_element_type::<i32>());
    /// # assert!(expected.has_allocator_type::<Global>());
    /// #
    /// let mut result = OpaqueVec::from(array);
    /// #
    /// # assert!(result.has_element_type::<i32>());
    /// # assert!(result.has_allocator_type::<Global>());
    /// #
    /// result.reverse::<i32, Global>();
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// ```
    ///
    /// Reversing a palindromic sequence.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let palindrome: [i32; 7] = [1, 2, 3, 4, 3, 2, 1];
    /// let expected = OpaqueVec::from(palindrome);
    /// #
    /// # assert!(expected.has_element_type::<i32>());
    /// # assert!(expected.has_allocator_type::<Global>());
    /// #
    /// let mut result = OpaqueVec::from(palindrome);
    /// #
    /// # assert!(result.has_element_type::<i32>());
    /// # assert!(result.has_allocator_type::<Global>());
    /// #
    /// result.reverse::<i32, Global>();
    ///
    /// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
    /// ```
    #[inline]
    pub fn reverse<T, A>(&mut self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.reverse();
    }
}

impl OpaqueVec {
    /// Clones an [`OpaqueVec`].
    ///
    /// This method acts identically to an implementation of the [`Clone`] trait on a type-projected
    /// vector [`TypedProjVec`], or a generics [`Vec`].
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the elements of `self` and the [`TypeId`]
    /// of the memory allocator of `self` do not match the requested element type `T` and
    /// allocator type `A`, respectively.
    ///
    /// Examples
    ///
    /// Cloning an empty [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_vec = OpaqueVec::new::<i32>();
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert!(opaque_vec.is_empty());
    ///
    /// let cloned_opaque_vec = opaque_vec.clone::<i32, Global>();
    /// #
    /// # assert!(cloned_opaque_vec.has_element_type::<i32>());
    /// # assert!(cloned_opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert!(cloned_opaque_vec.is_empty());
    ///
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), cloned_opaque_vec.as_slice::<i32, Global>());
    /// ```
    ///
    /// Cloning a non-empty [`OpaqueVec`].
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use crate::opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    /// let opaque_vec = OpaqueVec::from(array);
    /// #
    /// # assert!(opaque_vec.has_element_type::<i32>());
    /// # assert!(opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert!(!opaque_vec.is_empty());
    ///
    /// let cloned_opaque_vec = opaque_vec.clone::<i32, Global>();
    /// #
    /// # assert!(cloned_opaque_vec.has_element_type::<i32>());
    /// # assert!(cloned_opaque_vec.has_allocator_type::<Global>());
    /// #
    /// assert!(!cloned_opaque_vec.is_empty());
    ///
    /// assert_eq!(opaque_vec.len(), cloned_opaque_vec.len());
    /// assert_eq!(opaque_vec.as_slice::<i32, Global>(), cloned_opaque_vec.as_slice::<i32, Global>());
    /// ```
    #[inline]
    pub fn clone<T, A>(&self) -> Self
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let proj_self = self.as_proj::<T, A>();
        let proj_cloned_self = Clone::clone(proj_self);
        let cloned_self = OpaqueVec::from_proj(proj_cloned_self);

        cloned_self
    }
}

impl fmt::Debug for OpaqueVec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpaqueVec")
            .finish()
    }
}

impl<T> From<&[T]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(slice: &[T]) -> Self {
        let proj_vec = TypedProjVec::from(slice);

        Self::from_proj(proj_vec)
    }
}

impl<T> From<&mut [T]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(slice: &mut [T]) -> Self {
        let proj_vec = TypedProjVec::from(slice);

        Self::from_proj(proj_vec)
    }
}

impl<const N: usize, T> From<&[T; N]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(array: &[T; N]) -> Self {
        let proj_vec = TypedProjVec::from(array);

        Self::from_proj(proj_vec)
    }
}

impl<const N: usize, T> From<&mut [T; N]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(array: &mut [T; N]) -> Self {
        let proj_vec = TypedProjVec::from(array);

        Self::from_proj(proj_vec)
    }
}

impl<T, A> From<Vec<T, A>> for OpaqueVec
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn from(vec: Vec<T, A>) -> Self {
        let proj_vec = TypedProjVec::from(vec);

        Self::from_proj(proj_vec)
    }
}

impl<T> From<&Vec<T>> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(vec: &Vec<T>) -> Self {
        let proj_vec = TypedProjVec::from(vec);

        Self::from_proj(proj_vec)
    }
}

impl<T> From<&mut Vec<T>> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(vec: &mut Vec<T>) -> Self {
        let proj_vec = TypedProjVec::from(vec);

        Self::from_proj(proj_vec)
    }
}

impl<T, A> From<Box<[T], A>> for OpaqueVec
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn from(slice: Box<[T], A>) -> Self {
        let proj_vec = TypedProjVec::from(slice);

        Self::from_proj(proj_vec)
    }
}

impl<const N: usize, T> From<[T; N]> for OpaqueVec
where
    T: any::Any,
{
    fn from(array: [T; N]) -> Self {
        let proj_vec = TypedProjVec::from(array);

        Self::from_proj(proj_vec)
    }
}

impl From<&str> for OpaqueVec {
    #[track_caller]
    fn from(st: &str) -> Self {
        From::from(st.as_bytes())
    }
}

impl<T> FromIterator<T> for OpaqueVec
where
    T: any::Any,
{
    #[inline]
    #[track_caller]
    fn from_iter<I>(iter: I) -> OpaqueVec
    where
        I: IntoIterator<Item = T>,
    {
        let proj_vec = TypedProjVec::from_iter(iter);

        Self::from_proj(proj_vec)
    }
}

mod dummy {
    use super::*;
    use core::marker;
    use core::ptr::NonNull;

    #[allow(dead_code)]
    pub(super) struct DummyAlloc {
        _do_not_construct: marker::PhantomData<()>,
    }

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, _layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            panic!("[`DummyAlloc::allocate`] should never actually be called. Its purpose is to test struct layouts.");
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: alloc::Layout) {
            panic!("[`DummyAlloc::deallocate`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }
}

mod layout_testing_types {
    use super::*;
    use core::marker;

    #[allow(dead_code)]
    pub(super) struct TangentSpace {
        tangent: [f32; 3],
        bitangent: [f32; 3],
        normal: [f32; 3],
        _do_not_construct: marker::PhantomData<()>,
    }

    #[allow(dead_code)]
    pub(super) struct SurfaceDifferential {
        dpdu: [f32; 3],
        dpdv: [f32; 3],
        _do_not_construct: marker::PhantomData<()>,
    }

    #[allow(dead_code)]
    pub(super) struct OctTreeNode {
        center: [f32; 3],
        extent: f32,
        children: [Option<Box<OctTreeNode>>; 8],
        occupancy: u8,
        _do_not_construct: marker::PhantomData<()>,
    }
}

#[cfg(test)]
mod vec_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_vec_match_sizes<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypedProjVec<T, A>>();
        let result = mem::size_of::<OpaqueVec>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_vec_match_alignments<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypedProjVec<T, A>>();
        let result = mem::align_of::<OpaqueVec>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_vec_match_offsets<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::offset_of!(TypedProjVec<T, A>, inner);
        let result = mem::offset_of!(OpaqueVec, inner);

        assert_eq!(result, expected, "Opaque and Typed Projected data types offsets mismatch");
    }

    macro_rules! layout_tests {
        ($module_name:ident, $element_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_vec_layout_match_sizes() {
                    run_test_opaque_vec_match_sizes::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_vec_layout_match_alignments() {
                    run_test_opaque_vec_match_alignments::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_vec_layout_match_offsets() {
                    run_test_opaque_vec_match_offsets::<$element_typ, $alloc_typ>();
                }
            }
        };
    }

    layout_tests!(unit_zst_global, (), alloc::Global);
    layout_tests!(u8_global, u8, alloc::Global);
    layout_tests!(u16_global, u16, alloc::Global);
    layout_tests!(u32_global, u32, alloc::Global);
    layout_tests!(u64_global, u64, alloc::Global);
    layout_tests!(tangent_space_global, layout_testing_types::TangentSpace, alloc::Global);
    layout_tests!(surface_differential_global, layout_testing_types::SurfaceDifferential, alloc::Global);
    layout_tests!(oct_tree_node_global, layout_testing_types::OctTreeNode, alloc::Global);

    layout_tests!(unit_zst_dummy_alloc, (), dummy::DummyAlloc);
    layout_tests!(u8_dummy_alloc,  u8, dummy::DummyAlloc);
    layout_tests!(u16_dummy_alloc, u16, dummy::DummyAlloc);
    layout_tests!(u32_dummy_alloc, u32, dummy::DummyAlloc);
    layout_tests!(u64_dummy_alloc, u64, dummy::DummyAlloc);
    layout_tests!(tangent_space_dummy_alloc, layout_testing_types::TangentSpace, dummy::DummyAlloc);
    layout_tests!(surface_differential_dummy_alloc, layout_testing_types::SurfaceDifferential, dummy::DummyAlloc);
    layout_tests!(oct_tree_node_dummy_alloc, layout_testing_types::OctTreeNode, dummy::DummyAlloc);
}

#[cfg(test)]
mod vec_assert_send_sync {
    use super::*;

    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjVec<i32, alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjVec<i32, dummy::DummyAlloc>>();
    }
}
