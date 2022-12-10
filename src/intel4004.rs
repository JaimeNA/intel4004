use super::intel4001::Intel4001;

// Stack

struct Stack{                                                        // 3 x 12 bits array
    addrs: [u16; 3],
    sp: u8,                                                          // Stack Pointer
}

impl Stack {

    pub fn new() -> Self {
        Stack{
            addrs: [0x00; 3],
            sp: 0x00,
        }
    }

    pub fn push(&mut self, addr: u16) {
        self.addrs[self.sp as usize] = addr;

        if self.sp < 3 {
            self.sp += 1;
        }
    }

    pub fn pop(&mut self) -> u16 {
        let addr = self.addrs[self.sp as usize];
        self.addrs[self.sp as usize] = 0x00;                                  // Reset stack level.

        if self.sp > 0 {
            self.sp -= 1;                                            // Down 1 level if the sp is not on level 0.
        }

        addr
    }

}

// Intel 4004(CPU)

pub struct Intel4004 {
    pc:    u16,
    carry: bool, 
    acc:   u8,
    index: [u8; 16],                                                 // Dynamic RAM cell array of 16 x 4 bits.
    stack: Stack,                                                 
    ram_addr: u8,
    rom_addr: u8,
    signal: bool,
    pub rom: Intel4001,                                            
}

impl Intel4004 {
    pub fn new() -> Self {
        Intel4004 {
            pc: 0x00,
            carry: false,
            acc: 0x00,
            index: [0x00; 16],
            stack: Stack::new(),
            ram_addr: 0x0,
            rom_addr: 0x0,
            signal: false, 
            rom: Intel4001::new(),
        }
    }
  
    pub fn clock(&mut self) {
        self.decode_op(self.rom.fetch_u8(self.pc.into()));
    }

    // --- Getters and setters ---

    pub fn get_pc(&self) -> u16 {
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
        &self.stack.addrs
    }

    // --- Instructions ---

    // 1-word instructions take 1 instruction cycle while 2-word intructions take 2.
    fn decode_op(&mut self, op_code: u8) {
        match op_code & 0xF0{
            // Machine instructions
            0x00 => self.nop()       ,
            0x10 => self.jcn(op_code & 0x0F),  // 2-word instruction
            0x20 => {                          // Check last 4 bit to see which function to call.
                if (op_code & 0x0F) % 2 == 0 {
                    self.fim(op_code & 0x0F);
                } else {
                    self.src(op_code & 0x0F);
                }
            }, 
            0x30 => {                          // Check last 4 bit to see which function to call. 
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
            _ => {                            // Temporarly to test functions.
                self.stack.push(0x01);
                self.bbl(0x2);
            }     
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

        let c1 = (condition & 0x8) >> 3;
        let c2 = (condition & 0x4) >> 2;
        let c3 = (condition & 0x2) >> 1;
        let c4 = condition & 0x1;

        self.pc += 1;

        if c1 != 1 && ((self.acc == 0 && c2 == 1) || (self.carry && c3 == 1) || (self.signal && c4 == 1)) {
            self.pc = self.rom.fetch_u8(self.pc.into()) as u16;
        } else {
            self.pc += 1;
        }
    }

    // Fetch immediate from ROM data X to specified index register pair.
    fn fim(&mut self, opa: u8) {

        self.pc += 1;

        let rp = ((opa >> 1) * 2) as usize;

        self.index[rp] = self.rom.fetch_u8(self.pc.into());

        self.pc += 1;
    }

    // Send register control. Send the context of the specified index pair to ROM and RAM at set time.
    fn src(&mut self, opa: u8) {
        self.pc += 1;

        self.ram_addr = opa >> 1;
    }

    // Fetch indirect from ROM. Send content of index register pair location 0 out as an address. Data fetched is placed in specied register pair.
    fn fin(&mut self, opa: u8) {
        self.pc += 1;

        let rp = ((opa >> 1) * 2) as usize;
        self.index[rp] = self.rom.fetch_u8(self.index[0] as usize);
    }

    // Jump indirect. Send contents of register pair RRR out as an address at A1 and A2 time (ROM fetch cycles).
    fn jin(&mut self, opa: u8) {
        self.pc += 1;

        let rp = ((opa >> 1) * 2) as usize;
        self.rom_addr = self.index[rp];
    }

    // Jump unconditional. To specified address.
    fn jun(&mut self, opa: u8) {
        self.pc += 1;

        self.pc = ((opa & 0x0F) as u16 * 256) + (self.rom.fetch_u8(self.pc.into()) as u16);          // Join the last 4 bits of OPA with the next 8 bits.
    }

    // Jump to subroutine of specified ROM address, save on address(Up 1 level in stack).
    fn jms(&mut self, opa: u8) {
        self.stack.push(self.pc);

        self.pc = ((opa & 0x0F) as u16 * 256) + (self.rom.fetch_u8(self.pc.into()) as u16);          // Join the last 4 bits of OPA with the next 8 bits.
    }

    // Increment contect of specified register.
    fn inc(&mut self, opa: u8) {
        self.pc += 1;
        
        let reg_addr =(opa & 0x0F) as usize;
        self.index[reg_addr] += 1;
    }

    // Increment contect of specified register. Go to specified ROM address if result != 0, otherwise skip/
    fn isz(&mut self, opa: u8) {
        self.pc += 1;

        let rom_addr = self.rom.fetch_u8(self.pc.into()) as u16;
        let reg_addr =(opa & 0x0F) as usize;

        self.index[reg_addr] += 1;
        if  self.index[reg_addr] != 0 {
            self.pc = rom_addr;
        } else {
            self.pc += 1;
        }
    }

    // Add contents of specified register to accumulator with carry.
    fn add(&mut self, opa: u8) {
        self.pc += 1;

        let reg_addr = (opa & 0x0F) as usize;

        self.acc += self.index[reg_addr] + self.carry as u8;
        self.carry = false;

        if self.acc & 0xF0 != 0 {             // Check if the accumulator value is greater than 4 bits.
            self.acc &= 0x0F;                 // Remove the extra bits.
            self.carry = true;
        }
    }

    // Subtract contents of specified register to accumulator with borrow. 
    fn sub(&mut self, opa: u8) {
        self.pc += 1;

        let reg_addr = (opa & 0x0F) as usize;

        self.acc -= self.index[reg_addr] + !self.carry as u8;
        self.carry = false;

        if self.acc & 0xF0 != 0 {             
            self.acc &= 0x0F;                 
            self.carry = true;
        }
    }

    // Load contents of specified register to accumulator.
    fn ld(&mut self, opa: u8) {
        let reg_addr = (opa & 0x0F) as usize;

        self.acc = self.index[reg_addr];

        self.pc += 1;
    }

    // Exchange contents of specified index register and accumulator.
    fn xch(&mut self, opa: u8) {
        self.pc += 1;

        let reg_addr = (opa & 0x0F) as usize;

        let temp = self.acc;
        self.acc = self.index[reg_addr];
        self.index[reg_addr] = temp;
    }

    // Branch back (down 1 level in stack) and load specified data to accumulator.
    fn bbl(&mut self, opa: u8) {
        self.pc = self.stack.pop();
        self.acc = opa & 0x0F;
    }

    // Load specified data to accumulator
    fn ldm(&mut self, opa: u8) {
        self.pc += 1;

        self.acc = opa & 0x0F;
    }
}
