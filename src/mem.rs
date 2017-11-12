/// Store 'v' value 'c' times with STOSQ instruction.
#[inline(always)]
pub fn stosq(v: u64, c: u64) {
    unsafe { asm!(
    "rep stosq"
    :: "{rax}"(v), "{rcx}"(c)
    :: "volatile"
    ); }
}
