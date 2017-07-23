use super::*;

/// Module containing all IDT gates variants.
mod gates;
pub use self::gates::*;

/// Interrupt Descriptor Table. Raw structure to represent actual table in
/// the memory. Use IdtCtrl to edit IDT.
#[repr(packed)]
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
