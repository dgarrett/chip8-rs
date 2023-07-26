pub struct CPU {
    reg: [u8; 16],
    pc: usize,
    mem: [u8; 0x1000],
    stack: [u16; 16],
    sp: usize,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: [0; 16],
            pc: 0,
            mem: [0; 4096],
            stack: [0; 16],
            sp: 0,
        }
    }

    fn read_opcode(&self) -> u16 {
        let pc = self.pc as usize;
        let op_byte1 = self.mem[pc] as u16;
        let op_byte2 = self.mem[pc + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;
            // let kk = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.sp;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        stack[sp] = self.pc as u16;
        self.sp += 1;
        self.pc = addr as usize;
    }

    fn ret(&mut self) {
        if self.sp == 0 {
            panic!("Stack underflow")
        }

        self.sp -= 1;
        let addr = self.stack[self.sp];
        self.pc = addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let x_val = self.reg[x as usize];
        let y_val = self.reg[y as usize];

        let (val, overflow) = x_val.overflowing_add(y_val);
        self.reg[x as usize] = val;

        self.reg[0xF] = overflow as u8;
    }
}

#[test]
fn test_add() {
    let mut cpu = CPU::new();

    cpu.reg[0] = 5;
    cpu.reg[1] = 10;
    cpu.reg[2] = 10;
    cpu.reg[3] = 10;

    let mem = &mut cpu.mem;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.reg[0], 35);
}

#[test]
fn test_add_twice() {
    let mut cpu = CPU::new();

    cpu.reg[0] = 5;
    cpu.reg[1] = 10;

    // call twice
    let test_call = [0x21, 0x00, 0x21, 0x00, 0x00, 0x00];

    cpu.mem[0x0..0x6].copy_from_slice(&test_call);

    // add B to A twice
    let add_twice = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];

    cpu.mem[0x100..0x106].copy_from_slice(&add_twice);

    cpu.run();

    // A = (A + B + B)
    // call 2x
    // 5 + 10 + 10 + 10 + 10 == 45
    assert_eq!(cpu.reg[0], 45);
}
