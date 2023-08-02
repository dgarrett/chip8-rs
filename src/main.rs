mod cpu;

use clap::Parser;
use minifb::{Key, Scale, Window, WindowOptions};
use std::fs;

use crate::cpu::{CPU, HEIGHT, WIDTH};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ROM file path
    #[arg(
        default_value_t = ("/Users/dylangarrett/Downloads/chip8-roms-master/demos/Maze [David Winter, 199x].ch8".to_string())
    )]
    rom: String,
}

fn load_rom(path: &str, cpu: &mut CPU) -> std::io::Result<()> {
    println!("Loading {path}");
    let rom = fs::read(path)?;
    cpu.load(&rom);
    Ok(())
}

fn main() {
    let args = Args::parse();

    let mut cpu = CPU::new();
    load_rom(&args.rom, &mut cpu).expect("Failed to load");

    let mut winopts = WindowOptions::default();
    winopts.scale = Scale::X8;

    let mut window =
        Window::new("chip8-rs - ESC to exit", WIDTH, HEIGHT, winopts).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    println!("Starting");
    while window.is_open() && !window.is_key_down(Key::Escape) && !cpu.halted() {
        let redraw = cpu.step();

        if redraw {
            window
                .update_with_buffer(cpu.disp(), WIDTH, HEIGHT)
                .unwrap();
        }
    }
}
