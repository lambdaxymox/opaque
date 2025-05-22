mod bench_get_unchecked;
mod bench_push;
mod bench_replace_insert;
mod bench_shift_insert;
mod bench_shift_remove_forget_unchecked;
mod bench_swap_remove_forget_unchecked;

pub use self::bench_get_unchecked::bench_get_unchecked;
pub use self::bench_push::bench_push;
pub use self::bench_replace_insert::bench_replace_insert;
pub use self::bench_shift_insert::bench_shift_insert;
pub use self::bench_shift_remove_forget_unchecked::bench_shift_remove_forget_unchecked;
pub use self::bench_swap_remove_forget_unchecked::bench_swap_remove_forget_unchecked;
