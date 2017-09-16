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

impl<'a> EntryHandle<'a> for P1EHandle {

    type Variant = P1EVariant<'a>;

    fn variant(&self) -> P1EVariant<'a> {
        unsafe { P1EVariant::P1E(&mut *(self.addr as *const P1E as *mut _)) }
    }
}

impl<'a> EntryHandle<'a> for P2EHandle {

    type Variant = P2EVariant<'a>;

    fn variant(&self) -> P2EVariant<'a> {
        let data = unsafe { *(self.addr as *const u64) };

        let is_map = |data: u64| -> bool {
            let val: u64 = PageFlag::ps().into();
            val & data == 1
        };

        unsafe {
            if is_map(data) {
                P2EVariant::Map(&mut *(self.addr as *const P2EMap as *mut _))
            } else {
                P2EVariant::Ref(&mut *(self.addr as *const P2ERef as *mut _))
            }
        }
    }
}

impl P2EHandle {

    /// Store given P2ERef by this handle table position pointer.
    ///
    /// # Safety
    /// Changing paging tables may violate memory consistency.
    pub unsafe fn set_ref(&self, e: P2ERef) -> &mut P2ERef {
        let ptr = self.addr as *const P2ERef as *mut _;
        *ptr = e;
        &mut *ptr
    }

    /// Store given P2EMap by this handle table position pointer.
    ///
    /// # Safety
    /// Changing paging tables may violate memory consistency.
    pub unsafe fn set_map(&self, e: P2EMap) -> &mut P2EMap {
        let ptr = self.addr as *const P2EMap as *mut _;
        *ptr = e;
        &mut *ptr
    }
}

impl<'a> EntryHandle<'a> for P3EHandle {

    type Variant = P3EVariant<'a>;

    fn variant(&self) -> P3EVariant<'a> {
        unsafe { P3EVariant::P3E(&mut *(self.addr as *const P3E as *mut _)) }
    }
}

impl<'a> EntryHandle<'a> for P4EHandle {

    type Variant = P4EVariant<'a>;

    fn variant(&self) -> P4EVariant<'a> {
        unsafe { P4EVariant::P4E(&mut *(self.addr as *const P4E as *mut _)) }
    }
}

macro_rules! impl_table {
    ($x:ident, $e:ident) => (
        impl<'a> super::Table<'a> for $x {

            type Handle = $e;

            fn entry_handle(&self, index: u16) -> Option<Self::Handle> {
                if self.limit_broken_by(index) {
                    return None;
                }

                let index = index as u64;
                Some($e::from_raw_addr(self.addr() + index * 8))
            }

            fn limit(&self) -> u16 {
                (self.entry_count() * 8) as _ // always 4096
            }

            fn entry_count(&self) -> u16 {
                self.entries.len() as _ // always 512
            }

            fn addr(&self) -> u64 {
                &self.entries[0] as *const _ as _
            }

            fn limit_step() -> u16 { 8 }
        }
    );
}

impl_table!(P1, P1EHandle);
impl_table!(P2, P2EHandle);
impl_table!(P3, P3EHandle);
impl_table!(P4, P4EHandle);
