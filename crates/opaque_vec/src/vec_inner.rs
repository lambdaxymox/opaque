use crate::raw_vec::{OpaqueRawVec, TypedProjRawVec};
use crate::drain::Drain;
use crate::extract_if::ExtractIf;
use crate::splice::Splice;

use core::any;
use core::cmp;
use core::mem;
use core::ops;
use core::slice;
use core::ptr;
use core::ptr::NonNull;
use core::mem::{
    ManuallyDrop,
    MaybeUninit,
};
use alloc_crate::alloc;
use alloc_crate::boxed::Box;
use alloc_crate::vec::Vec;

use opaque_alloc::TypedProjAlloc;
use opaque_error::TryReserveError;

unsafe fn drop_fn<T>(value: NonNull<u8>) {
    let to_drop = value.as_ptr() as *mut T;

    unsafe {
        ptr::drop_in_place(to_drop)
    }
}

#[inline(always)]
const fn get_drop_fn<T>() -> Option<unsafe fn(NonNull<u8>)> {
    if mem::needs_drop::<T>() {
        Some(drop_fn::<T> as unsafe fn(NonNull<u8>))
    } else {
        None
    }
}

#[repr(C)]
pub(crate) struct TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    data: TypedProjRawVec<T, A>,
    length: usize,
    element_type_id: any::TypeId,
    allocator_type_id: any::TypeId,
    drop_fn: Option<unsafe fn(NonNull<u8>)>,
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) const fn element_type_id(&self) -> any::TypeId {
        self.element_type_id
    }

    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.allocator_type_id
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        let data = TypedProjRawVec::new_in(proj_alloc);
        let length = 0;
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();
        let drop_fn = get_drop_fn::<T>();

        Self { data, length, element_type_id, allocator_type_id, drop_fn, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let data = TypedProjRawVec::with_capacity_in(capacity, proj_alloc);
        let length = 0;
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();
        let drop_fn = get_drop_fn::<T>();

        Self { data, length, element_type_id, allocator_type_id, drop_fn, }
    }

    #[inline]
    pub(crate) fn try_with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Result<Self, TryReserveError> {
        let data = TypedProjRawVec::try_with_capacity_in(capacity, proj_alloc)?;
        let length = 0;
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();
        let drop_fn = get_drop_fn::<T>();

        Ok(Self { data, length, element_type_id, allocator_type_id, drop_fn, })
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts_proj_in(ptr: *mut T, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let data = unsafe {
            TypedProjRawVec::from_raw_parts_in(ptr, capacity, proj_alloc)
        };
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();
        let drop_fn = get_drop_fn::<T>();

        Self { data, length, element_type_id, allocator_type_id, drop_fn, }
    }

    #[inline]
    pub(crate) unsafe fn from_parts_proj_in(ptr: NonNull<T>, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let data = unsafe {
            TypedProjRawVec::from_non_null_in(ptr, capacity, proj_alloc)
        };
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();
        let drop_fn = get_drop_fn::<T>();

        Self { data, length, element_type_id, allocator_type_id, drop_fn, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new_in(alloc: A) -> Self {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::new_proj_in(proj_alloc)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::with_capacity_proj_in(capacity, proj_alloc)
    }

    #[inline]
    pub(crate) fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, TryReserveError> {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::try_with_capacity_proj_in(capacity, proj_alloc)
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts_in(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self {
        let proj_alloc = TypedProjAlloc::new(alloc);

        unsafe {
            Self::from_raw_parts_proj_in(ptr, length, capacity, proj_alloc)
        }
    }

    #[inline]
    pub(crate) unsafe fn from_parts_in(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self {
        let proj_alloc = TypedProjAlloc::new(alloc);

        unsafe {
            Self::from_parts_proj_in(ptr, length, capacity, proj_alloc)
        }
    }
}

impl<T> TypedProjVecInner<T, alloc::Global>
where
    T: any::Any,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new() -> Self {
        Self::new_in(alloc::Global)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, alloc::Global)
    }

    #[inline]
    pub(crate) fn try_with_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        Self::try_with_capacity_in(capacity, alloc::Global)
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        unsafe {
            Self::from_raw_parts_in(ptr, length, capacity, alloc::Global)
        }
    }

    #[inline]
    pub(crate) unsafe fn from_parts(ptr: NonNull<T>, length: usize, capacity: usize) -> Self {
        unsafe {
            Self::from_parts_in(ptr, length, capacity, alloc::Global)
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) const fn capacity(&self) -> usize {
        self.data.capacity()
    }

    #[inline]
    pub(crate) const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.length
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn allocator(&self) -> &TypedProjAlloc<A> {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.data.allocator()
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        debug_assert!(new_len <= self.capacity());

        self.length = new_len;
    }

    #[inline]
    pub(crate) fn iter(&self) -> slice::Iter<'_, T> {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_slice().iter()
    }

    #[inline]
    pub(crate) fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_mut_slice().iter_mut()
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn get_unchecked<I>(&self, index: I) -> &<I as slice::SliceIndex<[T]>>::Output
    where
        I: slice::SliceIndex<[T]>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe {
            self.as_slice().get_unchecked(index)
        }
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn get_mut_unchecked<I>(&mut self, index: I) -> &mut <I as slice::SliceIndex<[T]>>::Output
    where
        I: slice::SliceIndex<[T]>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe {
            self.as_mut_slice().get_unchecked_mut(index)
        }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn push(&mut self, value: T) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let length = self.len();

        if length == self.data.capacity() {
            self.data.grow_one();
        }

        unsafe {
            let end = self.as_mut_ptr().add(length);
            ptr::write(end, value);
            self.set_len(length + 1);
        }
    }

    #[inline]
    pub(crate) fn pop(&mut self) -> Option<T> {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if self.len() == 0 {
            None
        } else {
            let last_value = unsafe {
                self.set_len(self.len() - 1);
                core::hint::assert_unchecked(self.len() < self.capacity());

                ptr::read(self.as_ptr().add(self.len()))
            };

            Some(last_value)
        }
    }

    #[inline]
    pub(crate) fn replace_insert(&mut self, index: usize, value: T) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("replace_insert index out of bounds: Got index `{index}`. Need index `{index}` <= len, where len is `{length}`.");
        }

        let length = self.len();
        if index > length {
            index_out_of_bounds_failure(index, length);
        }

        if length == self.capacity() {
            self.data.grow_one();
        }

        unsafe {
            if index < length {
                let value_ptr = self.as_mut_ptr().add(index);

                let _old_value = ptr::read(value_ptr);

                ptr::write(value_ptr, value);
            } else {
                let value_ptr = self.as_mut_ptr().add(index);

                // SAFETY: We are pushing to the end of the vector, so no dropping is needed.
                ptr::write(value_ptr, value);

                // We pushed to the vec instead of replacing a value inside the vec.
                self.set_len(length + 1);
            }
        }
    }

    #[inline]
    pub(crate) fn shift_insert(&mut self, index: usize, value: T) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("shift_insert index out of bounds: Got index `{index}`. Need index `{index}` <= len, where len is `{length}`.");
        }

        let length = self.len();
        if index > length {
            index_out_of_bounds_failure(index, length);
        }

        if length == self.data.capacity() {
            self.data.grow_one();
        }

        unsafe {
            {
                let slot_ptr = self.as_mut_ptr().add(index);
                if index < length {
                    ptr::copy(slot_ptr, slot_ptr.add(1), length - index);
                }

                ptr::write(slot_ptr, value);
            }

            self.set_len(length + 1);
        }
    }

    #[inline]
    pub(crate) fn swap_remove(&mut self, index: usize) -> T {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("swap_remove index out of bounds: Got `{index}`, length is `{length}`.");
        }

        let length = self.len();
        if index >= length {
            index_out_of_bounds_failure(index, length);
        }

        let value = unsafe {
            let _value = ptr::read(self.as_ptr().add(index));
            let base_ptr = self.as_mut_ptr();
            ptr::copy(base_ptr.add(length - 1), base_ptr.add(index), 1);
            self.set_len(length - 1);

            _value
        };

        value
    }

    #[inline]
    pub(crate) fn shift_remove(&mut self, index: usize) -> T {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("shift_remove index out of bounds: Got `{index}`, length is `{length}`.");
        }

        let length = self.len();
        if index >= length {
            index_out_of_bounds_failure(index, length);
        }

        let value = unsafe {
            let _value = {
                let ptr = self.as_mut_ptr().add(index);
                let __value = ptr::read(ptr);

                ptr::copy(ptr.add(1), ptr, length - index - 1);

                __value
            };

            self.set_len(length - 1);

            _value
        };

        value
    }

    #[inline]
    pub(crate) fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_slice().contains(value)
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.data.ptr() as *const T
    }

    #[inline]
    pub(crate) const fn as_mut_ptr(&mut self) -> *mut T {
        self.data.ptr()
    }

    #[inline]
    pub(crate) const fn as_non_null(&mut self) -> NonNull<T> {
        // SAFETY: A [`TypedProjVecInner`] always holds a non-null pointer.
        self.data.non_null()
    }

    #[inline]
    pub(crate) const fn as_slice(&self) -> &[T] {
        unsafe {
            let data_ptr = self.as_ptr();
            let len = self.len();

            slice::from_raw_parts(data_ptr, len)
        }
    }

    #[inline]
    pub(crate) const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            let data_ptr = self.as_mut_ptr();
            let len = self.len();

            slice::from_raw_parts_mut(data_ptr, len)
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_raw_parts(self) -> (*mut T, usize, usize) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let mut me = ManuallyDrop::new(self);
        let ptr = me.as_mut_ptr();
        let len = me.len();
        let capacity = me.capacity();

        (ptr, len, capacity)
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_parts(self) -> (NonNull<T>, usize, usize) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let mut me = ManuallyDrop::new(self);

        // SAFETY: An `OpaqueVec` always has a non-null pointer.
        let ptr = unsafe { NonNull::new_unchecked(me.as_mut_ptr()) };
        let len = me.len();
        let capacity = me.capacity();

        (ptr, len, capacity)
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_raw_parts_with_alloc(self) -> (*mut T, usize, usize, TypedProjAlloc<A>)
    where
        A: alloc::Allocator,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let mut me = ManuallyDrop::new(self);
        let ptr = me.as_mut_ptr();
        let len = me.len();
        let capacity = me.capacity();
        let alloc = unsafe { core::ptr::read(me.allocator()) };

        (ptr, len, capacity, alloc)
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_parts_with_alloc(self) -> (NonNull<T>, usize, usize, TypedProjAlloc<A>)
    where
        A: alloc::Allocator,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let mut me = ManuallyDrop::new(self);

        // SAFETY: An `OpaqueVec` always has a non-null pointer.
        let ptr = unsafe { NonNull::new_unchecked(me.as_mut_ptr()) };
        let len = me.len();
        let capacity = me.capacity();
        let alloc = unsafe { core::ptr::read(me.allocator()) };

        (ptr, len, capacity, alloc)
    }

    #[inline]
    pub(crate) fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe {
            let ptr = self.as_mut_ptr().add(self.len()) as *mut MaybeUninit<T>;
            let len = self.capacity() - self.len();

            slice::from_raw_parts_mut(ptr, len)
        }
    }

    pub(crate) fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
    where
        A: alloc::Allocator,
        R: ops::RangeBounds<usize>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        // Memory safety
        //
        // When the Drain is first created, it shortens the length of
        // the source vector to make sure no uninitialized or moved-from elements
        // are accessible at all if the Drain's destructor never gets to run.
        //
        // Drain will ptr::read out the values to remove.
        // When finished, remaining tail of the vec is copied back to cover
        // the hole, and the vector length is restored to the new length.
        //
        let len = self.len();
        let ops::Range { start, end } = core::slice::range(range, ..len);

        unsafe {
            // set self.vec length's to start, to be safe in case Drain is leaked
            self.set_len(start);
            let range_slice = slice::from_raw_parts(self.as_ptr().add(start), end - start);

            Drain::from_parts(end, len - end, range_slice.iter(), NonNull::from(self))
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    #[must_use]
    pub(crate) fn get<I>(&self, index: I) -> Option<&<I as slice::SliceIndex<[T]>>::Output>
    where
        I: slice::SliceIndex<[T]>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_slice().get(index)
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as slice::SliceIndex<[T]>>::Output>
    where
        I: slice::SliceIndex<[T]>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_mut_slice().get_mut(index)
    }

    #[inline]
    pub(crate) fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if self.len() == self.capacity() {
            return Err(value);
        }

        self.push(value);

        Ok(())
    }

    #[inline]
    #[track_caller]
    unsafe fn append_elements(&mut self, other: *const [T]) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let count = other.len();
        self.reserve(count);
        let length = self.len();
        unsafe {
            ptr::copy_nonoverlapping(other as *const T, self.as_mut_ptr().add(length), count)
        };

        self.length += count;
    }

    #[inline]
    #[track_caller]
    pub(crate) fn append(&mut self, other: &mut Self) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe {
            self.append_elements(other.as_slice() as _);
            other.set_len(0);
        }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn into_boxed_slice(mut self) -> Box<[T], TypedProjAlloc<A>>
    where
        A: alloc::Allocator,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe {
            self.shrink_to_fit();
            let mut me = ManuallyDrop::new(self);
            let len = me.len();
            let ptr = me.as_mut_ptr();
            let slice_ptr = std::ptr::slice_from_raw_parts_mut(ptr, len);
            let alloc = core::ptr::read(me.allocator());

            Box::from_raw_in(slice_ptr, alloc)
        }
    }

    #[inline]
    pub(crate) fn split_off(&mut self, at: usize) -> Self
    where
        A: alloc::Allocator + Clone,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds(at: usize, len: usize) -> ! {
            panic!("`at` split index (is {at}) should be <= len (is {len})");
        }

        if at > self.len() {
            index_out_of_bounds(at, self.len());
        }

        let other_len = self.len() - at;
        let mut other = {
            let cloned_alloc = self.allocator().clone();
            let box_alloc = cloned_alloc.into_boxed_alloc();
            let split_alloc = TypedProjAlloc::from_boxed_alloc(box_alloc);

            TypedProjVecInner::with_capacity_proj_in(other_len, split_alloc)
        };

        // Unsafely `set_len` and copy items to `other`.
        unsafe {
            self.set_len(at);
            other.set_len(other_len);

            ptr::copy_nonoverlapping(self.as_ptr().add(at), other.as_mut_ptr(), other.len());
        }

        other
    }

    #[inline]
    pub(crate) fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        A: alloc::Allocator,
        F: FnMut() -> T,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let len = self.len();
        if new_len > len {
            self.extend::<_>(core::iter::repeat_with(f).take(new_len - len));
        } else {
            self.truncate(new_len);
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.data.try_reserve(self.len(), additional)
    }

    #[inline]
    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.data.try_reserve_exact(self.len(), additional)
    }

    #[inline]
    #[track_caller]
    pub(crate) fn reserve(&mut self, additional: usize) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.data.reserve(self.len(), additional);
    }

    #[inline]
    #[track_caller]
    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.data.reserve_exact(self.len(), additional);
    }

    #[inline]
    #[track_caller]
    pub(crate) fn shrink_to_fit(&mut self) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.data.shrink_to_fit(self.capacity());
    }

    #[inline]
    #[track_caller]
    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if self.capacity() > min_capacity {
            self.data.shrink_to_fit(cmp::max(self.len(), min_capacity));
        }
    }
}
impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn clear(&mut self) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let elements: *mut [T] = self.as_mut_slice();

        unsafe {
            self.set_len(0);
            ptr::drop_in_place(elements);
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn extend_with(&mut self, count: usize, value: T)
    where
        T: Clone,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        struct SetLenOnDrop<'a> {
            len: &'a mut usize,
            local_len: usize,
        }

        impl<'a> SetLenOnDrop<'a> {
            #[inline]
            fn new(len: &'a mut usize) -> Self {
                SetLenOnDrop { local_len: *len, len }
            }

            #[inline]
            fn increment(&mut self, increment: usize) {
                self.local_len += increment;
            }

            #[inline]
            fn current(&self) -> usize {
                self.local_len
            }
        }

        impl Drop for SetLenOnDrop<'_> {
            #[inline]
            fn drop(&mut self) {
                *self.len = self.local_len;
            }
        }

        self.reserve(count);

        let length = self.len();

        unsafe {
            let mut ptr = self.as_mut_ptr().add(self.len());
            // Use SetLenOnDrop to work around bug where compiler
            // might not realize the store through `ptr` through self.set_len()
            // don't alias.
            let mut local_length = SetLenOnDrop::new(&mut self.length);

            // Write all elements except the last one
            for _ in 1..count {
                ptr::write(ptr, value.clone());
                ptr = ptr.add(1);
                // Increment the length in every step in case clone() panics
                local_length.increment(1);
            }

            if count > 0 {
                // We can write the last element directly without cloning needlessly
                ptr::write(ptr, value);
                local_length.increment(1);
            }

            debug_assert_eq!(local_length.current(), length + count);
            // len set by scope guard
        }
    }

    #[inline]
    pub(crate) fn extend_from_iter<I>(&mut self, iterator: I)
    where
        T: Clone,
        I: Iterator<Item = T>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        for item in iterator {
            self.push(item);
        }
    }

    #[inline]
    pub(crate) fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.extend_from_iter::<_>(other.iter().cloned())
    }

    #[inline]
    pub(crate) fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let len = self.len();

        if new_len > len {
            self.extend_with(new_len - len, value)
        } else {
            self.truncate(new_len);
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.retain_mut::<_>(|elem| f(elem));
    }

    pub(crate) fn retain_mut<F>(&mut self, mut f: F)
    where
        A: alloc::Allocator,
        F: FnMut(&mut T) -> bool,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let original_len = self.len();

        if original_len == 0 {
            // Empty case: explicit return allows better optimization, vs letting compiler infer it
            return;
        }

        // Avoid double drop if the drop guard is not executed,
        // since we may make some holes during the process.
        unsafe { self.set_len(0) };

        // Vec: [Kept, Kept, Hole, Hole, Hole, Hole, Unchecked, Unchecked]
        //      |<-              processed len   ->| ^- next to check
        //                  |<-  deleted cnt     ->|
        //      |<-              original_len                          ->|
        // Kept: Elements which predicate returns true on.
        // Hole: Moved or dropped element slot.
        // Unchecked: Unchecked valid elements.
        //
        // This drop guard will be invoked when predicate or `drop` of element panicked.
        // It shifts unchecked elements to cover holes and `set_len` to the correct length.
        // In cases when predicate and `drop` never panic, it will be optimized out.
        struct BackshiftOnDrop<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            v: &'a mut TypedProjVecInner<T, A>,
            processed_len: usize,
            deleted_cnt: usize,
            original_len: usize,
        }

        impl<T, A> Drop for BackshiftOnDrop<'_, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            fn drop(&mut self) {
                if self.deleted_cnt > 0 {
                    // SAFETY: Trailing unchecked items must be valid since we never touch them.
                    unsafe {
                        core::ptr::copy(
                            self.v.as_ptr().add(self.processed_len),
                            self.v.as_mut_ptr().add(self.processed_len - self.deleted_cnt),
                            self.original_len - self.processed_len,
                        );
                    }
                }
                // SAFETY: After filling holes, all items are in contiguous memory.
                unsafe {
                    self.v.set_len(self.original_len - self.deleted_cnt);
                }
            }
        }

        let mut g = BackshiftOnDrop {
            v: self,
            processed_len: 0,
            deleted_cnt: 0,
            original_len,
        };

        fn process_loop<F, T, A, const DELETED: bool>(original_len: usize, f: &mut F, g: &mut BackshiftOnDrop<'_, T, A>)
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync,
            F: FnMut(&mut T) -> bool,
        {
            while g.processed_len != original_len {
                // SAFETY: Unchecked element must be valid.
                let cur = unsafe { &mut *g.v.as_mut_ptr().add(g.processed_len) };
                if !f(cur) {
                    // Advance early to avoid double drop if `drop_in_place` panicked.
                    g.processed_len += 1;
                    g.deleted_cnt += 1;
                    // SAFETY: We never touch this element again after dropped.
                    unsafe { core::ptr::drop_in_place(cur) };
                    // We already advanced the counter.
                    if DELETED {
                        continue;
                    } else {
                        break;
                    }
                }
                if DELETED {
                    // SAFETY: `deleted_cnt` > 0, so the hole slot must not overlap with current element.
                    // We use copy for move, and never touch this element again.
                    unsafe {
                        let hole_slot = g.v.as_mut_ptr().add(g.processed_len - g.deleted_cnt);
                        core::ptr::copy_nonoverlapping(cur, hole_slot, 1);
                    }
                }
                g.processed_len += 1;
            }
        }

        // Stage 1: Nothing was deleted.
        process_loop::<F, T, A, false>(original_len, &mut f, &mut g);

        // Stage 2: Some elements were deleted.
        process_loop::<F, T, A, true>(original_len, &mut f, &mut g);

        // All item are processed. This can be optimized to `set_len` by LLVM.
        drop(g);
    }

    pub(crate) fn dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let len = self.len();
        if len <= 1 {
            return;
        }

        // Check if we ever want to remove anything.
        // This allows to use copy_non_overlapping in next cycle.
        // And avoids any memory writes if we don't need to remove anything.
        let mut first_duplicate_idx: usize = 1;
        let start = self.as_mut_ptr();
        while first_duplicate_idx != len {
            let found_duplicate = unsafe {
                // SAFETY: first_duplicate always in range [1..len)
                // Note that we start iteration from 1 so we never overflow.
                let prev = start.add(first_duplicate_idx.wrapping_sub(1));
                let current = start.add(first_duplicate_idx);
                // We explicitly say in docs that references are reversed.
                same_bucket(&mut *current, &mut *prev)
            };
            if found_duplicate {
                break;
            }
            first_duplicate_idx += 1;
        }
        // Don't need to remove anything.
        // We cannot get bigger than len.
        if first_duplicate_idx == len {
            return;
        }

        // INVARIANT: vec.len() > read > write > write-1 >= 0
        struct FillGapOnDrop<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            // Offset of the element we want to check if it is duplicate
            read: usize,

            // Offset of the place where we want to place the non-duplicate
            // when we find it.
            write: usize,

            // The Vec that would need correction if `same_bucket` panicked
            vec: &'a mut TypedProjVecInner<T, A>,
        }

        impl<'a, T, A> Drop for FillGapOnDrop<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            fn drop(&mut self) {
                // This code gets executed when `same_bucket` panics.
                //
                // SAFETY: invariant guarantees that `read - write`
                // and `len - read` never overflow and that the copy is always
                // in-bounds.
                unsafe {
                    let ptr = self.vec.as_mut_ptr();
                    let len = self.vec.len();

                    // How many items were left when `same_bucket` panicked.
                    // Basically vec[read..].len()
                    let items_left = len.wrapping_sub(self.read);

                    // Pointer to first item in vec[write..write+items_left] slice
                    let dropped_ptr = ptr.add(self.write);
                    // Pointer to first item in vec[read..] slice
                    let valid_ptr = ptr.add(self.read);

                    // Copy `vec[read..]` to `vec[write..write+items_left]`.
                    // The slices can overlap, so `copy_nonoverlapping` cannot be used
                    core::ptr::copy(valid_ptr, dropped_ptr, items_left);

                    // How many items have been already dropped
                    // Basically vec[read..write].len()
                    let dropped = self.read.wrapping_sub(self.write);

                    self.vec.set_len(len - dropped);
                }
            }
        }

        // Drop items while going through Vec, it should be more efficient than
        // doing slice partition_dedup + truncate

        // Construct gap first and then drop item to avoid memory corruption if `T::drop` panics.
        let mut gap: FillGapOnDrop<'_, T, A> = FillGapOnDrop {
            read: first_duplicate_idx + 1,
            write: first_duplicate_idx,
            vec: self,
        };

        unsafe {
            // SAFETY: we checked that first_duplicate_idx in bounds before.
            // If drop panics, `gap` would remove this item without drop.
            core::ptr::drop_in_place(start.add(first_duplicate_idx));
        }

        // SAFETY: Because of the invariant, read_ptr, prev_ptr and write_ptr
        // are always in-bounds and read_ptr never aliases prev_ptr
        unsafe {
            while gap.read < len {
                let read_ptr = start.add(gap.read);
                let prev_ptr = start.add(gap.write.wrapping_sub(1));

                // We explicitly say in docs that references are reversed.
                let found_duplicate = same_bucket(&mut *read_ptr, &mut *prev_ptr);
                if found_duplicate {
                    // Increase `gap.read` now since the drop may panic.
                    gap.read += 1;
                    // We have found duplicate, drop it in-place
                    core::ptr::drop_in_place(read_ptr);
                } else {
                    let write_ptr = start.add(gap.write);

                    // read_ptr cannot be equal to write_ptr because at this point
                    // we guaranteed to skip at least one element (before loop starts).
                    core::ptr::copy_nonoverlapping(read_ptr, write_ptr, 1);

                    /* We have filled that place, so go further */
                    gap.write += 1;
                    gap.read += 1;
                }
            }

            // Technically we could let `gap` clean up with its Drop, but
            // when `same_bucket` is guaranteed to not panic, this bloats a little
            // the codegen, so we just do it manually
            gap.vec.set_len(gap.write);
            mem::forget(gap);
        }
    }

    #[inline]
    pub(crate) fn dedup_by_key<F, K>(&mut self, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.dedup_by::<_>(|a, b| key(a) == key(b))
    }

    #[inline]
    pub(crate) fn dedup(&mut self)
    where
        T: PartialEq,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.dedup_by(|a, b| a == b)
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    where
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item=T>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Splice::new(self.drain(range), replace_with.into_iter())
    }

    #[inline]
    pub(crate) fn extract_if<F, R>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    where
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        ExtractIf::new(self, filter, range)
    }

    #[inline]
    pub(crate) fn truncate(&mut self, len: usize) {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());
        
        if len > self.len() {
            return;
        }

        let remaining_len = self.len() - len;
        unsafe {
            let slice = ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(len), remaining_len);
            self.set_len(len);
            ptr::drop_in_place(slice);
        }
    }
}

