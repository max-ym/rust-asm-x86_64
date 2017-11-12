/// Store 'v' value 'c' times with STOSQ instruction by given 'addr' address.
#[inline(always)]
pub fn stosq(addr: u64, v: u64, c: u64) {
    unsafe { asm!(
    "rep stosq"
    :: "{rax}"(v), "{rcx}"(c), "{rdi}"(addr)
    :: "volatile"
    ); }
}
