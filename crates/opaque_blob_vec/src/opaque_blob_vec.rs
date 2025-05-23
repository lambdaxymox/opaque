use crate::blob_vec_memory;
use crate::blob_vec_memory::BlobVecMemory;

use core::any;
use core::fmt;
use core::marker;
use std::alloc;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

use opaque_alloc::{OpaqueAlloc, TypedProjAlloc};
use opaque_error;


#[repr(C)]
struct BlobVecInner {
    element_layout: alloc::Layout,
    length: usize,
    buffer: BlobVecMemory,
    drop_fn: Option<unsafe fn(NonNull<u8>)>,
}

impl BlobVecInner {
    #[inline]
    #[must_use]
    #[track_caller]
    fn new_in<A>(alloc: TypedProjAlloc<A>, element_layout: alloc::Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let length = 0;
        let opaque_alloc = OpaqueAlloc::from_proj(alloc);
        let buffer = BlobVecMemory::new_in(opaque_alloc, element_layout);

        Self {
            element_layout,
            length,
            buffer,
            drop_fn,
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    fn with_capacity_in<A>(capacity: usize, alloc: TypedProjAlloc<A>, element_layout: alloc::Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let length = 0;
        let opaque_alloc = OpaqueAlloc::from_proj(alloc);
        let buffer = BlobVecMemory::with_capacity_in(capacity, opaque_alloc, element_layout);

        Self {
            element_layout,
            length,
            buffer,
            drop_fn,
        }
    }

    #[inline]
    fn try_with_capacity_in<A>(
        capacity: usize,
        alloc: TypedProjAlloc<A>,
        element_layout: alloc::Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Result<Self, opaque_error::TryReserveError>
    where
        A: any::Any + alloc::Allocator,
    {
        let length = 0;
        let opaque_alloc = OpaqueAlloc::from_proj(alloc);
        let buffer = BlobVecMemory::try_with_capacity_in(capacity, opaque_alloc, element_layout)?;
        let vec = Self {
            element_layout,
            length,
            buffer,
            drop_fn,
        };

        Ok(vec)
    }

    #[inline]
    unsafe fn from_raw_parts_in<A>(
        ptr: *mut u8,
        length: usize,
        capacity: usize,
        alloc: TypedProjAlloc<A>,
        element_layout: alloc::Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let capacity_bytes = unsafe {
            blob_vec_memory::new_capacity(capacity, element_layout)
        };
        let opaque_alloc = OpaqueAlloc::from_proj(alloc);
        let buffer = unsafe {
            BlobVecMemory::from_raw_parts_in(ptr, capacity_bytes, opaque_alloc)
        };

        Self {
            element_layout,
            length,
            buffer,
            drop_fn,
        }
    }

    #[inline]
    unsafe fn from_parts_in<A>(
        ptr: NonNull<u8>,
        length: usize,
        capacity: usize,
        alloc: TypedProjAlloc<A>,
        element_layout: alloc::Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let capacity_bytes = unsafe {
            blob_vec_memory::new_capacity(capacity, element_layout)
        };
        let opaque_alloc = OpaqueAlloc::from_proj(alloc);
        let buffer = unsafe {
            BlobVecMemory::from_nonnull_in(ptr, capacity_bytes, opaque_alloc)
        };

        Self {
            element_layout,
            length,
            buffer,
            drop_fn,
        }
    }
}

impl BlobVecInner {
    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.buffer.allocator_type_id()
    }

    #[inline]
    const fn allocator(&self) -> &OpaqueAlloc {
        self.buffer.allocator()
    }
}

impl BlobVecInner {
    #[inline]
    const fn element_layout(&self) -> alloc::Layout {
        self.element_layout
    }

    #[inline]
    const fn capacity(&self) -> usize {
        self.buffer.capacity(self.element_layout.size())
    }

    #[inline]
    const fn len(&self) -> usize {
        self.length
    }

    #[inline]
    const fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl BlobVecInner {
    #[inline]
    const fn as_ptr(&self) -> *const u8 {
        self.buffer.ptr() as *const u8
    }

    #[inline]
    const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buffer.ptr()
    }

    #[inline]
    const fn as_non_null(&mut self) -> NonNull<u8> {
        // SAFETY: A `Vec` always holds a non-null pointer.
        unsafe { NonNull::new_unchecked(self.as_mut_ptr()) }
    }

    fn as_byte_slice(&self) -> &[u8] {
        if self.is_empty() {
            return &[];
        }

        let slice = unsafe {
            let data_ptr = self.as_ptr();
            let len = self.element_layout().size() * self.len();

            std::slice::from_raw_parts(data_ptr, len)
        };

        slice
    }
}

impl BlobVecInner {
    #[inline]
    fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.length = new_len;
    }

    #[inline]
    fn grow_one(&mut self) {
        self.buffer.grow_one(self.element_layout);
    }

    fn get_unchecked(&self, index: usize) -> NonNull<u8> {
        let base_ptr = self.as_ptr();
        let element = unsafe {
            let ptr = base_ptr.add(index * self.element_layout.size());
            NonNull::new_unchecked(ptr as *mut u8)
        };

        element
    }

    fn get_mut_unchecked(&mut self, index: usize) -> NonNull<u8> {
        let base_ptr = self.as_mut_ptr();
        let element = unsafe {
            let ptr = base_ptr.add(index * self.element_layout.size());
            NonNull::new_unchecked(ptr)
        };

        element
    }

    fn push(&mut self, value: NonNull<u8>) {
        let length = self.len();

        if length == self.capacity() {
            self.buffer.grow_one(self.element_layout);
        }

        let element_size = self.element_layout().size();

        unsafe {
            let buffer_end = self.as_mut_ptr().add(element_size * length);
            core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), buffer_end, element_size);
        }