impl<T, A> Extend<T> for TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        for item in iter.into_iter() {
            self.push(item);
        }
    }
}

impl<'a, T, A> Extend<&'a T> for TypedProjVecInner<T, A>
where
    T: any::Any + Copy,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        for item in iter.into_iter() {
            self.push(*item);
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    #[inline]
    pub(crate) fn from_slice_proj_in(slice: &[T], proj_alloc: TypedProjAlloc<A>) -> TypedProjVecInner<T, A> {
        struct DropGuard<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync + Clone,
        {
            vec: &'a mut TypedProjVecInner<T, A>,
            num_init: usize,
        }

        impl<'a, T, A> Drop for DropGuard<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync + Clone,
        {
            #[inline]
            fn drop(&mut self) {
                // SAFETY:
                // items were marked initialized in the loop below
                unsafe {
                    self.vec.set_len(self.num_init);
                }
            }
        }

        let mut vec: TypedProjVecInner<T, A> = TypedProjVecInner::with_capacity_proj_in(slice.len(), proj_alloc);
        let mut guard = DropGuard {
            vec: &mut vec,
            num_init: 0,
        };
        let slots = guard.vec.spare_capacity_mut();
        // .take(slots.len()) is necessary for LLVM to remove bounds checks
        // and has better codegen than zip.
        for (i, b) in slice.iter().enumerate().take(slots.len()) {
            guard.num_init = i;
            slots[i].write(b.clone());
        }

        mem::forget(guard);

        // SAFETY:
        // the vec was allocated and initialized above to at least this length.
        unsafe {
            vec.set_len(slice.len());
        }

        vec
    }

    #[inline]
    pub(crate) fn from_slice_in(slice: &[T], alloc: A) -> TypedProjVecInner<T, A> {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::from_slice_proj_in(slice, proj_alloc)
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn from_boxed_slice(box_slice: Box<[T], TypedProjAlloc<A>>) -> TypedProjVecInner<T, A> {
        let length = box_slice.len();
        let capacity = box_slice.len();
        let (ptr, alloc) = {
            let (slice_ptr, _alloc) = Box::into_non_null_with_allocator(box_slice);
            let _ptr: NonNull<T> = unsafe { NonNull::new_unchecked(slice_ptr.as_ptr() as *mut T) };
            (_ptr, _alloc)
        };
        let vec = unsafe {
            TypedProjVecInner::from_parts_proj_in(ptr, length, capacity, alloc)
        };

        vec
    }
}

impl<T, A> Clone for TypedProjVecInner<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let cloned_alloc = self.allocator().clone();

        Self::from_slice_proj_in(self.as_slice(), cloned_alloc)
    }
}

