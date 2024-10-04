//! Emulating the computer's instruction set in software

fn main() {
    let register_file = RegisterFile {
        r0: 0, r5: 0, r10: 0, pc: 0x200,
        r1: 0, r6: 0, r11: 0, cpsr: 0x00000000,
        r2: 0, r7: 0, r12: 0,
        r3: 0, r8: 0, r13: 0,
        r4: 0, r9: 0, r14: 0,
    };

    let cpu = CPU{
        general_purpose_registers: register_file,
        application_program_status: register_file.cpsr,
    };
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct RegisterFile {
    r0: u32, r5: u32, r10: u32, pc: u32,
    r1: u32, r6: u32, r11: u32, cpsr: u32,
    r2: u32, r7: u32, r12: u32,
    r3: u32, r8: u32, r13: u32,
    r4: u32, r9: u32, r14: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct CPU {
    general_purpose_registers: RegisterFile,
    application_program_status: u32,
}

impl CPU {
    fn read_instr(&self, mem: &[usize]) -> u32 {
        // Reads instruction. Instructions are 32 bits long. 
        // The instruction here will determine how to interprate the rest of the bits.
        let pos = self.general_purpose_registers.pc;
        pos
    }

    fn load(&mut self, mem: &[usize]) {
        let instr = self.read_instr(&mem);
        // Advances to the next instruction in memory.
        // Effective pc is located 4 instructions ahead of the current one.
        // This implementation assumes 4 bits alignment.
        self.general_purpose_registers.pc += 4;

        // Decoding the instruction
        let opcode = ((instr & 0xFFC00000) >> 22) as u8;
        let x = ((instr & 0x003FFC00) >> 10) as u8;
        let y = ((instr & 0x000003E0) >> 5) as u8;
        let d = ((instr & 0x0000001F) >> 0) as u8;

        // Loading data from the memory into the registers
        match (opcode, x, y, d) {
            (0, 0, 0, 0) => { return; },
            (0x8, 0, 1, _) | (0x8, 1, 0, _) => {
                self.general_purpose_registers.r0 = mem[0 as usize] as u32;
                self.general_purpose_registers.r1 = mem[1 as usize] as u32;
                self.general_purpose_registers.r6 = mem[d as usize] as u32;
            },
            (0x8, 0, 2, _) | (0x8, 2, 0, _) => {
                self.general_purpose_registers.r0 = mem[0 as usize] as u32;
                self.general_purpose_registers.r2 = mem[1 as usize] as u32;
                self.general_purpose_registers.r6 = mem[d as usize] as u32;
            },
            (0x8, 0, 3, _) | (0x8, 3, 0, _) => {
                self.general_purpose_registers.r0 = mem[0 as usize] as u32;
                self.general_purpose_registers.r3 = mem[3 as usize] as u32;
                self.general_purpose_registers.r6 = mem[d as usize] as u32;
            },
            (0x8, 1, 2, _) | (0x8, 2, 1, _) => {
                self.general_purpose_registers.r1 = mem[1 as usize] as u32;
                self.general_purpose_registers.r2 = mem[2 as usize] as u32;
                self.general_purpose_registers.r6 = mem[d as usize] as u32;
            },
            (0x8, 1, 3, _) | (0x8, 3, 1, _) => {
                self.general_purpose_registers.r1 = mem[1 as usize] as u32;
                self.general_purpose_registers.r3 = mem[3 as usize] as u32;
                self.general_purpose_registers.r6 = mem[d as usize] as u32;
            },
            (0x8, 2, 3, _) | (0x8, 3, 2, _) => {
                self.general_purpose_registers.r2 = mem[2 as usize] as u32;
                self.general_purpose_registers.r3 = mem[3 as usize] as u32;
                self.general_purpose_registers.r6 = mem[d as usize] as u32;
            },
            _ => println!("Invalid instruction: instr {:04x}", instr),
        }
    }

    fn adds(&mut self, mut x: u16, y: u16) {
        // Target registers are passed as parameters.
        // Performing addition operation
        let (val, v_overflow_condition) = x.overflowing_add(y);
        x = val;

        // Value should be returned to the memory here via store here.

        if v_overflow_condition {
            // Setting overflow flag.
            let v = self.application_program_status ^ 0x08000000;
            self.application_program_status = v;
        }
    }

    fn subs(&mut self, mut x: u16, y: u16) {
        // Target registers are passed as parameters
        // Performing saturating sub operation. Underflows are set at 0.
        let val = x.saturating_sub(y);
        x = val;

        // Store functionality to return back the value to the memory.

        if x > y {
            // Setting negative value flag
            let n = (self.application_program_status ^ 0x80000000) >> 1;
            self.application_program_status = n;
        }
    }

    fn mul_(&mut self, mut r_x: u16, r_y: u16) {
        r_x *= r_y;
    }

    fn div_(&mut self, mut r_x: u8, r_y: u8) {
        let quotient = r_x / r_y;
        let remainder = r_x % r_y;
        r_x = quotient;
    }

    fn call(&mut self, addr: u16) {
        todo!("Calling a function");
    }

    fn ret(&mut self) {
        todo!("Returning a function back to its caller's address.")
    }

    fn run() {
        todo!("Running our programs");
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Stack([usize; 16]);

impl Stack {
    fn push(&mut self) {
        todo!("Implement adding data into the stack");
    }
    fn pop(&mut self) {
        todo!("Fetching data from the stack");
    }
    fn top(&self) {
        todo!("Return address pointer to the top frame.");
    }
    fn length(&self) {
        todo!("Length of the stack, or stack pointer");
    }
    fn is_empty(&self) {
        todo!("Check if the stack is empty");
    }
}
