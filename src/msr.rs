#![allow(dead_code)]

/// Info read from MSR.
#[derive(Clone, Copy)]
pub struct Info {
    pub eax     : u32,
    pub edx     : u32,
}

impl Info {

    /// Get data in MSR by it's code. This function is unsafe as
    /// some MSRs may not be defined and so this call will cause
    /// General Protection fault. Ensure that MSR with given ID actually
    /// exists.
    pub unsafe fn read_by_id(id: u32) -> Info {
        let (a, d);
        asm!(
            "rdmsr"
            : "={eax}"(a), "={edx}"(d)
            : "{ecx}"(id)
        );

        Info { eax:a, edx:d }
    }

    pub unsafe fn write_by_id(&self, id: u32) {
        asm!(
            "wrmsr"
            : // No output
            : "{eax}"(self.eax), "{edx}"(self.edx), "{ecx}"(id)
        );
    }

    /// See 'read_by_id'. Note, that this function generally must not be used.
    /// It is more appropriate to use relevant 'read' function in the structure
    /// that represents the desired MSR.
    pub unsafe fn read(msr: Msr) -> Info {
        Self::read_by_id(msr as u32)
    }

    pub unsafe fn write(&self, msr: Msr) {
        Self::write_by_id(self, msr as u32)
    }
}

/// Model Specific Register ID list.
#[repr(u32)]
pub enum Msr {
    ApicBase        = 0x01B,
    TscDeadline     = 0x6E0,
    Xss             = 0xDA0,
}

macro_rules! derive_info {
    ($x:ident) => (
        #[derive(Clone, Copy)]
        pub struct $x {
            eax     : u32,
            edx     : u32,
        }

        impl Into<Info> for $x {

            fn into(self) -> Info {
                Info { eax: self.eax, edx: self.edx }
            }
        }

        impl AsRef<Info> for $x {

            fn as_ref(&self) -> &Info {
                unsafe { ::core::mem::transmute_copy(self) }
            }
        }

        impl $x {

            /// Read this given MSR. Note that if it is not defined
            /// in the processor, General Protection fault will be
            /// rised. You need to ensure that processor supports this MSR.
            pub unsafe fn read() -> Self {
                let info = Info::read(Msr::$x);
                // Convert the Info structure to correspond to given MSR.
                ::core::mem::transmute(info)
            }

            pub unsafe fn write(&self)  {
                let info: &Info = self.as_ref();
                info.write(Msr::$x);
            }
        }
    );
}

derive_info!(ApicBase);
derive_info!(TscDeadline);
derive_info!(Xss);

impl ApicBase {

    /// Whether this processor (core) is a Bootstrap Processor.
    pub fn bsp(&self) -> bool {
        self.edx & 0b0000_0000_0000_0000_0000_0001_0000_0000 != 0
    }

    pub fn x2apic_enabled(&self) -> bool {
        self.edx & 0b0000_0000_0000_0000_0000_0100_0000_0000 != 0
    }

    pub fn apic_global_enabled(&self) -> bool {
        self.edx & 0b0000_0000_0000_0000_0000_1000_0000_0000 != 0
    }

    pub fn apic_global_enable(&mut self) {
        self.edx |= 0b0000_0000_0000_0000_0000_1000_0000_0000;
    }

    pub fn apic_global_disable(&mut self) {
        self.edx &= 0b1111_1111_1111_1111_1111_0111_1111_1111;
    }

    pub fn apic_base(&self) -> u64 {
        let rdx = self.edx as u64;
        let rax = self.eax as u64;
        ((rdx & 0xFFFFF000) >> 12) | (rax << 20)
    }

     pub fn set_apic_base(&mut self, base: u64) {
         let d = (base & 0xFFFFF000) as u32;
         let a = (base >> 20)        as u32;

         // Clean corresponding bits before assigning them new values.
         self.eax = 0;
         self.edx &= 0x00000FFF;

         self.eax = a;
         self.edx = d;
     }
}

impl TscDeadline {

    /// Whether TSC Deadline MSR is supported by the system.
    ///
    /// Is checked by calling CPUID instruction which may be slow.
    pub fn exists() -> bool {
        ::cpuid::Features::get().tsc_deadline_supported()
    }

    /// Set timestamp.
    pub fn set(&mut self, timestamp: u64) {
        self.eax = (timestamp >> 00) as u32;
        self.edx = (timestamp >> 32) as u32;
    }

    /// Disarm timer.
    pub fn disarm(&mut self) {
        self.set(0); // Zero timestamp disarms the timer (Intel manual).
    }

    /// Current MSR value.
    pub fn value(&self) -> u64 {
        self.eax as u64 + (self.edx as u64) << 32
    }
}
