use super::msr::ApicBase;
use super::cpuid;

/// Local APIC handle.
pub struct LocalApic {
    apic_base_msr   : ApicBase,
}

/// List of all local APIC registers and their addresses.
#[repr(usize)]
#[derive(PartialEq, Clone, Copy)]
pub enum LocalApicReg {
    Id                      = 0x020, // RW (Nehalem RO)
    Version                 = 0x030, // RO

    TaskPriority            = 0x080, // RW
    ArbitrationPriority     = 0x090, // RO, 1
    ProcessorPriority       = 0x0A0, // RO
    Eoi                     = 0x0B0, // WO
    RemoteRead              = 0x0C0, // RO, 1
    LogicalDestination      = 0x0D0, // RW
    DestinationFormat       = 0x0E0, // RW
    SpuriousInterruptVector = 0x0F0, // RW
    Isr0                    = 0x100, // RO
    Isr1                    = 0x110, // RO
    Isr2                    = 0x120, // RO
    Isr3                    = 0x130, // RO
    Isr4                    = 0x140, // RO
    Isr5                    = 0x150, // RO
    Isr6                    = 0x160, // RO
    Isr7                    = 0x170, // RO
    Tmr0                    = 0x180, // RO
    Tmr1                    = 0x190, // RO
    Tmr2                    = 0x1A0, // RO
    Tmr3                    = 0x1B0, // RO
    Tmr4                    = 0x1C0, // RO
    Tmr5                    = 0x1D0, // RO
    Tmr6                    = 0x1E0, // RO
    Tmr7                    = 0x1F0, // RO
    Irr0                    = 0x200, // RO
    Irr1                    = 0x210, // RO
    Irr2                    = 0x220, // RO
    Irr3                    = 0x230, // RO
    Irr4                    = 0x240, // RO
    Irr5                    = 0x250, // RO
    Irr6                    = 0x260, // RO
    Irr7                    = 0x270, // RO
    ErrorStatus             = 0x280, // RO

    LvtCmci                 = 0x2F0, // RW
    InterruptCommand0       = 0x300, // RW
    InterruptCommand1       = 0x310, // RW
    LvtTimer                = 0x320, // RW
    LvtThermalSensor        = 0x330, // RW, 2
    LvtPerformanceCounters  = 0x340, // RW, 3
    LvtLint0                = 0x350, // RW
    LvtLint1                = 0x360, // RW
    LvtError                = 0x370, // RW
    InitialCount            = 0x380, // RW
    CurrentCount            = 0x390, // RO

    DivideConfiguration     = 0x3E0, // RW

    // 1  - Not supported on Pentium 4 and Xeon.
    //
    // 2  - Introduced in Pentium 4 and Xeon. This APIC registers and its
    //      associated functions are implementation-dependent and may not be
    //      present in future IA-32 or Intel 64 processors.
    //
    // 3  - Introduced in Pentium Pro. This APIC register and its
    //      associated function are implementation-dependent and may not be
    //      present in future IA-32 or Intel 64 processors.
}

/// Version register local APIC type and version number.
pub enum VersionNumber {
    Discrete    (u8),
    Integrated  (u8),
}

/// Delivery status of LVT interrupts.
pub enum DeliveryStatus {
    Idle,
    SendPending,
}

/// Version register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Version {
    reg     : u32,
}

/// Value of DivideConfiguration register of APIC.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct DivideConfiguration {
    reg     : u32,
}

/// End Of Interrupt register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Eoi {
    reg     : u32,
}

/// Spurious Interrupt register.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct SpuriousInterrupt {
    reg     : u32,
}

/// Value of Local Vector Table Timer register of APIC.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct LvtTimer {
    reg     : u32,
}

/// LVT Timer mode.
#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
pub enum LvtTimerMode {
    OneShot     = 0b00 << 17,
    Periodic    = 0b01 << 17,
    TscDeadline = 0b10 << 17,
}

/// Mask that can be used to clear out all bits except timer mode
/// in LVT Timer register.
const LVT_TIMER_MODE_MASK: u32 = 0b11 << 17;

