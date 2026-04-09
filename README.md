# LED 3-Mode Controller

Firmware for YD-RP2040 that cycles through 3 LED modes using the onboard button. Demonstrates GPIO control, debouncing, and state management in Rust.

## Features

- 3 LED modes controlled by onboard button
- Proper button debouncing
- No external components required
- Low-level Rust firmware (`no_std`)

## LED Modes

| Mode | Behavior | Description |
|------|----------|-------------|
| **0** | OFF | LED is completely off |
| **1** | ON | LED stays on continuously |
| **2** | BLINK | LED blinks rapidly (50ms on / 50ms off) |

Press the **USER** button to cycle: OFF → ON → BLINK → OFF → ...

## Hardware

- **Board:** YD-RP2040
- **Components:** All onboard (no external wiring needed!)
  - Onboard blue LED (GPIO25)
  - Onboard USER button

## How to Build

1. Install Rust and the ARM Cortex-M0+ target:
```bash
rustup target add thumbv6m-none-eabi
```

2. Build the project:
```bash
cargo build --release
```

3. Convert ELF to UF2:
```bash
cargo install elf2uf2-rs
elf2uf2-rs target/thumbv6m-none-eabi/release/hello_rust hello_rust.uf2
```

## How to Flash

1. Hold **BOOT** button on YD-RP2040
2. Press **RESET** button (or connect USB while holding BOOT)
3. Board appears as USB drive **RPI-RP2**
4. Copy **`hello_rust.uf2`** to the drive
5. Board will reboot automatically and start running

## Built With

- **Language:** Rust (embedded `no_std`)
- **HAL:** [rp2040-hal](https://github.com/rp-rs/rp-hal)
- **Board crate:** [vcc-gnd-yd-rp2040](https://github.com/rp-rs/rp-hal-boards/tree/main/boards/vcc-gnd-yd-rp2040)
- **Panic handler:** `panic-halt`

## How it Works

The firmware monitors the USER button state and cycles through 3 modes on each press:

1. **Button Debouncing:** Compares current and previous button states to detect a clean press (transition from released to pressed)
2. **Mode Switching:** Increments mode counter (0 → 1 → 2 → 0) on each button press
3. **LED Control:**
   - Mode 0: Sets GPIO25 LOW (LED off)
   - Mode 1: Sets GPIO25 HIGH (LED on)
   - Mode 2: Toggles GPIO25 with 50ms delays (fast blinking)

## Project Structure
