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
