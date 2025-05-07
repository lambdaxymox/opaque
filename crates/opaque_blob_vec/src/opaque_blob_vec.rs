use crate::opaque_vec_memory;
use crate::opaque_vec_memory::OpaqueVecMemory;
use opaque_alloc::OpaqueAlloc;

use core::fmt;
use std::alloc::Layout;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

use opaque_error;

pub struct OpaqueBlobVec {
    element_layout: Layout,
    length: usize,
    data: OpaqueVecMemory<OpaqueAlloc>,
    drop_fn: Option<unsafe fn(NonNull<u8>)>,
}

impl OpaqueBlobVec {
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn new_in(alloc: OpaqueAlloc, element_layout: Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self {
        let length = 0;
        let data = OpaqueVecMemory::new_in(alloc, element_layout);

        Self {
            element_layout,
            length,
            data,
            drop_fn,
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in(capacity: usize, alloc: OpaqueAlloc, element_layout: Layout, drop_fn: Option<unsafe fn(NonNull<u8>)>) -> Self {
        let length = 0;
        let data = OpaqueVecMemory::with_capacity_in(capacity, alloc, element_layout);

        Self {
            element_layout,
            length,
            data,
            drop_fn,
        }
    }

    #[inline]
    pub fn try_with_capacity_in(
        capacity: usize,
        alloc: OpaqueAlloc,
        element_layout: Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Result<Self, opaque_error::TryReserveError> {
        let length = 0;
        let data = OpaqueVecMemory::try_with_capacity_in(capacity, alloc, element_layout)?;
        let vec = Self {
            element_layout,
            length,
            data,
            drop_fn,
        };

        Ok(vec)
    }

    #[inline]
    pub unsafe fn from_raw_parts_in(
        ptr: *mut u8,
        length: usize,
        capacity: usize,
        alloc: OpaqueAlloc,
        element_layout: Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Self {
        let capacity_bytes = opaque_vec_memory::new_capacity(capacity, element_layout);
        let data = OpaqueVecMemory::from_raw_parts_in(ptr, capacity_bytes, alloc);

        Self {
            element_layout,
            length,
            data,
            drop_fn,
        }
    }

    #[inline]
    pub unsafe fn from_parts_in(
        ptr: NonNull<u8>,
        length: usize,
        capacity: usize,
        alloc: OpaqueAlloc,
        element_layout: Layout,
        drop_fn: Option<unsafe fn(NonNull<u8>)>,
    ) -> Self {
        let capacity_bytes = opaque_vec_memory::new_capacity(capacity, element_layout);
        let data = OpaqueVecMemory::from_nonnull_in(ptr, capacity_bytes, alloc);

        Self {
            element_layout,
            length,
            data,
            drop_fn,
        }
    }

    #[inline]
    pub const fn allocator(&self) -> &OpaqueAlloc {
        self.data.allocator()
    }
}

impl OpaqueBlobVec {
    #[inline]
    pub const fn element_layout(&self) -> Layout {
        self.element_layout
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.data.capacity(self.element_layout.size())
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.length
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    #[inline]
    pub fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.length = new_len;
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.data.ptr() as *const u8
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.ptr()
    }

    #[inline]
    pub const fn as_non_null(&mut self) -> NonNull<u8> {
        // SAFETY: A `Vec` always holds a non-null pointer.
        unsafe { NonNull::new_unchecked(self.as_mut_ptr()) }
    }

    #[inline]
    pub fn grow_one(&mut self) {
        self.data.grow_one(self.element_layout);
    }

    pub fn get_unchecked(&self, index: usize) -> NonNull<u8> {
        let base_ptr = self.as_ptr();
        let element = unsafe {
            let ptr = base_ptr.add(index * self.element_layout.size());
            NonNull::new_unchecked(ptr as *mut u8)
        };

        element
    }

    pub fn get_mut_unchecked(&mut self, index: usize) -> NonNull<u8> {
        let base_ptr = self.as_mut_ptr();
        let element = unsafe {
            let ptr = base_ptr.add(index * self.element_layout.size());
            NonNull::new_unchecked(ptr)
        };

        element
    }

    pub fn push(&mut self, value: NonNull<u8>) {
        let length = self.len();

        if length == self.capacity() {
            self.data.grow_one(self.element_layout);
        }

        let element_size = self.element_layout().size();

        unsafe {
            let buffer_end = self.as_mut_ptr().add(element_size * length);
            core::ptr::copy_nonoverlapping::<u8>(value.as_ptr(), buffer_end, element_size);
        }

        self.length = length + 1;
    }

    pub fn as_byte_slice(&self) -> &[u8] {
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

    #[must_use]
    pub fn swap_remove_forget_unchecked(&mut self, index: usize) -> NonNull<u8> {
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
    pub fn shift_remove_forget_unchecked(&mut self, index: usize) -> NonNull<u8> {
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

    pub fn replace_insert(&mut self, index: usize, value: NonNull<u8>) {
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

    pub fn shift_insert(&mut self, index: usize, value: NonNull<u8>) {
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
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.data.try_reserve(self.length, additional, self.element_layout)
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.data.try_reserve_exact(self.length, additional, self.element_layout)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(self.length, additional, self.element_layout);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(self.length, additional, self.element_layout);
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        if self.capacity() > self.length {
            self.data.shrink_to_fit(self.length, self.element_layout);
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        if self.capacity() > min_capacity {
            self.data
                .shrink_to_fit(std::cmp::max(self.length, min_capacity), self.element_layout);
        }
    }

    pub fn clear(&mut self) {
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

    pub fn truncate(&mut self, len: usize) {
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
    pub fn extend_with(&mut self, count: usize, value: NonNull<u8>) {
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
    pub unsafe fn append(&mut self, other: NonNull<u8>, count: usize) {
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

impl Drop for OpaqueBlobVec {
    fn drop(&mut self) {
        self.clear();

        unsafe {
            self.data.deallocate(self.element_layout);
        }
    }
}

impl fmt::Debug for OpaqueBlobVec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("OpaqueBlobVec").finish()
    }
}