impl<T, A> Drop for TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn drop(&mut self) {
        unsafe {
            // use drop for [T]
            // use a raw slice to refer to the elements of the vector as weakest necessary type;
            // could avoid questions of validity in certain cases
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.length))
        }

        // TypedProjRawVec handles deallocation
    }
}

impl<T> From<&[T]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any + Clone,
{
    fn from(slice: &[T]) -> Self {
        Self::from_slice_in(slice, alloc::Global::default())
    }
}

impl<T> From<&mut [T]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any + Clone,
{
    fn from(slice: &mut [T]) -> Self {
        Self::from_slice_in(slice, alloc::Global::default())
    }
}

impl<const N: usize, T> From<&[T; N]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any + Clone,
{
    fn from(array: &[T; N]) -> Self {
        Self::from(array.as_slice())
    }
}

impl<const N: usize, T> From<&mut [T; N]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any + Clone,
{
    fn from(array: &mut [T; N]) -> Self {
        Self::from(array.as_mut_slice())
    }
}

impl<T, A> From<Vec<T, A>> for TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn from(vec: Vec<T, A>) -> Self {
        let (ptr, length, capacity, alloc) = vec.into_parts_with_alloc();
        let inner = unsafe {
            TypedProjVecInner::from_parts_in(ptr, length, capacity, alloc)
        };

