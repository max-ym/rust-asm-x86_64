use super::*;

/// Module containing all IDT gates variants.
mod gates;
pub use self::gates::*;

/// Interrupt Descriptor Table. Raw structure to represent actual table in
/// the memory. Use IdtCtrl to edit IDT.
#[repr(packed)]
#[allow(dead_code)]
pub struct Idt {

    /// The array of all 256 gates of the IDT.
    gates:  [(u64, u64); 256],
}

/// The IDT controller. Saves information about table (like table limit) and
/// provides a set of methods to work with IDT.
pub struct IdtCtrl {
    limit   : u16,
    idt     : *mut Idt,
}

/// Interrupt Stack Table.
#[repr(u16)]
pub enum Ist {
    Ist0 = 0,
    Ist1 = 1,
    Ist2 = 2,
    Ist3 = 3,
}

/// Handle for entry of IDT.
pub struct IdtGateHandle {
    addr    : u64
}

impl<'a> EntryHandle<'a> for IdtGateHandle {

    type Variant = IdtGateVariant<'a>;

    fn variant(&self) -> Self::Variant {
        let ptr = self.addr as *const u64;
        let data = unsafe { *ptr };
        let type_field = DescriptorType::type_field_from_raw64(data);

        use self::IdtGateVariant::*;
        if type_field == DescriptorType::InterruptGate as _ {
            let ptr = ptr as *const InterruptGate;
            Interrupt(unsafe { &*ptr })
        } else if type_field == DescriptorType::TrapGate as _ {
            let ptr = ptr as *const TrapGate;
            Trap(unsafe { &*ptr })
        } else {
            Unknown
        }
    }
}

impl IdtGateHandle {

    /// Create IdtGateHandle by providing entry address.
    pub fn new_by_addr(entry_addr: u64) -> Self {
        IdtGateHandle {
            addr: entry_addr
        }
    }
}

impl<'a> Table<'a> for IdtCtrl {

    type Handle = IdtGateHandle;

    fn entry_handle(&self, index: u16) -> Option<Self::Handle> {
        if self.limit_broken_by(index) {
            return None;
        }

        let offset = index * Self::limit_step();
        let addr = self.addr() + offset as u64;

        Some(IdtGateHandle::new_by_addr(addr))
    }

    fn limit(&self) -> u16 {
        self.limit
    }

    /// Get address of the table.
    fn addr(&self) -> u64 {
        self.idt as u64
    }

    fn limit_step() -> u16 {
        8
    }
}

impl<'a> DtLimit<'a> for IdtCtrl {

    unsafe fn set_limit(&mut self, limit: u16) {
        self.limit = limit;
    }
}
