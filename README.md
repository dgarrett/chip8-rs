# chip8-rs

This is a continuation of the CHIP-8 project from Rust in Action by Tim
McNamara.

The project uses minifb for cross-platform windowing and rodio for
cross-platform audio.

All CHIP-8 opcodes are implemented and a few ROMs seem to work well, but I
haven't done any investigations on performance.

This is just an educational exercise and likely won't be continued, though I'd
like to get it working in webassembly at some point.

## Instructions
- `cargo run <path to ROM file>`