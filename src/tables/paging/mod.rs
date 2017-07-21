use super::{Entry, EntryHandle, EntryVariant};

/// Module containing definitions of entries.
mod ent;
pub use self::ent::*;

#[repr(packed)]
pub struct P1 {
    entries : [P1E; 512]
}

#[repr(packed)]
pub struct P2 {
    entries : [u64; 512] // May be P2EMap or P2ERef.
}

#[repr(packed)]
pub struct P3 {
    entries : [P3E; 512]
}

#[repr(packed)]
pub struct P4 {
    entries : [P4E; 512]
}

macro_rules! implp_default {
    ($name:ident) => (
        impl Default for $name {
            fn default() -> $name {
                $name {
                    entries: [Default::default(); 512]
                }
            }
        }
    );
}

implp_default!(P1);
implp_default!(P2);
implp_default!(P3);
implp_default!(P4);
