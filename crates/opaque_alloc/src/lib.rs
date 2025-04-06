#![feature(allocator_api)]
use std::alloc;
use std::alloc::{Allocator, Layout};
use std::any::TypeId;
use std::ptr::NonNull;
use std::fmt;
use std::any;

pub trait BoxedAllocator: Allocator {
    fn clone_boxed(&self) -> Box<dyn BoxedAllocator>;
}

impl<A> BoxedAllocator for A
where
    A: Allocator + Clone + 'static,
{
    fn clone_boxed(&self) -> Box<dyn BoxedAllocator> {
        Box::new(self.clone())
    }
}

pub struct OpaqueAlloc {
    alloc: Box<dyn BoxedAllocator>,
    type_id: TypeId,
}

impl OpaqueAlloc {
    #[inline]
    pub fn new<A>(alloc: A) -> Self
    where
        A: Allocator + Clone + 'static,
    {
        let new_alloc = Box::new(alloc);
        let type_id: TypeId = TypeId::of::<A>();

        Self {
            alloc: new_alloc,
            type_id,
        }
    }

    #[inline]
    pub fn is_type<A>(&self) -> bool
    where
        A: Allocator + Clone + 'static,
    {
        self.type_id == TypeId::of::<A>()
    }

    pub fn downcast_any<A>(self) -> Option<Box<A>>
    where
        A: Allocator + Clone + 'static,
    {
        if self.is_type::<A>() {
            let boxed_alloc = unsafe {
                let unboxed_alloc = Box::into_raw(self.alloc);
                Box::from_raw(unboxed_alloc as *mut A)
            };

            Some(boxed_alloc)
        } else {
            None
        }
    }
}

unsafe impl alloc::Allocator for OpaqueAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        self.alloc.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            self.alloc.deallocate(ptr, layout);
        }
    }
}

impl Clone for OpaqueAlloc {
    fn clone(&self) -> Self {
        Self {
            alloc: self.alloc.clone_boxed(),
            type_id: self.type_id,
        }
    }
}

impl fmt::Debug for OpaqueAlloc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpaqueAlloc")
            .field("alloc", &format_args!("{:?}", any::type_name::<Box<dyn BoxedAllocator>>()))
            .field("type_id", &self.type_id)
            .finish()
    }
}
