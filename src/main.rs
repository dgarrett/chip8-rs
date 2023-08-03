mod cpu;

use clap::Parser;
use log::debug;
use minifb::{Key, Scale, Window, WindowOptions};
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::fs;
use std::time::Duration;

use crate::cpu::{CPU, HEIGHT, WIDTH};

const WINDOW_TITLE: &str = "chip8-rs - ESC to exit";
const WINDOW_TITLE_PAUSED: &str = "chip8-rs (PAUSED) - ESC to exit";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ROM file path
    #[arg(
        // default_value_t = ("".to_string())
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
    env_logger::init();
    let args = Args::parse();

    let mut cpu = CPU::new();
    let mut cpu_snapshot: Option<CPU> = None;
    load_rom(&args.rom, &mut cpu).expect("Failed to load");

    let mut winopts = WindowOptions::default();
    winopts.scale = Scale::X16;

    let mut window = Window::new(WINDOW_TITLE, WIDTH, HEIGHT, winopts).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // ~15 FPS
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600 * 4)));

    const KEYS: [Key; 16] = [
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

    let mut pause = false;

    // Sound setup
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = SineWave::new(340.0)
        .take_duration(Duration::from_secs_f32(0.1))
        .amplify(0.30);

    println!("Starting");
    while window.is_open() && !window.is_key_down(Key::Escape) && !cpu.halted() {
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            println!(
                "Space pressed. {}",
                if pause { "Unpausing" } else { "Pausing" }
            );
            pause = !pause;
            window.set_title(if pause {
                WINDOW_TITLE_PAUSED
            } else {
                WINDOW_TITLE
            })
        }

        if window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) {
            println!("Snapshotting");
            cpu_snapshot = Some(cpu.clone());
        } else if window.is_key_pressed(Key::Key9, minifb::KeyRepeat::No) {
            if let Some(cpu_snapshot) = &cpu_snapshot {
                println!("Restoring");
                cpu = cpu_snapshot.clone();
            } else {
                println!("No snapshot to restore");
            }
        }

        if !pause {
            for (i, key) in KEYS.into_iter().enumerate() {
                debug!("Key {i} {key:?} state {}", window.is_key_down(key));
                cpu.key_state(i as u8, window.is_key_down(key));
            }

            // 240Hz
            for _ in 0..16 {
                cpu.delay_timer_tick();

                if cpu.sound_timer_tick() {
                    sink.append(source.clone());
                }

                cpu.step();
            }
        }

        window
            .update_with_buffer(cpu.disp(), WIDTH, HEIGHT)
            .unwrap();
    }
}
