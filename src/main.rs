use std::fs::File;
use std::io::Read;
mod emulator;

fn main() {
    let mut emulator = emulator::Emulator::new();

    let mut file =
        File::open("test_roms/blargg_nes_cpu_test5/official.nes").expect("Failed to open ROM file");
    let mut rom_buffer = Vec::new();
    file.read_to_end(&mut rom_buffer)
        .expect("Failed to read ROM data");

    // Skip the 16-byte iNES header and pass only the actual ROM data
    // will need to implement this later to read the cartridge header
    if rom_buffer.len() > 16 {
        emulator.bus.load_rom(&rom_buffer[16..]);
    } else {
        panic!("ROM file is too small to contain a valid iNES header!");
    }

    emulator.cpu.reset(&emulator.bus);

    println!("Starting Emulator");
    let mut running = true;
    while running {
        if emulator.cpu.nmi {
            emulator.cpu.nmi = false;
            emulator.cpu.handle_interrupts(&mut emulator.bus, 0);
            continue;
        }

        if emulator.cpu.irq && !emulator.cpu.registers.interrupt_disable {
            emulator.cpu.handle_interrupts(&mut emulator.bus, 1);
            continue;
        }

        let opcode = emulator.cpu.fetch(&mut emulator.bus);

        let status = emulator.bus.read(0x6000);
        if status != 0x80 && status != 0x81 {
            let mut msg = String::new();
            let mut addr = 0x6004;
            loop {
                let b = emulator.bus.read(addr);
                if b == 0 {
                    break;
                }
                msg.push(b as char);
                addr += 1;
            }
            println!("Test finished with status: {:#04X}\n{}", status, msg);
            break;
        }

        emulator.cpu.execute(opcode, &mut emulator.bus);
    }
}
