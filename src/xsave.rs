/// Check XSAVE support through CPUID instruction.
pub fn is_supported() -> bool {
    ::cpuid::Features::get().xsave_supported()
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
