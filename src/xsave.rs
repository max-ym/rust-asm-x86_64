pub use msr::Xss as XssMsr;
pub use cr::Xcr0;

/// Check XSAVE support through CPUID instruction.
pub fn is_supported() -> bool {
    fast_is_supported(::cpuid::Features::get())
}

/// Check XSAVE support by using already read CPUID value.
pub fn fast_is_supported(f: ::cpuid::Features) -> bool {
    f.xsave_supported()
}

/// Enable XSAVE by modifying CR4.
pub fn enable() {
    unsafe {
        use ::cr::Reg;

        let mut cr4 = ::cr::Cr4::read();
        cr4.enable_osxsave();
        cr4.save();
    }
}

/// XSAVE mask that is used in Xcr0 and MSR XSS.
#[derive(Clone, Copy, Default)]
pub struct Mask {
    val     : u64,
}

macro_rules! impl_xcr_flag {
    ($cons:ident, $get:ident, $set:ident, $unset:ident, $docs:expr) => (
        #[doc=$docs]
        pub fn $get(&self) -> bool {
            self.val & Self::$cons != 0
        }

        #[doc=$docs]
        pub fn $set(&mut self) {
            self.val |= Self::$cons;
        }

        #[doc=$docs]
        pub fn $unset(&mut self) {
            self.val &= !Self::$cons;
        }
    );

    ($cons:ident, $get:ident, $set:ident, $unset:ident) => (
        impl_xcr_flag!($cons, $get, $set, $unset, "");
    );
}

impl Mask {

    const SSE           : u64 = 1 << 1;
    const AVX           : u64 = 1 << 2;
    const MPX           : u64 = 3 << 3; // 3 = 0b11. Two bits required.
    const AVX512        : u64 = 7 << 5; // 7 = 0b111.
    const PKRU          : u64 = 1 << 9;

    impl_xcr_flag!(SSE, sse, enable_sse, disable_sse,
            "SSE component save enable flag. This does not affect the
            availability of the instructions. Thus, SSE instructions
            can be executed even with the flag unset.");

    impl_xcr_flag!(AVX, avx, enable_avx, disable_avx,
            "AVX component enable flag. If disabled,
            AVX instruction will cause invalid opcode exception.");

    impl_xcr_flag!(MPX, mpx, enable_mpx, disable_mpx,
            "MPX component enable flag. If disabled,
            MPX instruction will cause invalid opcode exception.");

    impl_xcr_flag!(AVX512, avx512, enable_avx512, disable_avx512,
            "AVX512 component enable flag. If disabled,
            AVX512 instruction will cause invalid opcode exception.
            To enable component SSE and AVX must be enabled too.");

    impl_xcr_flag!(PKRU, pkru, enable_pkru, disable_pkru,
            "PKRU component save enable flag. This does not affect the
            availability of the instructions. Thus, PKRU instructions
            can be executed even with the flag unset.");
}

impl Into<u64> for Mask {

    fn into(self) -> u64 {
        self.val
    }
}

impl From<u64> for Mask {

    fn from(val: u64) -> Self {
        Mask { val }
    }
}

macro_rules! impl_xsave {
    ($name:ident, $ins:expr, $doc:tt) => {
        #[doc=$doc]
        pub unsafe fn $name(xarea: u64, mask: Mask) {
            let eax = (mask.val      ) as u32;
            let edx = (mask.val >> 32) as u32;
            asm!(
                concat!($ins, " [$0]")
                :
                : "r"(xarea), "{eax}"(eax), "{edx}"(edx)
                :
                : "intel"
            );
        }
    };

    ($name:ident, $ins:expr) => {
        impl_xsave!($name, $ins, "");
    };
}

/// Call XSAVE instruction and pass given xsave memory area and
/// set given mask.
///
/// Note that XSAVE instruction support must be enabled, memory area
/// needs to be 64-byte aligned.
impl_xsave!(xsave, "xsave",
    "Call XSAVE instruction and pass given xsave memory area and
    set given mask.

    Note that XSAVE instruction support must be enabled, memory area
    needs to be 64-byte aligned."
);

impl_xsave!(xrstor, "xrstor",
    "Restore saved state with XRSTOR instruction."
);

impl_xsave!(xsaveopt, "xsaveopt",
    "Save the state with XSAVEOPT instruction."
);

impl_xsave!(xsavec, "xsavec",
    "Save the state using XSAVEC instruction."
);

impl_xsave!(xsaves, "xsaves",
    "Save the state using XSAVES instruction."
);

impl_xsave!(xrstors, "xrstors",
    "Restore the saved state with XRSTORS instruction."
);
