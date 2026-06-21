# Arduino Uno R3 Blinky Example

This example demonstrates how to use the `wrtos` Time-Triggered Cyclic Executive RTOS on the **Elegoo/Arduino Uno R3** (ATmega328P). 

It features a custom **Timer HAL** (Timer 1) and a **Board Support Package (BSP)** for the Uno R3, operating entirely without interrupts.

---

## 1. Architecture

The example is split into two components:
1. **`uno-bsp`** (Library): Exposes the Board Support Package.
   - `registers`: The `AvrRegisters` trait, defining ATmega328P registers.
   - `timer`: The timer driver implementing `wrtos_core::Timer` using 16-bit Timer 1 in CTC mode.
   - `led`: The LED controller for Pin 13 (PB5).
   - `serial`: A placeholder polling serial driver.
2. **`uno-blinky`** (Binary): The cyclic executive application.
   - `schedule.toml`: Configures a 10 Hz frame rate and schedules `blink_led` at Frame 0 and Frame 5.
   - `src/main.rs`: The entry point (supporting both host simulation and AVR bare metal).

### Portability & The Registers Trait
To avoid external dependencies (`avr-gcc`, `avrdude`, target spec files) and ensure **100% unit test coverage**, all hardware modules are generic over the `AvrRegisters` trait:
- **`RealRegisters`**: Used when compiled for the actual AVR microcontroller. Performs volatile memory-mapped register writes/reads.
- **`MockRegisters`**: Used under testing and host simulation. Keeps register states in an `Rc<RefCell<...>>` structure so all simulated hardware components write to and read from a shared logical memory block.

---

## 2. Host Simulation & Testing (Zero Dependencies)

You can build, test, and run this entire blinky example directly on your host machine (Windows/macOS/Linux) without installing any AVR compilers or hardware tools.

### Running the Live Simulation
To run a live interactive simulation of the blinky program on your host command line:
```bash
cargo run --bin uno-blinky
```
This runs the cyclic executive step-by-step using simulated registers and prints the LED's state changes:
```text
=== Elegoo Arduino Uno R3 Blinky RTOS Simulation (Host Mode) ===
Running the cyclic executive on the host using MockRegisters...
Tick Rate: 10 Hz (100 ms frames)
Major Cycle: 10 frames (1.0 second)

Starting simulation for 2 major cycles (20 frames)...
Frame 00: LED state toggled -> ON  [🟢]
Frame 01: (running, LED remains ON)
...
Frame 05: LED state toggled -> OFF [🔴]
```

### Running the Unit Tests (TDD Verification)
To execute all the BSP and application unit tests:
```bash
cargo test -p uno-bsp
cargo test -p uno-blinky
```

---

## 3. Loading Onto the Arduino Uno R3

If you wish to load the program onto a physical Arduino Uno board, follow the steps below.

### Prerequisites
1. **Rust Nightly Toolchain**: Cross-compilation for AVR (a Tier 3 target) requires the unstable standard library build feature (`-Z build-std`).
   ```bash
   rustup toolchain install nightly
   rustup +nightly target add avr-specs
   ```
2. **AVR Linker & Flashing Tool**:
   - Install `avr-gcc` (contains the AVR linker) and `avrdude` (flashing utility).
   - Ensure both are added to your system `PATH`.

### Step 1: Compile the Code
Build the binary for the ATmega328P target:
```bash
cargo +nightly build --target avr-atmega328p -Z build-std -p uno-blinky --release
```

### Step 2: Convert to Hex Format
Convert the compiled `.elf` binary to an Intel HEX file ready for flashing:
```bash
avr-objcopy -O ihex -R .eeprom target/avr-atmega328p/release/uno-blinky target/avr-atmega328p/release/uno-blinky.hex
```

### Step 3: Flash the Board
Connect your Arduino Uno via USB, identify its COM port (e.g., `COM3` on Windows, `/dev/ttyACM0` on Linux), and flash using `avrdude`:
```bash
avrdude -F -V -c arduino -p m328p -P <PORT> -b 115200 -U flash:w:target/avr-atmega328p/release/uno-blinky.hex:i
```
Replace `<PORT>` with your actual serial port name.