        self.length = length + 1;
    }

    #[must_use]
    fn swap_remove_forget_unchecked(&mut self, index: usize) -> NonNull<u8> {
        debug_assert!(index < self.len());

        let new_length = self.length - 1;
        let element_size = self.element_layout.size();
        if index != new_length {
            unsafe {
                core::ptr::swap_nonoverlapping::<u8>(
                    self.get_mut_unchecked(index).as_ptr(),
                    self.get_mut_unchecked(new_length).as_ptr(),
                    element_size,
                );
            }
        }

        self.length = new_length;

        let ptr = unsafe { self.as_mut_ptr().byte_add(element_size * new_length) };

        unsafe { NonNull::new_unchecked(ptr) }
    }

    #[must_use]
    fn shift_remove_forget_unchecked(&mut self, index: usize) -> NonNull<u8> {
        debug_assert!(index < self.len());

        let new_length = self.length - 1;
        let element_size = self.element_layout.size();
        for i in index..new_length {
            unsafe {
                core::ptr::swap_nonoverlapping::<u8>(
                    self.get_mut_unchecked(i).as_ptr(),
                    self.get_mut_unchecked(i + 1).as_ptr(),
                    element_size,
                )
            }
        }

        self.length = new_length;

        let ptr = unsafe { self.as_mut_ptr().byte_add(element_size * new_length) };

        unsafe { NonNull::new_unchecked(ptr) }
    }

    fn replace_insert(&mut self, index: usize, value: NonNull<u8>) {
        struct DropGuard<F: FnOnce()> {
            callback: ManuallyDrop<F>,
        }

        impl<F: FnOnce()> DropGuard<F> {
            fn new(callback: F) -> Self {
                Self {
                    callback: ManuallyDrop::new(callback),
                }
            }
        }

        impl<F: FnOnce()> Drop for DropGuard<F> {
            fn drop(&mut self) {
                let callback = unsafe { ManuallyDrop::take(&mut self.callback) };

                callback();
            }
        }

        debug_assert!(index <= self.len());

        let element_size = self.element_layout.size();
        let length = self.len();

        if length == self.capacity() {
            self.grow_one();
        }

        unsafe {
            if index < length {
                let replaced_ptr = NonNull::new_unchecked(self.as_mut_ptr().add(element_size * index));

                if let Some(drop_fn) = self.drop_fn {
                    let old_length = self.length;
                    self.length = 0;

                    let guard = DropGuard::new(|| drop_fn(value));

                    drop_fn(replaced_ptr);

                    // We successfully called drop on the value to be replaced, so we no longer
                    // need the `value` guard.
                    core::mem::forget(guard);

                    // Restore the old length.
                    self.length = old_length;
                }

                // SAFETY: The old element has been dropped, so no memory leak will occur with just copying the new
                // value into memory.
                core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), replaced_ptr.as_ptr(), element_size);
            } else {
                let replaced_ptr = NonNull::new_unchecked(self.as_mut_ptr().add(element_size * index));

                // SAFETY: We are pushing to the end of the vector, so no dropping is needed.
                core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), replaced_ptr.as_ptr(), element_size);

                // We pushed to the vec instead of replacing a value inside the vec.
                self.length += 1;
            }
        }
    }

    fn shift_insert(&mut self, index: usize, value: NonNull<u8>) {
        debug_assert!(index <= self.len());

        let length = self.len();
        if length == self.capacity() {
            self.grow_one();
        }

        unsafe {
            {
                let element_size = self.element_layout.size();
                let byte_index = element_size * index;
                let base_ptr = self.as_mut_ptr().add(byte_index);
                if index < length {
                    let byte_count = element_size * (length - index);

                    core::ptr::copy(base_ptr, base_ptr.add(element_size), byte_count);
                }

                core::ptr::copy(value.as_ptr(), base_ptr, element_size);
            }
            self.set_len(length + 1);
        }
    }

    #[inline]
    fn try_reserve(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.buffer.try_reserve(self.length, additional, self.element_layout)
    }

    #[inline]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.buffer.try_reserve_exact(self.length, additional, self.element_layout)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        self.buffer.reserve(self.length, additional, self.element_layout);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn reserve_exact(&mut self, additional: usize) {
        self.buffer.reserve_exact(self.length, additional, self.element_layout);
    }

    #[track_caller]
    #[inline]
    fn shrink_to_fit(&mut self) {
        if self.capacity() > self.length {
            self.buffer.shrink_to_fit(self.length, self.element_layout);
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn shrink_to(&mut self, min_capacity: usize) {
        if self.capacity() > min_capacity {
            self.buffer
                .shrink_to_fit(std::cmp::max(self.length, min_capacity), self.element_layout);
        }
    }

    fn clear(&mut self) {
        let len = self.length;
        self.length = 0;

        if let Some(drop_fn) = self.drop_fn {
            let size = self.element_layout.size();
            let ptr = self.as_non_null();
            for i in 0..len {
                let element = unsafe { ptr.byte_add(i * size) };
                unsafe {
                    drop_fn(element);
                }
            }
        }
    }

    fn truncate(&mut self, len: usize) {
        unsafe {
            if len > self.len() {
                return;
            }

            let remaining_len = self.len() - len;
            let element_size = self.element_layout.size();
            let len_bytes = element_size * len;
            let remaining_len_bytes = element_size * remaining_len;
            let slice = core::ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(len_bytes), remaining_len_bytes);
            self.set_len(len);

            core::ptr::drop_in_place(slice);
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn extend_with(&mut self, count: usize, value: NonNull<u8>) {
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
            fn increment_len(&mut self, increment: usize) {
                self.local_len += increment;
            }

            #[inline]
            fn current_len(&self) -> usize {
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

        unsafe {
            let element_size = self.element_layout.size();
            let len_bytes = element_size * self.len();
            let mut ptr = self.as_mut_ptr().add(len_bytes);

            // Use SetLenOnDrop to work around bug where compiler
            // might not realize the store through `ptr` through self.set_len()
            // don't alias.
            let mut local_len = SetLenOnDrop::new(&mut self.length);

            // Write all elements except the last one
            for _ in 1..count {
                core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), ptr, element_size);
                ptr = ptr.add(element_size);
                // Increment the length in every step in case clone() panics
                local_len.increment_len(1);
            }

            if count > 0 {
                // We can write the last element directly without cloning needlessly
                core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), ptr, element_size);
                local_len.increment_len(1);
            }

            // len set by scope guard
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    unsafe fn append(&mut self, other: NonNull<u8>, count: usize) {
        self.reserve(count);
        let length = self.len();

        unsafe {
            let element_size = self.element_layout.size();
            let ptr = self.as_mut_ptr().add(element_size * length);
            core::ptr::copy_nonoverlapping(other.as_ptr(), ptr, element_size * count);
        }

        self.length += count;
    }
}

impl Drop for BlobVecInner {
    fn drop(&mut self) {
        self.clear();

        unsafe {
            self.buffer.deallocate(self.element_layout);
        }
    }
}

#[repr(transparent)]
pub struct TypedProjBlobVec<A = alloc::Global>
where
    A: any::Any + alloc::Allocator,
{
    inner: BlobVecInner,
    _marker: marker::PhantomData<A>,
}

impl<A> TypedProjBlobVec<A>
where
    A: any::Any + alloc::Allocator,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in(alloc: TypedProjAlloc<A>, element_layout: alloc::Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self {
        let inner = BlobVecInner::new_in(alloc, element_layout, drop_fn);

        Self {
            inner,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in(capacity: usize, alloc: TypedProjAlloc<A>, element_layout: alloc::Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self {
        let inner = BlobVecInner::with_capacity_in(capacity, alloc, element_layout, drop_fn);

        Self {
            inner,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub fn try_with_capacity_in(
        capacity: usize,
        alloc: TypedProjAlloc<A>,
        element_layout: alloc::Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Result<Self, opaque_error::TryReserveError> {
        let inner = BlobVecInner::try_with_capacity_in(capacity, alloc, element_layout, drop_fn)?;

        let vec = Self {
            inner,
            _marker: marker::PhantomData,
        };

        Ok(vec)
    }

    #[inline]
    pub unsafe fn from_raw_parts_in(
        ptr: *mut u8,
        length: usize,
        capacity: usize,
        alloc: TypedProjAlloc<A>,
        element_layout: alloc::Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Self {
        let inner = unsafe {
            BlobVecInner::from_raw_parts_in(ptr, length, capacity, alloc, element_layout, drop_fn)
        };

        Self {
            inner,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub unsafe fn from_parts_in(
        ptr: NonNull<u8>,
        length: usize,
        capacity: usize,
        alloc: TypedProjAlloc<A>,
        element_layout: alloc::Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Self {
        let inner = unsafe {
            BlobVecInner::from_parts_in(ptr, length, capacity, alloc, element_layout, drop_fn)
        };

        Self {
            inner,
            _marker: marker::PhantomData,
        }
    }
}

impl<A> TypedProjBlobVec<A>
where
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        self.inner.allocator().as_proj::<A>()
    }
}

impl<A> TypedProjBlobVec<A>
where
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub const fn element_layout(&self) -> alloc::Layout {
        self.inner.element_layout()
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<A> TypedProjBlobVec<A>
where
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }

    #[inline]
    pub const fn as_non_null(&mut self) -> NonNull<u8> {
        self.inner.as_non_null()
    }

    pub fn as_byte_slice(&self) -> &[u8] {
        self.inner.as_byte_slice()
    }
}

impl<A> TypedProjBlobVec<A>
where
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len)
    }

    #[inline]
    pub fn grow_one(&mut self) {
        self.inner.grow_one()
    }

    pub fn get_unchecked(&self, index: usize) -> NonNull<u8> {
        self.inner.get_unchecked(index)
    }

    pub fn get_mut_unchecked(&mut self, index: usize) -> NonNull<u8> {
        self.inner.get_mut_unchecked(index)
    }

    pub fn push(&mut self, value: NonNull<u8>) {
        self.inner.push(value)
    }

    #[must_use]
    pub fn swap_remove_forget_unchecked(&mut self, index: usize) -> NonNull<u8> {
        self.inner.swap_remove_forget_unchecked(index)
    }

    #[must_use]
    pub fn shift_remove_forget_unchecked(&mut self, index: usize) -> NonNull<u8> {
        self.inner.shift_remove_forget_unchecked(index)
    }

    pub fn replace_insert(&mut self, index: usize, value: NonNull<u8>) {
        self.inner.replace_insert(index, value)
    }

    pub fn shift_insert(&mut self, index: usize, value: NonNull<u8>) {
        self.inner.shift_insert(index, value)
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.inner.try_reserve(additional)
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional)
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity)
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn extend_with(&mut self, count: usize, value: NonNull<u8>) {
        self.inner.extend_with(count, value)
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub unsafe fn append(&mut self, other: NonNull<u8>, count: usize) {
        unsafe {
            self.inner.append(other, count)
        }
    }
}

impl<A> fmt::Debug for TypedProjBlobVec<A>
where
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("TypedProjBlobVec")
            .field("element_layout", &self.element_layout())
            .field("length", &self.len())
            .field("data", &self.as_byte_slice())
            .field("drop_fn", &self.inner.drop_fn)
            .finish()
    }
}

#[repr(transparent)]
pub struct OpaqueBlobVec {
    inner: BlobVecInner,
}

impl OpaqueBlobVec {
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in<A>(alloc: TypedProjAlloc<A>, element_layout: alloc::Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_blob_vec = TypedProjBlobVec::new_in(alloc, element_layout, drop_fn);

        Self::from_proj(proj_blob_vec)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in<A>(capacity: usize, alloc: TypedProjAlloc<A>, element_layout: alloc::Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_blob_vec = TypedProjBlobVec::with_capacity_in(capacity, alloc, element_layout, drop_fn);

        Self::from_proj(proj_blob_vec)
    }

    #[inline]
    pub fn try_with_capacity_in<A>(
        capacity: usize,
        alloc: TypedProjAlloc<A>,
        element_layout: alloc::Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Result<Self, opaque_error::TryReserveError>
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_blob_vec = TypedProjBlobVec::try_with_capacity_in(capacity, alloc, element_layout, drop_fn)?;

        Ok(Self::from_proj(proj_blob_vec))
    }

    #[inline]
    pub unsafe fn from_raw_parts_in<A>(
        ptr: *mut u8,
        length: usize,
        capacity: usize,
        alloc: TypedProjAlloc<A>,
        element_layout: alloc::Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_blob_vec = unsafe {
            TypedProjBlobVec::from_raw_parts_in(ptr, length, capacity, alloc, element_layout, drop_fn)
        };

        Self::from_proj(proj_blob_vec)
    }

    #[inline]
    pub unsafe fn from_parts_in<A>(
        ptr: NonNull<u8>,
        length: usize,
        capacity: usize,
        alloc: TypedProjAlloc<A>,
        element_layout: alloc::Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_blob_vec = unsafe {
            TypedProjBlobVec::from_parts_in(ptr, length, capacity, alloc, element_layout, drop_fn)
        };

        Self::from_proj(proj_blob_vec)
    }
}

impl OpaqueBlobVec {
    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl OpaqueBlobVec {
    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator,
    {
        self.allocator_type_id() == any::TypeId::of::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<A>(&self)
    where
        A: any::Any + alloc::Allocator,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(st: &str, type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("{:?} type mismatch. Need `{:?}`, got `{:?}`", st, type_id_self, type_id_other);
        }

        if !self.has_allocator_type::<A>() {
            type_check_failed("Allocator", self.allocator_type_id(), any::TypeId::of::<A>());
        }
    }
}

impl OpaqueBlobVec {
    #[inline]
    pub const fn element_layout(&self) -> alloc::Layout {
        self.inner.element_layout()
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl OpaqueBlobVec {
    #[inline]
    pub fn as_proj<A>(&self) -> &TypedProjBlobVec<A>
    where
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<A>();

        unsafe { &*(self as *const OpaqueBlobVec as *const TypedProjBlobVec<A>) }
    }

    #[inline]
    pub fn as_proj_mut<A>(&mut self) -> &mut TypedProjBlobVec<A>
    where
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<A>();

        unsafe { &mut *(self as *mut OpaqueBlobVec as *mut TypedProjBlobVec<A>) }
    }

    #[inline]
    pub fn into_proj<A>(self) -> TypedProjBlobVec<A>
    where
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<A>();

        TypedProjBlobVec {
            inner: self.inner,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_proj<A>(proj_self: TypedProjBlobVec<A>) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        Self {
            inner: proj_self.inner,
        }
    }
}

impl OpaqueBlobVec {
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }

    #[inline]
    pub const fn as_non_null(&mut self) -> NonNull<u8> {
        self.inner.as_non_null()
    }

    pub fn as_byte_slice(&self) -> &[u8] {
        self.inner.as_byte_slice()
    }
}

impl OpaqueBlobVec {
    #[inline]
    pub fn allocator<A>(&self) -> &TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<A>();

        proj_self.allocator()
    }
}

impl OpaqueBlobVec {
    #[inline]
    pub fn set_len<A>(&mut self, new_len: usize)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.set_len(new_len);
    }

    #[inline]
    pub fn grow_one<A>(&mut self)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.grow_one();
    }

    pub fn get_unchecked<A>(&self, index: usize) -> NonNull<u8>
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<A>();

        proj_self.get_unchecked(index)
    }

    pub fn get_mut_unchecked<A>(&mut self, index: usize) -> NonNull<u8>
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.get_mut_unchecked(index)
    }

    pub fn push<A>(&mut self, value: NonNull<u8>)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.push(value);
    }

    #[must_use]
    pub fn swap_remove_forget_unchecked<A>(&mut self, index: usize) -> NonNull<u8>
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.swap_remove_forget_unchecked(index)
    }

    #[must_use]
    pub fn shift_remove_forget_unchecked<A>(&mut self, index: usize) -> NonNull<u8>
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.shift_remove_forget_unchecked(index)
    }

    pub fn replace_insert<A>(&mut self, index: usize, value: NonNull<u8>)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.replace_insert(index, value)
    }

    pub fn shift_insert<A>(&mut self, index: usize, value: NonNull<u8>)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.shift_insert(index, value)
    }

    #[inline]
    pub fn try_reserve<A>(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError>
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.try_reserve(additional)
    }

    #[inline]
    pub fn try_reserve_exact<A>(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError>
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.try_reserve_exact(additional)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve<A>(&mut self, additional: usize)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.reserve(additional);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve_exact<A>(&mut self, additional: usize)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.reserve_exact(additional);
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit<A>(&mut self)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.shrink_to_fit();
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shrink_to<A>(&mut self, min_capacity: usize)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.shrink_to(min_capacity);
    }

    pub fn clear<A>(&mut self)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.clear();
    }

    pub fn truncate<A>(&mut self, len: usize)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.truncate(len);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn extend_with<A>(&mut self, count: usize, value: NonNull<u8>)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        proj_self.extend_with(count, value);
    }
    
    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub unsafe fn append<A>(&mut self, other: NonNull<u8>, count: usize)
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<A>();

        unsafe {
            proj_self.append(other, count);
        }
    }
}

