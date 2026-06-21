#!/usr/bin/env bash
set -euo pipefail

# Script to build, test, and deploy the wrtos blinky example to Arduino Uno R3.

# Color helpers
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Step 1: Running Host Unit Tests ===${NC}"
if cargo test -p uno-bsp && cargo test -p uno-blinky; then
    echo -e "${GREEN}✔ Host unit tests passed successfully!${NC}\n"
else
    echo -e "${RED}✘ Host unit tests failed. Aborting deployment.${NC}"
    exit 1
fi

echo -e "${YELLOW}=== Step 2: Compiling for AVR target (ATmega328P) ===${NC}"
RUSTFLAGS="-C target-cpu=atmega328p -C panic=abort" cargo +nightly build \
    --target avr-none \
    -Z build-std \
    -p uno-blinky \
    --release

echo -e "${GREEN}✔ Target compilation succeeded!${NC}\n"

echo -e "${YELLOW}=== Step 3: Converting ELF to Intel HEX format ===${NC}"
avr-objcopy -O ihex -R .eeprom \
    target/avr-none/release/uno-blinky.elf \
    target/avr-none/release/uno-blinky.hex

echo -e "${GREEN}✔ HEX conversion completed!${NC}\n"

echo -e "${YELLOW}=== Step 4: Detecting Connected Arduino Uno Port ===${NC}"
# Automatically find serial port (supports ttyACM* and ttyUSB*)
PORT=""
for p in /dev/ttyACM* /dev/ttyUSB*; do
    if [ -e "$p" ]; then
        PORT="$p"
        break
    fi
done

if [ -n "$PORT" ]; then
    echo -e "${GREEN}✔ Found Arduino Uno on port: ${PORT}${NC}\n"
else
    echo -e "${RED}✘ No connected Arduino Uno detected (checked /dev/ttyACM* and /dev/ttyUSB*).${NC}"
    echo -e "${YELLOW}Please connect the board via USB and try again.${NC}"
    exit 1
fi

echo -e "${YELLOW}=== Step 5: Flashing the Board via avrdude ===${NC}"
avrdude -F -V -c arduino -p m328p -P "$PORT" -b 115200 -U flash:w:target/avr-none/release/uno-blinky.hex:i

echo -e "\n${GREEN}✔ Successfully built, tested, and deployed to Arduino Uno!${NC}"