        inner
    }
}

impl<T, A> From<&Vec<T, A>> for TypedProjVecInner<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn from(vec: &Vec<T, A>) -> Self {
        Self::from(vec.clone())
    }
}

impl<T, A> From<&mut Vec<T, A>> for TypedProjVecInner<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn from(vec: &mut Vec<T, A>) -> Self {
        Self::from(vec.clone())
    }
}

impl<T, A> From<Box<[T], TypedProjAlloc<A>>> for TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn from(slice: Box<[T], TypedProjAlloc<A>>) -> Self {
        Self::from_boxed_slice(slice)
    }
}

impl<const N: usize, T> From<[T; N]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any,
{
    fn from(array: [T; N]) -> Self {
        Self::from_boxed_slice(Box::new_in(array, TypedProjAlloc::new(alloc::Global)))
    }
}

impl<T> FromIterator<T> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any,
{
    #[inline]
    #[track_caller]
    fn from_iter<I>(iter: I) -> TypedProjVecInner<T, alloc::Global>
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();

        let mut vec = TypedProjVecInner::with_capacity(lower);

        for item in iter {
            vec.push(item);
        }

        vec
    }
}

#[repr(C)]
pub(crate) struct OpaqueVecInner {
    data: OpaqueRawVec,
    length: usize,
    element_type_id: any::TypeId,
    allocator_type_id: any::TypeId,
    drop_fn: Option<unsafe fn(NonNull<u8>)>,
}

