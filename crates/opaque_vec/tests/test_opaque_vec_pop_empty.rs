use opaque_vec::OpaqueVec;

fn run_test_opaque_vec_pop_empty1<T>()
where
    T: 'static,
{
    let mut vec = OpaqueVec::new::<T>();

    assert!(vec.pop::<T>().is_none());
}

fn run_test_opaque_vec_pop_empty2<T>()
where
    T: 'static,
{
    let mut vec = OpaqueVec::new::<T>();

    for _ in 0..65536 {
        assert!(vec.pop::<T>().is_none());
    }
}

fn run_test_opaque_vec_pop_empty_is_empty1<T>()
where
    T: 'static,
{
    let mut vec = OpaqueVec::new::<T>();

    assert!(vec.is_empty());

    vec.pop::<T>();

    assert!(vec.is_empty());
}

fn run_test_opaque_vec_pop_empty_is_empty2<T>()
where
    T: 'static,
{
    let mut vec = OpaqueVec::new::<T>();

    assert!(vec.is_empty());

    for _ in 0..65536 {
        vec.pop::<T>();
    }

    assert!(vec.is_empty());
}

macro_rules! generate_tests {
    ($($typ:ident),*) => {
        $(
            mod $typ {
                use super::*;

                #[test]
                fn test_opaque_vec_pop_empty1() {
                    run_test_opaque_vec_pop_empty1::<$typ>();
                }

                #[test]
                fn test_opaque_vec_pop_empty2() {
                    run_test_opaque_vec_pop_empty1::<$typ>();
                }

                #[test]
                fn test_opaque_vec_pop_empty_is_empty1() {
                    run_test_opaque_vec_pop_empty_is_empty1::<$typ>();
                }

                #[test]
                fn test_opaque_vec_pop_is_empty_is_empty2() {
                    run_test_opaque_vec_pop_empty_is_empty2::<$typ>();
                }
            }
        )*
    };
}

generate_tests!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
