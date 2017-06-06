/// Trait means that structure represents a register in a processor.
/// Changes are not immediately commited to the real register. This
/// trait defines functions that allow to read and store the data
/// to the real register in the CPU.
pub trait Reg {

    /// Read the register and create the struct to represent the value.
    unsafe fn read() -> Self;

    /// Save the changes to real register.
    unsafe fn save(&self);

    /// Re-read the real register and overwrite current data in the structure.
    unsafe fn reset(&mut self);
}

#[repr(packed)]
#[derive(Clone, Copy)]
/// Control Register 3.
pub struct Cr3 {
    data    : u64,
}

impl Reg for Cr3 {

    unsafe fn read() -> Self {
        let data: u64;
        asm!(
            "mov    rax, cr3"
        :   "={rax}" (data)
        ::: "intel"
        );

        Cr3 {
            data: data
        }
    }

    unsafe fn save(&self) {
        asm!(
            "mov    cr3, rax"
        ::  "{rax}" (self.data)
        ::  "intel"
        );
    }

    unsafe fn reset(&mut self) {
        *self = Self::read();
    }
}

impl Cr3 {

    /// Page-level write-through.
    pub fn pwt(&self) -> bool {
        self.data & (1 << 3) != 0
    }

    /// Page-level cache disable.
    pub fn pcd(&self) -> bool {
        self.data & (1 << 4) != 0
    }

    pub unsafe fn set_pwt(&mut self, val: bool) {
        let a = self.data & !(1 << 3);
        self.data = if val {
            a | (1 << 3)
        } else {
            a
        };
    }

    pub unsafe fn set_pcd(&mut self, val: bool) {
        let a = self.data & !(1 << 4);
        self.data = if val {
            a | (1 << 4)
        } else {
            a
        };
    }

    /// Address of PML4 (P4)
    pub fn addr(&self) -> usize {
        (self.data & !0x7FF) as usize
    }

    pub unsafe fn set_addr(&mut self, addr: usize) {
        let a = self.data & 0x7FF;
        self.data = a + addr as u64;
    }
}
