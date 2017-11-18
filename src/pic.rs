/// PIC interface.
pub struct Pic {
}

impl Pic {

    /// Create new interface to PIC.
    pub fn new() -> Self {
        Pic {}
    }

    /// Disable PIC.
    pub fn disable(&self) {
        use port::Port;

        let cmd = 0xFF; // Disable command
        let slv = 0xA1; // Slave PIC data port.
        let mst = 0x21; // Master PIC data port.

        let slv = Port::from(slv as u16);
        let mst = Port::from(mst as u16);

        // Send disable commands.
        slv.out_u8(cmd);
        mst.out_u8(cmd);
    }
}
