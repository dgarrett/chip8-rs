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
        // default_value_t = ("".to_string())
    )]
    rom: String,

    /// Verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn load_rom(path: &str, cpu: &mut CPU) -> std::io::Result<()> {
    println!("Loading {path}");
    let rom = fs::read(path)?;
    cpu.load(&rom);
    Ok(())
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut cpu = CPU::new();
    load_rom(&args.rom, &mut cpu).expect("Failed to load");

    let mut winopts = WindowOptions::default();
    winopts.scale = Scale::X16;

    let mut window =
        Window::new("chip8-rs - ESC to exit", WIDTH, HEIGHT, winopts).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let keys: [Key; 16] = [
        Key::X,
        Key::Key1,
        Key::Key2,
        Key::Key3,
        Key::Q,
        Key::W,
        Key::E,
        Key::A,
        Key::S,
        Key::D,
        Key::Z,
        Key::C,
        Key::Key4,
        Key::R,
        Key::F,
        Key::V,
    ];

    println!("Starting");
    let mut iter_sinc_redraw = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) && !cpu.halted() {
        for (i, key) in keys.into_iter().enumerate() {
            // println!("Key {i} {key:?} state {}", window.is_key_down(key));
            cpu.key_state(i as u8, window.is_key_down(key));
        }
        cpu.delay_timer_tick();
        let redraw = cpu.step();
        iter_sinc_redraw += 1;

        if redraw || iter_sinc_redraw > 60 {
            window
                .update_with_buffer(cpu.disp(), WIDTH, HEIGHT)
                .unwrap();
            iter_sinc_redraw = 0;
        }
    }
}
