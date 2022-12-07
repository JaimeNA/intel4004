use super::intel4001::Intel4001;

// Intel 4004(CPU)

pub struct Intel4004 {
    pc:    u8,
    carry: bool, 
    acc:   u8,
    index: [u8; 16],                                                 // Dynamic RAM cell array of 16 x 4 bits.
    stack: [u16; 3],                                                 // 3 x 12 bits array
    ram_addr: u8,
    signal: bool,
    pub rom: Intel4001,                                            
}
  
impl Intel4004 {
    pub fn new() -> Intel4004 {
        Intel4004 {
            pc: 0x00,
            carry: false,
            acc: 0x00,
            index: [0x00; 16],
            stack: [0x00; 3],
            ram_addr: 0x0,
            signal: false, 
            rom: Intel4001::new(),
        }
    }
  
    pub fn clock(&mut self) {
        self.decode_op(self.rom.fetch_u8(self.pc.into()));
    }

    // --- Getters and setters ---

    pub fn get_pc(&self) -> u8 {
        self.pc
    }

    pub fn get_carry(&self) -> bool {
        self.carry
    }

    pub fn get_acc(&self) -> u8 {
        self.acc
    }

    pub fn get_index(&self) -> &[u8; 16] {
        &self.index
    }

    pub fn get_stack(&self) -> &[u16; 3] {
        &self.stack
    }

    // --- Instructions ---

    // 1-word instructions take 1 instruction cycle while 2-word intructions take 2.
    fn decode_op(&mut self, op_code: u8) {
        match op_code & 0xF0{
            // Machine instructions
            0x00 => self.nop()       ,
            0x10 => self.jcn(op_code & 0x0F), // 2-word instruction
            0x20 => {                  // Check last 4 bit to see which function to call 
                if (op_code & 0x0F) % 2 == 0 {
                    self.fim(op_code & 0x0F);
                } else {
                    self.src(op_code & 0x0F);
                }
            }, 
            0x30 => {                  // Check last 4 bit to see which function to call 
                if (op_code & 0x0F) % 2 == 0 {
                    self.fin(op_code & 0x0F);
                } else {
                    self.jin(op_code & 0x0F);
                }
            },
            0x40 => self.jun(op_code & 0x0F), // 2-word instruction
            0x50 => self.jms(op_code & 0x0F), // 2-word instruction
            0x60 => self.inc(op_code & 0x0F),
            0x70 => self.isz(op_code & 0x0F), // 2-word instruction
            0x80 => self.add(op_code & 0x0F),
            0x90 => self.sub(op_code & 0x0F),
            0xA0 => self.ld(op_code & 0x0F),
            0xB0 => self.xch(op_code & 0x0F),
            0xC0 => self.bbl(op_code & 0x0F),
            0xD0 => self.ldm(op_code & 0x0F),
            _ => self.pc += 1        
        }
    }

    // A1, A2, A3 cycles are used to request data from the ROM, then M1 and M2 cycles are used to send the data to the CPU. 
    // Finally, the X1, X2 and X3 cycles are used to interpret and execute the instruction.

    // No operation.
    fn nop(&mut self) {
        self.pc += 1;
    }

    // Jump to ROM address X if condition is true, otherwise skip.
    fn jcn(&mut self, opa: u8) {

        let condition = opa & 0x0F;

        let C1 = (condition & 0x8) >> 3;
        let C2 = (condition & 0x4) >> 2;
        let C3 = (condition & 0x2) >> 1;
        let C4 = (condition & 0x1);

        self.pc += 1;

        if C1 != 1 && ((self.acc == 0 && C2 == 1) || (self.carry && C3 == 1) || (self.signal && C4 == 1)) {
            self.pc = self.rom.fetch_u8(self.pc.into());
        } else {
            self.pc += 1;
        }
    }

    // Fetch immediate from ROM data X to specified index register pair.
    fn fim(&mut self, opa: u8) {

        self.pc += 1;

        self.index[((opa >> 1) * 2) as usize] = self.rom.fetch_u8(self.pc.into());

        self.pc += 1;
    }

    // Send register control. Send the context of the specified index pair to ROM and RAM at set time.
    fn src(&mut self, opa: u8) {

        self.ram_addr = opa >> 1;
        
        self.pc += 1;
    }

    // Fetch indirect from ROM. Send content of index register pair location 0 out as an address. Data fetched is placed in specied register pair.
    fn fin(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Jump indirect. Send content of specified register pair out as an address at A1 and A2 time in the Instruction Cyle. (?)
    fn jin(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Jump unconditional. To specified address.
    fn jun(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Jump to subroutine of specified ROM address, save onl address(Up 1 level in stack).
    fn jms(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Increment contect of specified register.
    fn inc(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Increment contect of specified register. Go to specified ROM address if result != 0, otherwise skip/
    fn isz(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 2;
    }

    // Add contents of specified register to accumulator with carry.
    fn add(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Subtract contents of specified register to accumulator with borrow. 
    fn sub(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Load contents of specified register to accumulator.
    fn ld(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Exchange contents of specified index register and accumulator.
    fn xch(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Branch back (down 1 level in stack) and load specified data to accumulator.
    fn bbl(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }

    // Load specified data to accumulator
    fn ldm(&mut self, opa: u8) {

        // TODO: internal magic

        self.pc += 1;
    }
}
