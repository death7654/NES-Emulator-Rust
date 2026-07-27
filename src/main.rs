use std::fs::File;
use std::io::Read;
mod emulator;

use crate::emulator::display::Display;
use sdl2::event::Event;

fn main() {
    let mut emulator = emulator::Emulator::new();
    let mut event_pump = emulator.display.sdl.event_pump().unwrap();

    let mut file = File::open("games/zelda.nes").expect("Failed to open ROM file");
    let mut rom_buffer = Vec::new();
    file.read_to_end(&mut rom_buffer)
        .expect("Failed to read ROM data");

    if rom_buffer.len() > 16 {
        emulator.bus.load_rom(&rom_buffer);
    } else {
        panic!("ROM file is too small to contain a valid iNES header!");
    }

    emulator.cpu.reset(&mut emulator.bus);

    println!("Starting Emulator");
    let mut running = true;

    let mut canvas = emulator.display.canvas;
    let texture_creator = emulator.display.texture_creator;
    let mut frame_texture = Display::create_frame_texture(&texture_creator);

    while running {
        // poll events every iteration, regardless of frame timing
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    emulator.bus.input.press(key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    emulator.bus.input.release(key);
                }
                _ => {}
            }
        }

        if emulator.bus.poll_nmi() {
            emulator.cpu.handle_interrupts(&mut emulator.bus, 0);
            continue;
        }

        if emulator.cpu.irq && !emulator.cpu.registers.interrupt_disable {
            emulator.cpu.handle_interrupts(&mut emulator.bus, 1);
            continue;
        }

        let opcode = emulator.cpu.fetch(&mut emulator.bus);
        emulator.cpu.execute(opcode, &mut emulator.bus);

        if emulator.bus.ppu.frame_complete {
            emulator.bus.ppu.frame_complete = false;

            let mut distinct = std::collections::HashSet::new();
            for &px in emulator.bus.ppu.frame_buffer.iter() {
                distinct.insert(px);
            }

            Display::render_frame(
                &mut canvas,
                &mut frame_texture,
                &emulator.bus.ppu.frame_buffer,
            );
        }
    }
}
