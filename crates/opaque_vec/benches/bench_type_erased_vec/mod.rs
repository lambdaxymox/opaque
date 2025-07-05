mod bench_as_slice_index;
mod bench_get;
mod bench_pop;
mod bench_push;
mod bench_replace_insert;
mod bench_shift_insert;
mod bench_shift_remove;
mod bench_swap_remove;

pub use self::bench_as_slice_index::bench_as_slice_index;
pub use self::bench_get::bench_get;
pub use self::bench_pop::bench_pop;
pub use self::bench_push::bench_push;
pub use self::bench_replace_insert::bench_replace_insert;
pub use self::bench_shift_insert::bench_shift_insert;
pub use self::bench_shift_remove::bench_shift_remove;
pub use self::bench_swap_remove::bench_swap_remove;
