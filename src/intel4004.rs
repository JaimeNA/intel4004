use super::intel4001::Intel4001;
use super::intel4002::Intel4002;

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
    signal: bool,
    ram_addrs: u8,
    pub rom: Intel4001,     
    pub ram: Intel4002,                                             // For now it will only work with one RAM chip           
}

impl Intel4004 {
    pub fn new() -> Self {
        Intel4004 {
            pc: 0x00,
            carry: false,
            acc: 0x00,
            index: [0x00; 16],
            stack: Stack::new(),
            signal: false, 
            ram_addrs: 0x00,
            rom: Intel4001::new(),
            ram: Intel4002::new(),
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

    /// 1-word instructions take 1 instruction cycle while 2-word intructions take 2.
    fn decode_op(&mut self, op_code: u8) {
        match op_code & 0xF0{
            // Machine instructions
            0x00 => self.nop()       ,
            0x10 => self.jcn(op_code & 0x0F),  // 2-word instruction
            0x20 => {                          // Check last 4 bit to see which instruction to call.
                if (op_code & 0x0F) % 2 == 0 {
                    self.fim(op_code & 0x0F);
                } else {
                    self.src(op_code & 0x0F);
                }
            }, 
            0x30 => {                          // Check last 4 bit to see which instruction to call. 
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

            // Input/Output and RAM instructions
            0xE0 => match op_code & 0x0F {
                0x00 => self.wrm(),
                0x01 => self.wmp(),
                0x02 => self.wrr(),
                0x03 => self.wpm(),
                0x04 => self.wr0(),
                0x05 => self.wr1(),
                0x06 => self.wr2(),
                0x07 => self.wr3(),
                0x08 => self.sbm(),
                0x09 => self.rdm(),
                0x0A => self.rdr(),
                0x0B => self.adm(),
                0x0C => self.rd0(),
                0x0D => self.rd1(),
                0x0E => self.rd2(),
                0x0F => self.rd3(),
                _ => self.nop()
            },
            _ => self.nop()
        }
    }

    // A1, A2, A3 cycles are used to request data from the ROM, then M1 and M2 cycles are used to send the data to the CPU. 
    // Finally, the X1, X2 and X3 cycles are used to interpret and execute the instruction.

    // --- Machine instructions ---

    /// No operation.
    fn nop(&mut self) {
        self.pc += 1;
    }

    /// Jump to ROM address X if condition is true, otherwise skip.
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

    /// Fetch immediate from ROM data X to specified index register pair.
    fn fim(&mut self, opa: u8) {

        self.pc += 1;

        let rp = ((opa >> 1) * 2) as usize;

        self.index[rp] = self.rom.fetch_u8(self.pc.into());

        self.pc += 1;
    }

    /// Send register control. Send the context of the specified index pair to ROM and RAM at set time.
    fn src(&mut self, opa: u8) {
        self.pc += 1;

        self.ram_addrs = opa >> 1;
    }

    /// Fetch indirect from ROM. Send content of index register pair location 0 out as an address. Data fetched is placed in specied register pair.
    fn fin(&mut self, opa: u8) {
        self.pc += 1;

        let rp = ((opa >> 1) * 2) as usize;
        self.index[rp] = self.rom.fetch_u8(self.index[0] as usize);
    }

    /// Jump indirect. Send contents of register pair RRR out as an address at A1 and A2 time (ROM fetch cycles).
    fn jin(&mut self, opa: u8) {
        self.pc += 1;

        let rp = ((opa >> 1) * 2) as usize;
        self.ram_addrs = self.index[rp];
    }

    /// Jump unconditional. To specified address.
    fn jun(&mut self, opa: u8) {
        self.pc += 1;

        self.pc = ((opa & 0x0F) as u16 * 256) + (self.rom.fetch_u8(self.pc.into()) as u16);          // Join the last 4 bits of OPA with the next 8 bits.
    }

    /// Jump to subroutine of specified ROM address, save on address(Up 1 level in stack).
    fn jms(&mut self, opa: u8) {
        self.stack.push(self.pc);

        self.pc = ((opa & 0x0F) as u16 * 256) + (self.rom.fetch_u8(self.pc.into()) as u16);          // Join the last 4 bits of OPA with the next 8 bits.
    }

    /// Increment contect of specified register.
    fn inc(&mut self, opa: u8) {
        self.pc += 1;
        
        let reg_addr =(opa & 0x0F) as usize;
        self.index[reg_addr] += 1;
    }

    /// Increment contect of specified register. Go to specified ROM address if result != 0, otherwise skip/
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

    /// Add contents of specified register to accumulator with carry.
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

    /// Subtract contents of specified register to accumulator with borrow. 
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

    /// Load contents of specified register to accumulator.
    fn ld(&mut self, opa: u8) {
        let reg_addr = (opa & 0x0F) as usize;

        self.acc = self.index[reg_addr];

        self.pc += 1;
    }

    /// Exchange contents of specified index register and accumulator.
    fn xch(&mut self, opa: u8) {
        self.pc += 1;

        let reg_addr = (opa & 0x0F) as usize;

        let temp = self.acc;
        self.acc = self.index[reg_addr];
        self.index[reg_addr] = temp;
    }

    /// Branch back (down 1 level in stack) and load specified data to accumulator.
    fn bbl(&mut self, opa: u8) {
        self.pc = self.stack.pop();
        self.acc = opa & 0x0F;
    }

    /// Load specified data to accumulator
    fn ldm(&mut self, opa: u8) {
        self.pc += 1;

        self.acc = opa & 0x0F;
    }

    // --- Input/Output and RAM instructions ---

    /// Write contents of the accumulator into the previously selected RAM main memory character.
    fn wrm(&mut self) {
        self.pc += 1;

        self.ram.ram[self.ram_addrs as usize] = self.acc; // TODO: Find better way to access RAM.
    }

    /// Write contents of the accumulator into the previously selected RAM output port(output lines).
    fn wmp(&mut self) {
        self.pc += 1;

        self.ram.output = self.acc;
    }

    /// Write contents of the accumulator into the previously selected ROM output port(I/O lines).
    fn wrr(&mut self) {
        self.pc += 1;

        self.rom.io = self.acc;
    }

    /// Write the contents of the accumulator into the previously selected half byte of read/write program memory (for use with the 4008/4009 only).
    fn wpm(&mut self) {
        self.pc += 1; // Do nothing as there are no 4008/4009 implemented
    }

    /// Write the contents of the accumulator into the previously selected RAM status character 0.
    fn wr0(&mut self) {
        self.pc += 1;

       let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03;        // Each register has 16 main memory characters and 4 status characters
       self.ram.status[(ram_register * 4) as usize] = self.acc;         // 0 - 4 - 8 - C <- possible status index
    }

    /// Write the contents of the accumulator into the previously selected RAM status character 1.
    fn wr1(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.ram.status[((ram_register * 4) + 1) as usize] = self.acc;  // 1 - 5 - 9 - D
    }

    /// Write the contents of the accumulator into the previously selected RAM status character 2.
    fn wr2(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.ram.status[((ram_register * 4) + 2) as usize] = self.acc;  // 2 - 6 - A - E
    }

    /// Write the contents of the accumulator into the previously selected RAM status character 3.
    fn wr3(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.ram.status[((ram_register * 4) + 3) as usize] = self.acc;  // 3 - 7 - B - F
    }

    /// Subtract the previous selected RAM main memory characted from accumulator with borrow.
    fn sbm(&mut self) {
        self.pc += 1;

        self.acc -= self.ram.ram[self.ram_addrs as usize] + !self.carry as u8;
        self.carry = false;

        if self.acc & 0xF0 != 0 {             
            self.acc &= 0x0F;                 
            self.carry = true;
        }
    }

    /// Read the previous selected RAM main memory character into the accumulator.
    fn rdm(&mut self) {
        self.pc += 1;

        self.acc = self.ram.ram[self.ram_addrs as usize];
    }

    /// Read the contents of the previous selected ROM input port into the accumulator(I/O lines).
    fn rdr(&mut self) {
        self.pc += 1;

        self.acc = self.rom.io;
    }

    /// Add the previous selected RAM main memory character to accumulator with carry.
    fn adm(&mut self) {
        self.pc += 1;

        self.acc += self.ram.ram[self.ram_addrs as usize] + self.carry as u8;
        self.carry = false;

        if self.acc & 0xF0 != 0 {             
            self.acc &= 0x0F;                 
            self.carry = true;
        }
    }

    /// Read the previous selected RAM status character 0 into accumulator.
    fn rd0(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.acc = self.ram.status[(ram_register * 4) as usize];
    }

    /// Read the previous selected RAM status character 1 into accumulator.
    fn rd1(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.acc = self.ram.status[((ram_register * 4) + 1) as usize];
    }

    /// Read the previous selected RAM status character 2 into accumulator.
    fn rd2(&mut self) {

        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.acc = self.ram.status[((ram_register * 4) + 2) as usize];
    }

    /// Read the previous selected RAM status character 3 into accumulator.
    fn rd3(&mut self) {

        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.acc = self.ram.status[((ram_register * 4) + 3) as usize];
    }

    
}
