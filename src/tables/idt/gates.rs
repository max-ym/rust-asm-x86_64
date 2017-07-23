use super::*;

/// The list of architecture defined interrupt vectors.
/// For more information see Intel System Programming Guide.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum InterruptVector {

    DivideError     = 0,
    DebugException  = 1,
    Nmi             = 2,
    Breakpoint      = 3,
    Overflow        = 4,
    BoundRange      = 5,
    InvalidOpcode   = 6,
    NoMath          = 7,
    DoubleFault     = 8,

    InvalidTss          = 10,
    SegmentNotPresent   = 11,
    StackSegmentFault   = 12,
    GeneralProtection   = 13,
    PageFault           = 14,

    MathFault               = 16,
    AlignmentCheck          = 17,
    MachineCheck            = 18,
    SimdException           = 19,
    VirtualizationException = 20,
}

/// The structure of the trap gate.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TrapGate {

    /// First 16 bits of offset.
    offset0     : u16,

    /// Segment selector.
    segsel      : u16,

    flags       : u16,

    /// Bits 16-31 of offset.
    offset1     : u16,

    /// Bits 32-63 of offset.
    offset2     : u32,

    _reserved   : u32,
}

/// The structure of the interrupt gate.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct InterruptGate {

    /// First 16 bits of offset.
    offset0     : u16,

    /// Segment selector.
    segsel      : u16,

    flags       : u16,

    /// Bits 16-31 of offset.
    offset1     : u16,

    /// Bits 32-63 of offset.
    offset2     : u32,

    _reserved   : u32,
}

impl Default for TrapGate {

    fn default() -> Self {
        TrapGate {
            offset0     : 0,
            segsel      : 0,
            flags       : (super::DescriptorType::TrapGate as u16) << 8,
            offset1     : 0,
            offset2     : 0,
            _reserved   : 0,
        }
    }
}

impl Default for InterruptGate {

    fn default() -> Self {
        InterruptGate {
            offset0     : 0,
            segsel      : 0,
            flags       : (super::DescriptorType::InterruptGate as u16) << 8,
            offset1     : 0,
            offset2     : 0,
            _reserved   : 0,
        }
    }
}

impl Entry for TrapGate {
}

impl Entry for InterruptGate {
}

/// IDT gate variants.
pub enum IdtGateVariant<'a> {
    Trap        (&'a TrapGate       ),
    Interrupt   (&'a InterruptGate  ),
}

impl<'a> EntryVariant for IdtGateVariant<'a> {
}

/// IDT Gate trait.
pub trait IdtGate: Entry {

    /// Address of the function that handles the interrupt.
    /// Intel System Programming Manual calls it 'offset'.
    fn offset(&self) -> u64 {
        let (a, b, c) = self.offset_fields();

        let a = a as u64;
        let b = b as u64;
        let c = c as u64;

        a + (b << 16) + (c << 32)
    }

    /// Fields with offset as they are stored in descriptor.
    fn offset_fields(&self) -> (u16, u16, u32);

    /// Set the address of the function that handles the interrupt.
    unsafe fn set_offset(&mut self, offset: u64) {
        let a = (offset >> 00) & 0xFFFF;
        let b = (offset >> 16) & 0xFFFF;
        let c = (offset >> 32) & 0xFFFFFFFF;

        self.set_offset_fields((a as u16, b as u16, c as u32));
    }

    /// Set fields with offset as they are stored in descriptor.
    unsafe fn set_offset_fields(&mut self, offset: (u16, u16, u32));

    /// Segment Selector.
    fn segsel(&self) -> u16;

    unsafe fn set_segsel(&mut self, ss: u16);

    /// Interrupt Stack Table.
    fn ist(&self) -> Ist {
        unimplemented!()
    }

    unsafe fn set_ist(&mut self, ist: Ist) {
        let v = ist as u16;
        let a = self.flags() & !0x03;

        self.set_flags(a | v);
    }

    /// Present flag value.
    fn present(&self) -> bool {
        self.flags() & (1 << 15) != 0
    }

    /// Change present flag value.
    unsafe fn set_present(&mut self, v: bool) {
        let a = self.flags() & !(1 << 15);
        self.set_flags(
            if v {
                a | (1 << 15)
            } else {
                a
            }
        );
    }

    /// Descriptor Privilege Level.
    fn dpl(&self) -> Dpl {
        use self::Dpl::*;
        match (self.flags() & (3 << 13)) >> 13 {
            0 => Dpl0,
            1 => Dpl1,
            2 => Dpl2,
            3 => Dpl3,
            _ => unreachable!()
        }
    }

    unsafe fn set_dpl(&mut self, dpl: Dpl) {
        let v = dpl as u16;
        let v = v << 13;

        let f = self.flags() & (!(3u16 << 13));
        self.set_flags(f | v);
    }

    /// Get descriptor type field as enum.
    fn type_enum(&self) -> DescriptorType {
        self.type_value().into()
    }

    /// Descriptor type field value.
    fn type_value(&self) -> u16 {
        (self.flags() & 0x0F00) >> 8
    }

    /// Get all flags.
    fn flags(&self) -> u16;

    /// Set all flags with given value. Does not check if value is correct nor
    /// change any of it's bit. Even if some bits must be zero (but are set).
    unsafe fn set_flags(&mut self, f: u16);
}