impl OpaqueVecInner {
    #[inline]
    pub(crate) const fn element_type_id(&self) -> any::TypeId {
        self.element_type_id
    }

    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.allocator_type_id
    }
}

impl OpaqueVecInner {
    #[inline(always)]
    pub(crate) fn as_proj_assuming_type<T, A>(&self) -> &TypedProjVecInner<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe { &*(self as *const OpaqueVecInner as *const TypedProjVecInner<T, A>) }
    }

    #[inline(always)]
    pub(crate) fn as_proj_mut_assuming_type<T, A>(&mut self) -> &mut TypedProjVecInner<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe { &mut *(self as *mut OpaqueVecInner as *mut TypedProjVecInner<T, A>) }
    }

    #[inline(always)]
    pub(crate) fn into_proj_assuming_type<T, A>(self) -> TypedProjVecInner<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.element_type_id(), any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe { mem::transmute(self) }
    }

    #[inline(always)]
    pub(crate) fn from_proj<T, A>(proj_self: TypedProjVecInner<T, A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe { mem::transmute(proj_self) }
    }
}

impl OpaqueVecInner {
    #[inline]
    pub(crate) const fn element_layout(&self) -> alloc::Layout {
        self.data.element_layout()
    }

    #[inline]
    pub(crate) const fn capacity(&self) -> usize {
        self.data.capacity()
    }

    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.length
    }

    #[inline]
    pub(crate) const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl OpaqueVecInner {
    fn clear(&mut self) {
        struct SetLenOnDrop<'a> {
            length: &'a mut usize,
            local_length: usize,
        }

        impl<'a> SetLenOnDrop<'a> {
            #[inline]
            fn new(length: &'a mut usize) -> Self {
                Self {
                    local_length: *length,
                    length,
                }
            }

            #[inline]
            fn decrement(&mut self) {
                self.local_length -= 1;
            }

            #[inline]
            fn current(&self) -> usize {
                self.local_length
            }
        }

        impl Drop for SetLenOnDrop<'_> {
            #[inline]
            fn drop(&mut self) {
                *self.length = self.local_length;
            }
        }

        if let Some(drop_fn) = self.drop_fn {
            let len = self.length;
            let ptr = self.data.as_non_null();
            let mut length_on_drop = SetLenOnDrop::new(&mut self.length);
            let size = self.data.element_layout().size();
            for i in 0..len {
                length_on_drop.decrement();
                let element = unsafe { ptr.byte_add(i * size) };
                unsafe {
                    drop_fn(element);
                }
            }

            debug_assert_eq!(length_on_drop.current(), 0);
        } else {
            self.length = 0;
        }
    }
}

