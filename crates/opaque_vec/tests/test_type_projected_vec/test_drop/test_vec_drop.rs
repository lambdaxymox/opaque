use opaque_vec::TypeProjectedVec;

use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[derive(Clone)]
struct DropCounter {
    count: Rc<RefCell<u32>>,
}

impl DropCounter {
    #[inline]
    const fn new(count: Rc<RefCell<u32>>) -> Self {
        Self { count }
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        *self.count.borrow_mut() += 1;
    }
}

#[test]
fn test_typed_proj_vec_double_drop() {
    struct TwoTypeProjectedVec {
        x: TypeProjectedVec<DropCounter, alloc::Global>,
        y: TypeProjectedVec<DropCounter, alloc::Global>,
    }

    let (ref_count_x, ref_count_y) = (Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)));
    {
        let mut tv = TwoTypeProjectedVec {
            x: TypeProjectedVec::new(),
            y: TypeProjectedVec::new(),
        };

        tv.x.push(DropCounter::new(ref_count_x.clone()));
        tv.y.push(DropCounter::new(ref_count_y.clone()));

        drop(tv.x);
    }

    let count_x = *ref_count_x.borrow();
    let count_y = *ref_count_y.borrow();

    assert_eq!(count_x, 1);
    assert_eq!(count_y, 1);
}

#[test]
fn test_typed_proj_vec_drop_all_items1() {
    let count = 1024;
    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());
    {
        let mut vec = TypeProjectedVec::new();
        for i in 0..count {
            vec.push(counter.clone());
        }
    }

    let expected = count;
    let result = *ref_count.borrow();

    assert_eq!(result, expected);
}

#[test]
fn test_typed_proj_vec_push_should_not_drop_value() {
    let mut vec = TypeProjectedVec::new();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push(counter);

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_typed_proj_vec_replace_insert_should_not_drop_value() {
    let mut vec = TypeProjectedVec::new();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.replace_insert(0, counter);

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_typed_proj_vec_swap_remove_should_not_drop_return_value() {
    let mut vec = TypeProjectedVec::new();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push(counter);

    let _counter = vec.swap_remove(0);

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_typed_proj_vec_shift_remove_should_not_drop_return_value() {
    let mut vec = TypeProjectedVec::new();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push(counter);

    let _counter = vec.shift_remove(0);

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_typed_proj_vec_pop_should_not_drop_return_value() {
    let mut vec = TypeProjectedVec::new();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push(counter);

    let _counter = vec.pop();

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_typed_proj_vec_clear_should_drop() {
    let mut vec = TypeProjectedVec::new();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push(counter);

    vec.clear();

    let count = *ref_count.borrow();

    assert_eq!(count, 1);
}

#[test]
fn test_typed_proj_vec_drop_should_drop_elements() {
    let ref_count = Rc::new(RefCell::new(0));
    {
        let mut vec = TypeProjectedVec::new();

        let counter = DropCounter::new(ref_count.clone());

        vec.push(counter);
        // `vec` drops here.
    }

    let count = *ref_count.borrow();

    assert_eq!(count, 1);
}

#[test]
fn test_typed_proj_vec_drop_should_not_drop_swap_removed_elements() {
    let ref_count = Rc::new(RefCell::new(0));
    let _counter = {
        let mut vec = TypeProjectedVec::new();

        vec.push(DropCounter::new(ref_count.clone()));

        let counter = vec.swap_remove(0);

        counter
        // `vec` drops here.
    };

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_typed_proj_vec_drop_should_not_drop_shift_removed_elements() {
    let ref_count = Rc::new(RefCell::new(0));
    let _counter = {
        let mut vec = TypeProjectedVec::new();

        vec.push(DropCounter::new(ref_count.clone()));

        let counter = vec.shift_remove(0);

        counter
        // `vec` drops here.
    };

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_typed_proj_vec_drop_should_not_drop_popped_elements() {
    let ref_count = Rc::new(RefCell::new(0));
    let _counter = {
        let mut vec = TypeProjectedVec::new();

        vec.push(DropCounter::new(ref_count.clone()));

        let counter = vec.pop();

        counter
        // `vec` drops here.
    };

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}