/// Value of current timer count register of APIC.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TimerCurrentCount {
    reg     : u32,
}

/// Value of initial timer counter register of APIC.
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TimerInitialCount {
    reg     : u32,
}

/// Divide value that is set in DivideConfiguration register.
#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
pub enum DivideValue {
    Div1    = 0b1011,
    Div2    = 0b0000,
    Div4    = 0b0001,
    Div8    = 0b0010,
    Div16   = 0b0011,
    Div32   = 0b1000,
    Div64   = 0b1001,
    Div128  = 0b1010,
}

impl LocalApicReg {

    /// Pointer to given local APIC register.
    pub fn ptr32(&self, apic: &LocalApic) -> *const u32 {
        let addr = *self as usize + apic.base_addr();
        addr as _
    }

    /// Pointer to mutable local APIC register.
    pub fn ptr32_mut(&self, apic: &mut LocalApic) -> *mut u32 {
        self.ptr32(apic) as _
    }

    pub fn ptr64(&self, apic: &LocalApic) -> *const u64 {
        self.ptr32(apic) as _
    }

    pub fn ptr64_mut(&self, apic: &mut LocalApic) -> *mut u64 {
        self.ptr32(apic) as _
    }

    pub fn ptr128(&self, apic: &LocalApic) -> *const (u64, u64) {
        self.ptr32(apic) as _
    }

    pub fn ptr128_mut(&self, apic: &mut LocalApic) -> *mut (u64, u64) {
        self.ptr32(apic) as _
    }
}

impl VersionNumber {

    /// Create from given version number.
    pub fn from_number(num: u8) -> Self {
        if num <= 0xF {
            VersionNumber::Discrete(num)
        } else {
            VersionNumber::Integrated(num)
        }
    }
}

// Macro to create basic getter functions for local APIC registers.
macro_rules! ro {
    ($x:tt, $y:tt) => {
        pub fn $y(&self) -> u32 {
            Self::val(LocalApicReg::$x)
        }
    };
}

// Macro to create basic setter functions for local APIC registers.
macro_rules! wo {
    ($x:tt, $y:tt) => {
        pub fn $y(&self, val: u32) {
            Self::sval(LocalApicReg::$x, val)
        }
    };
}

macro_rules! lvt_entry_impl_base {
    () => {

        /// LVT vector.
        pub fn vector(&self) -> u8 {
            (self.reg & 0xF) as u8
        }

        /// Delivery status.
        pub fn delivery_status(&self) -> DeliveryStatus {
            use self::DeliveryStatus::*;

            if self.reg & (1 << 12) != 0 {
                SendPending
            } else {
                Idle
            }
        }

        /// Whether interrupt is masked or not.
        pub fn masked(&self) -> bool {
            self.reg & (1 << 16) != 0
        }
    };
}

impl LocalApic {

    /// Get Local APIC reference if APIC is actually available.
    /// To determine the presence of APIC, CPUID instruction is used.
    pub fn new() -> Option<LocalApic> {
        if Self::local_apic_is_present() {
            unsafe {
                Some(Self::unsafe_new())
            }
        } else {
            None
        }
    }

    /// Get Local APIC reference.
    ///
    /// # Safety
    /// Does not check whether Local APIC exists
    /// so caller must be sure this operation is valid.
    pub unsafe fn unsafe_new() -> LocalApic {
        LocalApic { apic_base_msr : ApicBase::read() }
    }

    /// Check if local APIC is present in given system. Function
    /// uses CPUID to check feature availability. Because of that
    /// calls to this function are slow.
    pub fn local_apic_is_present() -> bool {
        cpuid::Features::get().local_apic_is_present()
    }

    /// Base address of local APIC registers mapped to RAM.
    pub fn base_addr(&self) -> usize {
        self.apic_base_msr.apic_base() as _
    }

    /// Re-map local APIC registers to given new address.
    pub unsafe fn set_base_addr(&mut self, base: usize) {
        self.apic_base_msr.set_apic_base(base as _);
        self.apic_base_msr.write();
    }