impl Drop for  OpaqueVecInner {
    fn drop(&mut self) {
        self.clear();
        // `OpaqueRawVec` deallocates `self.data` from memory.
    }
}

mod dummy {
    use super::*;
    use core::ptr::NonNull;
    use std::marker;

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
mod vec_inner_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_vec_inner_match_sizes<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypedProjVecInner<T, A>>();
        let result = mem::size_of::<OpaqueVecInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_vec_inner_match_alignments<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypedProjVecInner<T, A>>();
        let result = mem::align_of::<OpaqueVecInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_vec_inner_match_offsets<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        assert_eq!(
            mem::offset_of!(TypedProjVecInner<T, A>, data),
            mem::offset_of!(OpaqueVecInner, data),
            "Opaque and Typed Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypedProjVecInner<T, A>, length),
            mem::offset_of!(OpaqueVecInner, length),
            "Opaque and Typed Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypedProjVecInner<T, A>, element_type_id),
            mem::offset_of!(OpaqueVecInner, element_type_id),
            "Opaque and Typed Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypedProjVecInner<T, A>, allocator_type_id),
            mem::offset_of!(OpaqueVecInner, allocator_type_id),
            "Opaque and Typed Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypedProjVecInner<T, A>, drop_fn),
            mem::offset_of!(OpaqueVecInner, drop_fn),
            "Opaque and Typed Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $element_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_vec_inner_layout_match_sizes() {
                    run_test_opaque_vec_inner_match_sizes::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_vec_inner_layout_match_alignments() {
                    run_test_opaque_vec_inner_match_alignments::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_vec_inner_layout_match_offsets() {
                    run_test_opaque_vec_inner_match_offsets::<$element_typ, $alloc_typ>();
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
mod vec_inner_assert_send_sync {
    use super::*;

    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjVecInner<i32, alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjVecInner<i32, dummy::DummyAlloc>>();
    }
}
