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

    TscDeadlineMsr          = 0x6E0, // RW, 4

    // 1  - Not supported on Pentium 4 and Xeon.
    //
    // 2  - Introduced in Pentium 4 and Xeon. This APIC registers and its
    //      associated functions are implementation-dependent and may not be
    //      present in future IA-32 or Intel 64 processors.
    //
    // 3  - Introduced in Pentium Pro. This APIC register and its
    //      associated function are implementation-dependent and may not be
    //      present in future IA-32 or Intel 64 processors.
    //
    // 4  - Some CPUs may not support.
}

/// Value of DivideConfiguration register of APIC.
#[repr(packed)]
pub struct DivideConfiguration {
    reg     : u32,
}

/// Value of Local Vector Table Timer register of APIC.
#[repr(packed)]
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
pub struct TimerCurrentCount {
    reg     : u32,
}

/// Value of initial timer counter register of APIC.
#[repr(packed)]
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

/// TSC Deadline MSR.
#[repr(packed)]
pub struct TscDeadlineMsr {
    // All dwords must be accessed separately (requirement of APIC).

    a   : u32,
    b   : u32,
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
        unsafe { &*(ptr as *const DivideConfiguration) }
    }

    /// Get divide configuration value.
    pub fn divide_configuration_mut(&mut self) -> &mut DivideConfiguration {
        unsafe { &mut *(self.divide_configuration() as *const _ as *mut _) }
    }

    /// TSC Deadline MST.
    ///
    /// # Safety
    /// Some systems do not have this MSR. Caller must ensure this MSR
    /// exists.
    pub unsafe fn unsafe_tsc_deadline_msr(&self) -> &TscDeadlineMsr {
        let ptr = LocalApicReg::TscDeadlineMsr.ptr64(self);
        &*(ptr as *const _)
    }

    /// TSC Deadline MST.
    ///
    /// # Safety
    /// Some systems do not have this MSR. Caller must ensure this MSR
    /// exists.
    pub unsafe fn unsafe_tsc_deadline_msr_mut(&mut self)
            -> &mut TscDeadlineMsr {
        let ptr = LocalApicReg::TscDeadlineMsr.ptr64_mut(self);
        &mut *(ptr as *mut _)
    }
}

impl LvtTimer {

    /// Current LVT Timer mode.
    pub fn mode(&self) -> LvtTimerMode {
        use self::LvtTimerMode::*;
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

impl TscDeadlineMsr {

    /// Whether TSC Deadline MSR is supported by the system.
    pub fn exists() -> bool {
        unimplemented!()
    }

    /// Set timestamp.
    pub fn set(&mut self, timestamp: u64) {
        self.a = (timestamp >> 00) as u32;
        self.b = (timestamp >> 32) as u32;
    }

    /// Disarm timer.
    pub fn disarm(&mut self) {
        self.set(0); // Zero timestamp disarms the timer (Intel manual).
    }

    /// Current MSR value.
    pub fn value(&self) -> u64 {
        self.a as u64 + (self.b as u64) << 32
    }
}
