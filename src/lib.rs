#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod opaque_alloc {
    pub use opaque_alloc::*;
}

pub mod opaque_error {
    pub use opaque_error::*;
}

pub mod opaque_hash {
    pub use opaque_hash::*;
}

pub mod opaque_vec {
    pub use opaque_vec::*;
}

pub mod opaque_index_map {
    pub use opaque_index_map::*;
}
