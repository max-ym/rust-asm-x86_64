/// Check XSAVE support through CPUID instruction.
pub fn is_supported() -> bool {
    ::cpuid::Features::get().xsave_supported()
}