impl fmt::Debug for OpaqueBlobVec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("OpaqueBlobVec").finish()
    }
}

#[cfg(test)]
mod blob_vec_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_blob_vec_match_sizes<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::size_of::<TypedProjBlobVec<A>>();
        let result = mem::size_of::<OpaqueBlobVec>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_blob_vec_match_alignments<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::align_of::<TypedProjBlobVec<A>>();
        let result = mem::align_of::<OpaqueBlobVec>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_blob_vec_match_offsets<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::offset_of!(TypedProjBlobVec<A>, inner);
        let result = mem::offset_of!(OpaqueBlobVec, inner);

        assert_eq!(result, expected, "Opaque and Typed Projected data types offsets mismatch");
    }

    struct Pair(u8, u64);

    struct DummyAlloc {}

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            alloc::Global.allocate(layout)
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
            unsafe {
                alloc::Global.deallocate(ptr, layout)
            }
        }
    }

    macro_rules! layout_tests {
        ($module_name:ident, $element_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_blob_vec_layout_match_sizes() {
                    run_test_opaque_blob_vec_match_sizes::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_blob_vec_layout_match_alignments() {
                    run_test_opaque_blob_vec_match_alignments::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_blob_vec_layout_match_offsets() {
                    run_test_opaque_blob_vec_match_offsets::<$element_typ, $alloc_typ>();
                }
            }
        };
    }

    layout_tests!(u8_global, u8, alloc::Global);
    layout_tests!(pair_dummy_alloc, Pair, DummyAlloc);
    layout_tests!(unit_zst_dummy_alloc, (), DummyAlloc);
}
