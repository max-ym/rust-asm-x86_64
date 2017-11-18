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

    /// Re-initializes and remaps PIC.
    /// offset1 and offset2 are new interrupt vectors offsets.
    pub fn remap(&self, offset1: u8, offset2: u8) {
        use port::Port;

        // Interface ports.
        let pic1cmd = Port::from(0x20u16);
        let pic2cmd = Port::from(0xA0u16);
        let pic1dat = Port::from(0x21u16);
        let pic2dat = Port::from(0xA1u16);

        // Backup masks.
        let a1 = pic1dat.in_u8();
        let a2 = pic2dat.in_u8();

        // Initialization command (cascade mode).
        let init_cmd = 0x01 + 0x10;

        // Start Initialization sequence.
        pic1cmd.out_u8(init_cmd);
        pic2cmd.out_u8(init_cmd);

        // Master and slave interrupt vector offsets.
        pic1cmd.out_u8(offset1);
        pic2cmd.out_u8(offset2);

        pic1cmd.out_u8(4); // Tell Master PIC IRQ2 is slave.
        pic2cmd.out_u8(2); // Tell Slave its cascade identity.

        // Set 8086 mode.
        pic1cmd.out_u8(1);
        pic2cmd.out_u8(1);

        // Restore masks.
        pic1dat.out_u8(a1);
        pic2dat.out_u8(a1);
    }
}
