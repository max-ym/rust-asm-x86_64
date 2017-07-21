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

macro_rules! new_handle {
    ($name:ident, $p:ident) => {
        pub struct $name {
            /// Entry data address.
            addr: u64
        }

        impl $name {

            /// Create given handle by pointing out actual entry in memory.
            pub fn from_raw_addr(addr: u64) -> Self {
                $name {
                    addr: addr
                }
            }
        }
    }
}

new_handle!(P1EHandle, P1);
new_handle!(P2EHandle, P2);
new_handle!(P3EHandle, P3);
new_handle!(P4EHandle, P4);

pub enum P1EVariant<'a> {
    P1E(&'a P1E)
}

pub enum P2EVariant<'a> {
    Map(&'a P2EMap),
    Ref(&'a P2ERef),
}

pub enum P3EVariant<'a> {
    P3E(&'a P3E)
}

pub enum P4EVariant<'a> {
    P4E(&'a P4E)
}

impl<'a> EntryVariant for P1EVariant<'a> {
}

impl<'a> EntryVariant for P2EVariant<'a> {
}

impl<'a> EntryVariant for P3EVariant<'a> {
}

impl<'a> EntryVariant for P4EVariant<'a> {
}

impl<'a> EntryHandle<'a> for P1EHandle {

    type Variant = P1EVariant<'a>;

    fn variant(&self) -> P1EVariant<'a> {
        unsafe { P1EVariant::P1E(&*(self.addr as *const P1E)) }
    }
}

impl<'a> EntryHandle<'a> for P2EHandle {

    type Variant = P2EVariant<'a>;

    fn variant(&self) -> P2EVariant<'a> {
        let data = unsafe { *(self.addr as *const u64) };

        let is_map = |addr: u64| -> bool {
            let val: u64 = PageFlag::ps().into();
            val & addr == 1
        };

        unsafe {
            if is_map(self.addr) {
                P2EVariant::Map(&*(self.addr as *const P2EMap))
            } else {
                P2EVariant::Ref(&*(self.addr as *const P2ERef))
            }
        }
    }
}

impl<'a> EntryHandle<'a> for P3EHandle {

    type Variant = P3EVariant<'a>;

    fn variant(&self) -> P3EVariant<'a> {
        unsafe { P3EVariant::P3E(&*(self.addr as *const P3E)) }
    }
}

impl<'a> EntryHandle<'a> for P4EHandle {

    type Variant = P4EVariant<'a>;

    fn variant(&self) -> P4EVariant<'a> {
        unsafe { P4EVariant::P4E(&*(self.addr as *const P4E)) }
    }
}
