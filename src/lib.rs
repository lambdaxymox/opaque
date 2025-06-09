#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod alloc {
    pub use opaque_alloc::*;
}

pub mod error {
    pub use opaque_error::*;
}

pub mod hash {
    pub use opaque_hash::*;
}

pub mod vec {
    pub use opaque_vec::*;
}

pub mod index_map {
    pub use opaque_index_map::*;
}
