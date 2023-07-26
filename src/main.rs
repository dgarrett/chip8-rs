mod cpu;

use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.run();
}
