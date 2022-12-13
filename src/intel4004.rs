use super::intel4001::Intel4001;
use super::intel4002::Intel4002;

use arbitrary_int::{u4};

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
    acc:   u4,
    index: [u4; 16],                                                 // Dynamic RAM cell array of 16 x 4 bits.
    stack: Stack,     
    signal: bool,
    command_control: u4,
    ram_addrs: u8,
    pub rom: Intel4001,     
    pub ram: Intel4002,                                             // For now it will only work with one RAM chip           
}

impl Intel4004 {
    pub fn new() -> Self {
        Intel4004 {
            pc: 0x00,
            carry: false,
            acc: u4::new(0x0),
            index: [u4::new(0x0); 16],
            stack: Stack::new(),
            signal: false, 
            command_control: u4::new(0x0),
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
        self.acc.value()
    }

    pub fn get_index(&self) -> &[u4; 16] {
        &self.index
    }

    pub fn get_stack(&self) -> &[u16; 3] {
        &self.stack.addrs
    }

    pub fn get_cc(&self) -> u8 {
        self.command_control.value()
    }

    pub fn get_ram_addrs(&self) -> u8 {
        self.ram_addrs
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn set_carry(&mut self, carry: bool) {
        self.carry = carry;
    }

    pub fn set_acc(&mut self, acc: u8) {
        self.acc = u4::new(acc);
    }

    pub fn set_index(&mut self, index: [u4; 16]) {
        self.index = index;
    }

    pub fn set_stack(&mut self, stack: [u16; 3]) {
        self.stack.addrs = stack;
    }

    pub fn set_cc(&mut self, cc: u8) {
        self.command_control = u4::new(cc);
    }

    pub fn set_ram_addrs(&mut self, ram_addrs: u8) {
        self.ram_addrs = ram_addrs;
    }

    // --- Instructions ---

    /// 1-word instructions take 1 instruction cycle while 2-word intructions take 2.
    pub fn decode_op(&mut self, op_code: u8) {
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

            // Accumulator group instructions
            0xF0 => match op_code & 0x0F {
                0x00 => self.clb(),
                0x01 => self.clc(),
                0x02 => self.iac(),
                0x03 => self.cmc(),
                0x04 => self.cma(),
                0x05 => self.ral(),
                0x06 => self.rar(),
                0x07 => self.tcc(),
                0x08 => self.dac(),
                0x09 => self.tcs(),
                0x0A => self.stc(),
                0x0B => self.daa(),
                0x0C => self.kbp(),
                0x0D => self.dcl(),
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

        if c1 != 1 && ((self.acc.value() == 0 && c2 == 1) || (self.carry && c3 == 1) || (self.signal && c4 == 1)) {
            self.pc = self.rom.fetch_u8(self.pc.into()) as u16;
        } else {
            self.pc += 1;
        }
    }

    /// Fetch immediate from ROM data X to specified index register pair.
    fn fim(&mut self, opa: u8) {

        self.pc += 1;

        let rp = ((opa >> 1) * 2) as usize;
        let value = self.rom.fetch_u8(self.pc.into());
        self.index[rp] = u4::new(value);

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
        let val = self.rom.fetch_u8(self.index[0].value() as usize);
        self.index[rp] = u4::new(val);
    }

    /// Jump indirect. Send contents of register pair RRR out as an address at A1 and A2 time (ROM fetch cycles).
    fn jin(&mut self, opa: u8) {
        self.pc += 1;

        let rp = ((opa >> 1) * 2) as usize;
        self.ram_addrs = self.index[rp].value();
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
        self.index[reg_addr] += u4::new(1);
    }

    /// Increment contect of specified register. Go to specified ROM address if result != 0, otherwise skip/
    fn isz(&mut self, opa: u8) {
        self.pc += 1;

        let rom_addr = self.rom.fetch_u8(self.pc.into()) as u16;
        let reg_addr =(opa & 0x0F) as usize;

        if self.index[reg_addr].value() != 0x0F {
            self.index[reg_addr] += u4::new(1);
        }
        if  self.index[reg_addr].value() != 0 {
            self.pc = rom_addr;
        } else {
            self.pc += 1;
        }
    }

    /// Add contents of specified register to accumulator with carry.
    fn add(&mut self, opa: u8) {
        self.pc += 1;

        let reg_addr = (opa & 0x0F) as usize;
        let mut val = self.acc.value();

        val += self.index[reg_addr].value() + self.carry as u8;
        self.carry = false;

        if val & 0xF0 != 0 {             // Check if the accumulator value is greater than 4 bits.
            val &= 0x0F;                 // Remove the extra bits.
            self.carry = true;
        }
        self.acc = u4::new(val);
    }

    /// Subtract contents of specified register to accumulator with borrow. 
    fn sub(&mut self, opa: u8) {
        self.pc += 1;

        let reg_addr = (opa & 0x0F) as usize;
        let mut val = self.acc.value();

        val -= self.index[reg_addr].value() + !self.carry as u8;
        self.carry = false;

        if val & 0xF0 != 0 {             
            val &= 0x0F;                 
            self.carry = true;
        }
        self.acc = u4::new(val);
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

        let temp = self.acc.value();
        self.acc = self.index[reg_addr];
        self.index[reg_addr] = u4::new(temp);
    }

    /// Branch back (down 1 level in stack) and load specified data to accumulator.
    fn bbl(&mut self, opa: u8) {
        self.pc = self.stack.pop();
        self.acc = u4::new(opa & 0x0F);
    }

    /// Load specified data to accumulator
    fn ldm(&mut self, opa: u8) {
        self.pc += 1;

        self.acc = u4::new(opa & 0x0F);
    }

    // --- Input/Output and RAM instructions ---

    /// Write contents of the accumulator into the previously selected RAM main memory character.
    fn wrm(&mut self) {
        self.pc += 1;

        self.ram.ram[self.ram_addrs as usize] = self.acc.value(); // TODO: Find better way to access RAM.
    }

    /// Write contents of the accumulator into the previously selected RAM output port(output lines).
    fn wmp(&mut self) {
        self.pc += 1;

        self.ram.output = self.acc.value();
    }

    /// Write contents of the accumulator into the previously selected ROM output port(I/O lines).
    fn wrr(&mut self) {
        self.pc += 1;

        self.rom.io = self.acc.value();
    }

    /// Write the contents of the accumulator into the previously selected half byte of read/write program memory (for use with the 4008/4009 only).
    fn wpm(&mut self) {
        self.pc += 1; // Do nothing as there are no 4008/4009 implemented
    }

    /// Write the contents of the accumulator into the previously selected RAM status character 0.
    fn wr0(&mut self) {
        self.pc += 1;

       let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03;                // Each register has 16 main memory characters and 4 status characters
       self.ram.status[(ram_register * 4) as usize] = self.acc.value();         // 0 - 4 - 8 - C <- possible status index
    }

    /// Write the contents of the accumulator into the previously selected RAM status character 1.
    fn wr1(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.ram.status[((ram_register * 4) + 1) as usize] = self.acc.value();  // 1 - 5 - 9 - D
    }

    /// Write the contents of the accumulator into the previously selected RAM status character 2.
    fn wr2(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.ram.status[((ram_register * 4) + 2) as usize] = self.acc.value();  // 2 - 6 - A - E
    }

    /// Write the contents of the accumulator into the previously selected RAM status character 3.
    fn wr3(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        self.ram.status[((ram_register * 4) + 3) as usize] = self.acc.value();  // 3 - 7 - B - F
    }

    /// Subtract the previous selected RAM main memory characted from accumulator with borrow.
    fn sbm(&mut self) {
        self.pc += 1;

        let mut val = self.acc.value();
        val -= self.ram.ram[self.ram_addrs as usize] + self.carry as u8;
        self.carry = false;

        if val & 0xF0 != 0 {             
            val &= 0xF;                 
            self.carry = true;
        }
        self.acc = u4::new(val);
    }

    /// Read the previous selected RAM main memory character into the accumulator.
    fn rdm(&mut self) {
        self.pc += 1;

        let val = self.ram.ram[self.ram_addrs as usize];
        self.acc = u4::new(val);
    }

    /// Read the contents of the previous selected ROM input port into the accumulator(I/O lines).
    fn rdr(&mut self) {
        self.pc += 1;

        self.acc = u4::new(self.rom.io);
    }

    /// Add the previous selected RAM main memory character to accumulator with carry.
    fn adm(&mut self) {
        self.pc += 1;

        let mut val = self.acc.value();

        val += self.ram.ram[self.ram_addrs as usize] + self.carry as u8;
        self.carry = false;

        if val & 0xF0 != 0 {             
            val &= 0xF;                 
            self.carry = true;
        }
        self.acc = u4::new(val);
    }

    /// Read the previous selected RAM status character 0 into accumulator.
    fn rd0(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        let val = self.ram.status[(ram_register * 4) as usize];
        self.acc = u4::new(val);
    }

    /// Read the previous selected RAM status character 1 into accumulator.
    fn rd1(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        let val = self.ram.status[((ram_register * 4) + 1) as usize];
        self.acc = u4::new(val);
    }

    /// Read the previous selected RAM status character 2 into accumulator.
    fn rd2(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        let val = self.ram.status[((ram_register * 4) + 2) as usize];
        self.acc = u4::new(val);
    }

    /// Read the previous selected RAM status character 3 into accumulator.
    fn rd3(&mut self) {
        self.pc += 1;

        let ram_register = ((self.ram_addrs & 0xF0) >> 4) & 0x03; 
        let val = self.ram.status[((ram_register * 4) + 3) as usize];
        self.acc = u4::new(val);
    }

    // --- Accumulator group instructions ---
    
    /// Clear accumulator and carry.
    fn clb(&mut self) {
        self.pc += 1;

        self.acc = u4::new(0x00);
        self.carry = false;
    }
    
    /// Clear carry.
    fn clc(&mut self) {
        self.pc += 1;

        self.carry = false;
    }
    
    /// Increment accumulator.
    fn iac(&mut self) {
        self.pc += 1;

        self.acc += u4::new(1);
    }
    
    /// Complement carry.
    fn cmc(&mut self) {
        self.pc += 1;

        self.carry = !self.carry;
    }
    
    /// Complement accumulator.
    fn cma(&mut self) {
        self.pc += 1;

        self.acc = !self.acc;
    }
    
    /// Rotate left accumulator and carry.
    fn ral(&mut self) {
        self.pc += 1;

        let new_carry = matches!(self.acc.value() & 8, 8); // Check if there is a digit at the end of the accumulator(supposed to be 4 bits).

        self.acc <<= 1;
        if self.carry {
            self.acc |= u4::new(1);
        }

        self.carry = new_carry;
    }
    
    /// Rotate left accumulator and carry.    
    fn rar(&mut self) {
        self.pc += 1;

        let new_carry = matches!(self.acc.value() & 1, 1); // Check if there is a digit at the start of the accumulator(supposed to be 4 bits).

        self.acc >>= 1;
        if self.carry {
            self.acc |= u4::new(8);
        }
        
        self.carry = new_carry;
    }
    
    /// Transmit carry to accumulator and clear carry.
    fn tcc(&mut self) {
        self.pc += 1;

        self.acc = u4::new(self.carry as u8);
        self.carry = false;
    }
    
    /// Decrement accumulator.
    fn dac(&mut self) {
        self.pc += 1;

        let mut val = self.acc.value();
        if val == 0 {           // Can't be negative.
            val = 0x0F;
            self.carry = false;
        } else {
            val -= 1;
            self.carry = true;       // Carry is reversed.
        }
        self.acc = u4::new(val);
    }
    
    /// Transfer carry subtract and clear carry.
    fn tcs(&mut self) {
        self.pc += 1;

        if self.carry {
            self.acc = u4::new(10);
        } else {
            self.acc = u4::new(9);
        }
        self.carry = false;
    }
    
    /// Set carry.
    fn stc(&mut self) {
        self.pc += 1;

        self.carry = true;
    }
    
    /// Decimal adjust accumulator.
    fn daa(&mut self) {
        self.pc += 1;

        let mut val = self.acc.value();
        if self.carry || val > 9 {
            val += 6;
            self.acc = u4::new(val & 0xF);
            if val > 0xF {
                self.carry = true;
            }
        }
    }
    
    /// Keyboard process. Converts contents of the accumulator from a one out of four code to a binary code.
    fn kbp(&mut self) {
        self.pc += 1;

        self.acc = u4::new(match self.acc.value() {
            0b0000 => 0,
            0b0001 => 1,
            0b0010 => 2,
            0b0100 => 3,
            0b1000 => 4,
            _ => 0xF,
          });
    }
    
    /// Designate command line.
    fn dcl(&mut self) {
        self.pc += 1;

        self.command_control = self.acc;
    }
}