    /// Version register.
    pub fn version(&self) -> &Version {
        let ptr = LocalApicReg::Version.ptr32(self);
        unsafe { &*(ptr as *const _) }
    }

    /// EOI register.
    pub fn eoi_mut(&mut self) -> &mut Eoi {
        let ptr = LocalApicReg::Eoi.ptr32_mut(self);
        unsafe { &mut *(ptr as *mut _) }
    }

    /// Spurious interrupt vector register.
    pub fn spurious_interrupt(&self) -> &SpuriousInterrupt {
        let ptr = LocalApicReg::SpuriousInterruptVector.ptr32(self);
        unsafe { &*(ptr as *const _) }
    }

    /// Spurious interrupt vector register.
    pub fn spurious_interrupt_mut(&mut self) -> &mut SpuriousInterrupt {
        let ptr = LocalApicReg::SpuriousInterruptVector.ptr32_mut(self);
        unsafe { &mut *(ptr as *mut _) }
    }

    /// LVT Timer register.
    pub fn lvt_timer(&self) -> &LvtTimer {
        let ptr = LocalApicReg::LvtTimer.ptr32(self);
        unsafe { &*(ptr as *const _) }
    }

    /// LVT Timer register.
    pub fn lvt_timer_mut(&mut self) -> &mut LvtTimer {
        let ptr = LocalApicReg::LvtTimer.ptr32_mut(self);
        unsafe { &mut *(ptr as *mut _) }
    }

    /// Get timer initial count register.
    pub fn initial_count(&self) -> &TimerInitialCount {
        let ptr = LocalApicReg::InitialCount.ptr32(self);
        unsafe { &*(ptr as *const _) }
    }

    /// Get timer initial count register.
    pub fn initial_count_mut(&mut self) -> &mut TimerInitialCount {
        let ptr = LocalApicReg::InitialCount.ptr32_mut(self);
        unsafe { &mut *(ptr as *mut _)}
    }

    /// Get timer current count register.
    pub fn current_count(&self) -> &TimerCurrentCount {
        let ptr = LocalApicReg::CurrentCount.ptr32(self);
        unsafe { &*(ptr as *const _) }
    }

    /// Get timer current count register.
    pub fn current_count_mut(&mut self) -> &mut TimerCurrentCount {
        let ptr = LocalApicReg::CurrentCount.ptr32_mut(self);
        unsafe { &mut *(ptr as *mut _)}
    }

    /// Get divide configuration value.
    pub fn divide_configuration(&self) -> &DivideConfiguration {
        let ptr = LocalApicReg::DivideConfiguration.ptr32(self);
        unsafe { &*(ptr as *const _) }
    }

    /// Get divide configuration value.
    pub fn divide_configuration_mut(&mut self) -> &mut DivideConfiguration {
        let ptr = LocalApicReg::DivideConfiguration.ptr32_mut(self);
        unsafe { &mut *(ptr as *mut _) }
    }
}

impl Version {

    /// Local APIC version number.
    pub fn version(&self) -> VersionNumber {
        VersionNumber::from_number((self.reg & 0xF) as _)
    }

    /// Max LVT entry count minus one.
    pub fn max_lvt_entry(&self) -> u8 {
        ((self.reg & 0x0F_00) >> 16) as u8
    }

    /// Check support of EOI broadcast suppression.
    pub fn eoi_broadcast_suppression(&self) -> bool {
        self.reg & 0x10_00 != 0
    }
}

impl Eoi {

    /// Send End-Of-Interrupt signal.
    pub fn signal(&mut self) {
        self.reg = 0;
    }
}

impl SpuriousInterrupt {

    /// Spurious interrupt vector.
    pub fn vector(&self) -> u8 {
        (self.reg & 0xFF) as _
    }

    /// Set spurious interrupt vector.
    pub fn set_vector(&mut self, vec: u8) {
        self.reg = self.reg & !0xFF | (vec as u32);
    }

    /// EOI broadcast suppression.
    pub fn eoi_broadcast_suppression(&self) -> bool {
        self.reg & 0b1_0000_0000_0000 != 0
    }

