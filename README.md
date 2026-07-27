# NES-Emulator-Rust

A NES emulator written from scratch in Rust, using SDL2 for windowing and video output.

# Project Screenshots
<img width="400" alt="image" src="https://github.com/user-attachments/assets/c221017d-a800-4a15-ba88-5ba1d49c951d" />
<img width="400" alt="image" src="https://github.com/user-attachments/assets/f2d750a8-b253-4ec2-a6c5-bd207e4e0b08" />
<img width="400" alt="image" src="https://github.com/user-attachments/assets/3d746864-f370-4205-8232-cc03d2186f26" />
<img width="400" alt="image" src="https://github.com/user-attachments/assets/836e818d-499e-4787-81c7-0718b7c4fcee" />


## Features

- CPU: full set of official 6502 opcodes, decoded using the cc/bbb/aaa bit-field pattern instead of a flat 256-entry table. Cycle-accurate ticking, including page-crossing penalties, branch timing, and OAM DMA CPU stalls (513/514 cycles).
- PPU: scanline/dot-accurate rendering — background tile fetching with the standard 8-cycle fetch pattern, sprite evaluation (secondary OAM, 8-sprites-per-scanline limit, overflow flag), 8x8 and 8x16 sprite modes, sprite-0 hit detection, background/sprite priority compositing.
- Mappers: NROM (Mapper 0) and MMC1 (Mapper 1), including MMC1's dynamic mirroring control.
- Input: NES controller emulated as an actual serial shift register (strobe + 8 sequential single-bit reads), not just a flat button byte.
- Display: renders to an SDL2 window in real time, scaled 3x from the native 256x240 resolution.

## Requirements

- Rust (stable toolchain)
- CMake, needed to build sdl2-sys from source via the bundled feature
- A C/C++ build toolchain (MSVC Build Tools on Windows, build-essential on Linux)

SDL2 doesn't need to be installed separately, it gets compiled from source as part of the build.

## Build instructions

Recent CMake versions (4.x+) dropped support for the older cmake_minimum_required version SDL2's bundled build script declares. Until that's updated upstream, set this once per shell session before building:

PowerShell (Windows):
```
$env:CMAKE_POLICY_VERSION_MINIMUM = "3.5"
```

bash/zsh (Linux/macOS):
```
export CMAKE_POLICY_VERSION_MINIMUM=3.5
```

To avoid setting this every session, set it permanently for your user account instead:

```
[System.Environment]::SetEnvironmentVariable("CMAKE_POLICY_VERSION_MINIMUM", "3.5", "User")
```

Then build and run:

```
cargo run
```

The first build takes a while since it's compiling SDL2 from source. Later builds reuse the cached output.

The CPU interpreter runs a lot faster with optimizations on, so it's worth tuning the dev profile instead of building in full release mode:

```
# Cargo.toml
[profile.dev]
opt-level = 2
```

## Running a ROM

Place a .nes ROM at games/mario.nes relative to the project root (make a games/ folder next to Cargo.toml if it doesn't exist). This path is currently hardcoded in main.rs, change it there to point at a different ROM.

Only iNES-format ROMs are supported, and only Mapper 0 (NROM), Mapper 1 (MMC1), and Mapper 3 are implemented. ROMs using other mappers will fail to load.

## Controls

NES Button - Keyboard
D-Pad - Arrow keys or WASD
A - K or X
B - J or Z
Select - Shift or Tab
Start - Enter or Space

## Known limitations

- No audio (APU) emulation
- Only official 6502 opcodes are implemented, no illegal/undocumented opcode support
- Only Mapper 0 and Mapper 1 are implemented, other mappers fail with an "Unsupported Mapper ID" error
- No save states or battery-backed save RAM
- ROM path is hardcoded rather than passed in as an argument
S
