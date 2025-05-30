use opaque_index_map::set::OpaqueIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::iter;
use std::hash;

fn run_test_opaque_index_set_empty_len<T, S, A>(build_hasher: S, alloc: A)
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let opaque_set = OpaqueIndexSet::with_hasher_in::<T, S, A>(build_hasher, alloc);
    let expected = 0;
    let result = opaque_set.len();

    assert_eq!(result, expected);
}

fn run_test_opaque_index_set_empty_is_empty<T, S, A>(build_hasher: S, alloc: A)
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let opaque_set = OpaqueIndexSet::with_hasher_in::<T, S, A>(build_hasher, alloc);

    assert!(opaque_set.is_empty());
}

fn run_test_opaque_index_set_empty_contains_no_values<T, S, A>(build_hasher: S, alloc: A)
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug + TryFrom<usize> + iter::Step,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let opaque_set = OpaqueIndexSet::with_hasher_in::<T, S, A>(build_hasher, alloc);
    let min_value = TryFrom::try_from(0).unwrap();
    let max_value = TryFrom::try_from(127).unwrap();
    for value in min_value..max_value {
        assert!(!opaque_set.contains::<T, T, S, A>(&value));
    }
}

fn run_test_opaque_index_set_empty_get<T, S, A>(build_hasher: S, alloc: A)
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug + TryFrom<usize> + iter::Step,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let opaque_set = OpaqueIndexSet::with_hasher_in::<T, S, A>(build_hasher, alloc);
    let min_value = TryFrom::try_from(0).unwrap();
    let max_value = TryFrom::try_from(127).unwrap();
    for value in min_value..max_value {
        let result = opaque_set.get::<T, T, S, A>(&value);

        assert!(result.is_none());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, value_type = $value_typ:ty) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_set_empty_len() {
                let build_hasher = hash::RandomState::default();
                let alloc = alloc::Global;
                run_test_opaque_index_set_empty_len::<$value_typ, hash::RandomState, alloc::Global>(build_hasher, alloc)
            }

            #[test]
            fn test_opaque_index_set_empty_is_empty() {
                let build_hasher = hash::RandomState::default();
                let alloc = alloc::Global;
                run_test_opaque_index_set_empty_is_empty::<$value_typ, hash::RandomState, alloc::Global>(build_hasher, alloc)
            }

            #[test]
            fn test_opaque_index_set_empty_contains_no_values() {
                let build_hasher = hash::RandomState::default();
                let alloc = alloc::Global;
                run_test_opaque_index_set_empty_contains_no_values::<$value_typ, hash::RandomState, alloc::Global>(build_hasher, alloc)
            }

            #[test]
            fn test_opaque_index_set_empty_get() {
                let build_hasher = hash::RandomState::default();
                let alloc = alloc::Global;
                run_test_opaque_index_set_empty_get::<$value_typ, hash::RandomState, alloc::Global>(build_hasher, alloc)
            }
        }
    };
}

generate_tests!(
    u64,
    value_type = u64
);
generate_tests!(
    usize,
    value_type = usize
);