    /// Enable EOI broadcast suppression.
    ///
    /// # Safety
    /// Not supported on some CPUs.
    pub unsafe fn enable_eoi_broadcast_suppression(&mut self) {
        self.reg |= 0b1_0000_0000_0000;
    }

    pub fn disable_eoi_broadcast_suppression(&mut self) {
        self.reg &= !0b1_0000_0000_0000;
    }

    pub fn focus_processor_checking(&self) -> bool {
        // NOTE if flag is zero then checking is enabled!
        self.reg & 0b0010_0000_0000 == 0
    }

    pub fn enable_focus_processor_checking(&mut self) {
        self.reg &= !0b0010_0000_0000;
    }

    /// Disable focus processor checking.
    ///
    /// # Safety
    /// Not supported on Pentium 4 and Xeon processors.
    pub unsafe fn disable_focus_processor_checking(&mut self) {
        self.reg |= 0b0010_0000_0000;
    }

    pub fn is_apic_software_enabled(&self) -> bool {
        self.reg & 0b0001_0000_0000 != 0
    }

    pub fn software_enable_apic(&mut self) {
        self.reg |= 0b0001_0000_0000;
    }

    pub fn software_disable_apic(&mut self) {
        self.reg &= !0b0001_0000_0000;
    }
}

impl LvtTimer {

    /// Current LVT Timer mode.
    pub fn mode(&self) -> LvtTimerMode {
        let val = self.reg & LVT_TIMER_MODE_MASK;
        unsafe { ::core::mem::transmute(val) }
    }

    /// Set LVT Timer mode.
    ///
    /// # Safety
    /// Caller must ensure that given mode is supported.
    /// TSC Deadline mode is not supported on some CPUs.
    pub unsafe fn set_mode(&mut self, mode: LvtTimerMode) {
        self.reg = self.reg & !LVT_TIMER_MODE_MASK | mode as u32;
    }
}

impl TimerCurrentCount {

    /// Get current timer value.
    pub fn value(&self) -> u32 {
        self.reg
    }
}

impl TimerInitialCount {

    /// Get initial timer value.
    pub fn value(&self) -> u32 {
        self.reg
    }

    /// Set new initial timer value.
    pub fn set(&mut self, value: u32) {
        self.reg = value;
    }

    /// Stop the timer by setting zero value.
    pub fn stop_timer(&mut self) {
        // This stops the timer according to Intel manual.
        self.set(0);
    }
}

impl DivideConfiguration {

    /// Set register specified value.
    unsafe fn set_reg(&mut self, val: u32) {
        self.reg = val;
    }

    /// Set specified divide value.
    pub fn set(&mut self, div: DivideValue) {
        unsafe { self.set_reg(div as _); }
    }

    /// Set divide factor 1.
    pub fn set_1(&mut self) {
        self.set(DivideValue::Div1);
    }

    /// Set divide factor 2.
    pub fn set_2(&mut self) {
        self.set(DivideValue::Div2);
    }

    /// Set divide factor 4.
    pub fn set_4(&mut self) {
        self.set(DivideValue::Div4);
    }

    /// Set divide factor 8.
    pub fn set_8(&mut self) {
        self.set(DivideValue::Div8);
    }

    /// Set divide factor 16.
    pub fn set_16(&mut self) {
        self.set(DivideValue::Div16);
    }

    /// Set divide factor 32.
    pub fn set_32(&mut self) {
        self.set(DivideValue::Div32);
    }

    /// Set divide factor 64.
    pub fn set_64(&mut self) {
        self.set(DivideValue::Div64);
    }

    /// Set divide factor 128.
    pub fn set_128(&mut self) {
        self.set(DivideValue::Div128);
    }

    /// Get value of this register.
    pub fn get(&self) -> DivideValue {
        use self::DivideValue::*;
        match self.reg {
            0b0000 => Div2  ,
            0b0001 => Div4  ,
            0b0010 => Div8  ,
            0b0011 => Div16 ,
            0b1000 => Div32 ,
            0b1001 => Div64 ,
            0b1010 => Div128,
            0b1011 => Div1  ,
            _      => unreachable!()
        }
    }
}
