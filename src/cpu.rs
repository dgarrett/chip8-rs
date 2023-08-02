pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct CPU {
    reg: [u8; 16],
    pc: usize,
    mem: [u8; 0x1000],
    stack: [u16; 16],
    sp: usize,
    i: u16,
    disp: [u32; WIDTH * HEIGHT],
    halted: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: [0; 16],
            pc: 0x200,
            mem: [0; 4096],
            stack: [0; 16],
            sp: 0,
            i: 0,
            disp: [0xffff; WIDTH * HEIGHT],
            halted: false,
        }
    }

    fn read_opcode(&self) -> u16 {
        let pc = self.pc as usize;
        let op_byte1 = self.mem[pc] as u16;
        let op_byte2 = self.mem[pc + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn step(&mut self) -> bool {
        let opcode = self.read_opcode();
        self.pc += 2;

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;

        println!(
            "opcode {:04x} pc {:04x} regs {:?}",
            opcode, self.pc, self.reg
        );

        match (c, x, y, d) {
            (0, 0, 0, 0) => self.halted = true,
            (0, 0, 0xE, 0) => self.disp_clear(),
            (0, 0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jump(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.eq(x, kk),
            (0x4, _, _, _) => self.neq(x, kk),
            (0x5, _, _, 0) => self.eq_xy(x, y),
            (0x6, _, _, _) => self.set(x, kk),
            (0x7, _, _, _) => self.inc(x, kk),
            (0x8, _, _, 0) => self.assn(x, y),
            (0x8, _, _, 0x1) => self.or_xy(x, y),
            (0x8, _, _, 0x2) => self.and_xy(x, y),
            (0x8, _, _, 0x3) => self.xor_xy(x, y),
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            (0x8, _, _, 0x5) => self.sub_xy(x, y),
            (0x8, _, _, 0x6) => self.rshift(x),
            (0x8, _, _, 0x7) => self.isub_xy(x, y),
            (0x8, _, _, 0xE) => self.lshift(x),
            (0x9, _, _, 0) => self.neq_xy(x, y),
            (0xA, _, _, _) => self.set_i(nnn),
            (0xB, _, _, _) => self.jmp_off(nnn),
            (0xC, _, _, _) => self.rand(x, kk),
            (0xD, _, _, _) => {
                self.draw(x, y, d);
                return true;
            }
            _ => panic!("bad opcode {:04x}", opcode),
        }

        false
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn disp(&self) -> &[u32] {
        &self.disp
    }

    pub fn load(&mut self, rom: &[u8]) {
        self.mem[0x200..(0x200 + rom.len())].copy_from_slice(rom);
    }

    fn disp_clear(&mut self) {
        self.disp = [0; WIDTH * HEIGHT];
    }

    fn ret(&mut self) {
        if self.sp == 0 {
            panic!("Stack underflow")
        }

        self.sp -= 1;
        let addr = self.stack[self.sp];
        self.pc = addr as usize;
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr as usize;
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

    fn eq(&mut self, x: u8, kk: u8) {
        let x_val = self.reg[x as usize];
        if x_val == kk {
            self.pc += 2;
        }
    }

    fn neq(&mut self, x: u8, kk: u8) {
        let x_val = self.reg[x as usize];
        if x_val != kk {
            self.pc += 2;
        }
    }

    fn eq_xy(&mut self, x: u8, y: u8) {
        let x_val = self.reg[x as usize];
        let y_val = self.reg[y as usize];
        if x_val == y_val {
            self.pc += 2;
        }
    }

    fn set(&mut self, x: u8, kk: u8) {
        self.reg[x as usize] = kk;
    }

    fn inc(&mut self, x: u8, kk: u8) {
        self.reg[x as usize] += kk;
        // no flag
    }

    fn assn(&mut self, x: u8, y: u8) {
        self.reg[x as usize] = self.reg[y as usize];
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        let x_val = self.reg[x as usize];
        let y_val = self.reg[y as usize];

        self.reg[x as usize] = x_val | y_val;
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        let x_val = self.reg[x as usize];
        let y_val = self.reg[y as usize];

        self.reg[x as usize] = x_val & y_val;
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_val = self.reg[x as usize];
        let y_val = self.reg[y as usize];

        self.reg[x as usize] = x_val ^ y_val;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let x_val = self.reg[x as usize];
        let y_val = self.reg[y as usize];

        let (val, overflow) = x_val.overflowing_add(y_val);
        self.reg[x as usize] = val;

        self.reg[0xF] = overflow as u8;
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        let x_val = self.reg[x as usize];
        let y_val = self.reg[y as usize];

        let (val, overflow) = x_val.overflowing_sub(y_val);
        self.reg[x as usize] = val;

        self.reg[0xF] = overflow as u8;
    }

    fn rshift(&mut self, x: u8) {
        let x_val = self.reg[x as usize];

        self.reg[0xF] = x_val & 0x1;
        self.reg[x as usize] = x_val >> 1;
    }

    fn isub_xy(&mut self, x: u8, y: u8) {
        let x_val = self.reg[x as usize];
        let y_val = self.reg[y as usize];

        let (val, overflow) = y_val.overflowing_sub(x_val);
        self.reg[x as usize] = val;

        self.reg[0xF] = overflow as u8;
    }

    fn lshift(&mut self, x: u8) {
        let x_val = self.reg[x as usize];

        self.reg[0xF] = x_val & 0x80;
        self.reg[x as usize] = x_val << 1;
    }

    fn neq_xy(&mut self, x: u8, y: u8) {
        let x_val = self.reg[x as usize];
        let y_val = self.reg[y as usize];

        if x_val != y_val {
            self.pc += 2;
        }
    }

    fn set_i(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn jmp_off(&mut self, nnn: u16) {
        self.pc = self.reg[0] as usize + nnn as usize;
    }

    fn rand(&mut self, x: u8, kk: u8) {
        self.reg[x as usize] = rand::random::<u8>() & kk;
    }

    fn set_px(&mut self, x: u8, y: u8, set: bool) -> bool {
        let pixel = &mut self.disp[x as usize + (y as usize * WIDTH)];
        let was_set = *pixel == 0xffff;

        *pixel = if set { 0xffff } else { 0 };

        was_set && (*pixel == 0)
    }

    fn draw(&mut self, x: u8, y: u8, d: u8) {
        self.reg[0xF] = 0;

        for y in y..(y + d) {
            let row_pxls = self.mem[self.i as usize + y as usize];
            for x_off in 0..8 {
                // let shift = 0x80 >> x_off;
                // let and = row_pxls & shift;
                // let set = and != 0;
                let set = (row_pxls & (0x80 >> x_off)) != 0;
                let unset = self.set_px(x + x_off, y, set);
                if unset {
                    self.reg[0xF] = 1;
                }
            }
        }
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
