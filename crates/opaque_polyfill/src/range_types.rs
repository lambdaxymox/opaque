use core::cmp::Ordering;
use core::fmt;
use core::hash::{
    Hash,
    Hasher,
};

macro_rules! define_valid_range_type {
    ($(
        $(#[$m:meta])*
        $vis:vis struct $name:ident($int:ident as $uint:ident in $low:literal..=$high:literal);
    )+) => {$(
        #[derive(Clone, Copy, Eq)]
        #[repr(transparent)]
        $(#[$m])*
        $vis struct $name($int);

        const _: () = {
            // With the `valid_range` attributes, it's always specified as unsigned
            assert!(<$uint>::MIN == 0);
            let ulow: $uint = $low;
            let uhigh: $uint = $high;
            assert!(ulow <= uhigh);

            assert!(size_of::<$int>() == size_of::<$uint>());
        };

        impl $name {
            #[inline]
            pub const fn new(val: $int) -> Option<Self> {
                if (val as $uint) >= ($low as $uint) && (val as $uint) <= ($high as $uint) {
                    // SAFETY: just checked the inclusive range
                    Some(unsafe { $name(val) })
                } else {
                    None
                }
            }

            /// Constructs an instance of this type from the underlying integer
            /// primitive without checking whether its zero.
            ///
            /// # Safety
            /// Immediate language UB if `val == 0`, as it violates the validity
            /// invariant of this type.
            #[inline]
            pub const unsafe fn new_unchecked(val: $int) -> Self {
                // SAFETY: Caller promised that `val` is non-zero.
                unsafe { $name(val) }
            }

            #[inline]
            pub const fn as_inner(self) -> $int {
                // SAFETY: This is a transparent wrapper, so unwrapping it is sound
                // (Not using `.0` due to MCP#807.)
                unsafe { core::mem::transmute(self) }
            }
        }

        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.as_inner() == other.as_inner()
            }
        }

        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                Ord::cmp(&self.as_inner(), &other.as_inner())
            }
        }

        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(Ord::cmp(self, other))
            }
        }

        impl Hash for $name {
            // Required method
            fn hash<H: Hasher>(&self, state: &mut H) {
                Hash::hash(&self.as_inner(), state);
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <$int as fmt::Debug>::fmt(&self.as_inner(), f)
            }
        }
    )+};
}

#[cfg(target_pointer_width = "16")]
define_valid_range_type! {
    pub struct UsizeNoHighBit(usize as usize in 0..=0x7fff);
}
#[cfg(target_pointer_width = "32")]
define_valid_range_type! {
    pub struct UsizeNoHighBit(usize as usize in 0..=0x7fff_ffff);
}
#[cfg(target_pointer_width = "64")]
define_valid_range_type! {
    pub struct UsizeNoHighBit(usize as usize in 0..=0x7fff_ffff_ffff_ffff);
}
