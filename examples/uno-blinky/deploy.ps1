# Windows PowerShell Deployment Script for wrtos Blinky
$ErrorActionPreference = "Stop"

Write-Host "=== Step 1: Running Host Unit Tests ===" -ForegroundColor Yellow
cargo test -p uno-bsp
cargo test -p uno-blinky
Write-Host "[OK] Host unit tests passed successfully!" -ForegroundColor Green

Write-Host "=== Step 2: Compiling for AVR target (ATmega328P) ===" -ForegroundColor Yellow
# Ensure paths are refreshed so we can find avr-gcc
$env:Path = [System.Environment]::GetEnvironmentVariable('Path','Machine') + ';' + [System.Environment]::GetEnvironmentVariable('Path','User')
$env:RUSTFLAGS = "-C target-cpu=atmega328p -C panic=abort"
cargo +nightly build --target avr-none -Z build-std -p uno-blinky --release
Write-Host "[OK] Target compilation succeeded!" -ForegroundColor Green

Write-Host "=== Step 3: Converting ELF to Intel HEX format ===" -ForegroundColor Yellow
avr-objcopy -O ihex -R .eeprom target/avr-none/release/uno-blinky.elf target/avr-none/release/uno-blinky.hex
Write-Host "[OK] HEX conversion completed!" -ForegroundColor Green

Write-Host "=== Step 4: Detecting Connected Arduino Uno Port ===" -ForegroundColor Yellow
$port = Get-CimInstance -Class Win32_PnPEntity | Where-Object { $_.Caption -like "*USB Serial Device*COM*" -or $_.Caption -like "*Arduino*COM*" } | ForEach-Object {
    if ($_.Caption -match '\((COM\d+)\)') { $Matches[1] }
} | Select-Object -First 1

if (-not $port) {
    # Fallback to any COM port
    $port = [System.IO.Ports.SerialPort]::GetPortNames() | Select-Object -First 1
}

if (-not $port) {
    Write-Error "No connected Arduino Uno or COM port detected. Please plug in the board."
    exit 1
}
Write-Host "[OK] Found Arduino Uno on port: $port" -ForegroundColor Green

Write-Host "=== Step 5: Locating avrdude.conf ===" -ForegroundColor Yellow
$packages_dir = "$env:LocalAppdata\Microsoft\WinGet\Packages"
$avr_gcc_dir = Get-ChildItem -Path $packages_dir -Filter "ZakKemble.avr-gcc*" | Select-Object -First 1 -ExpandProperty FullName
if (-not $avr_gcc_dir) {
    Write-Error "Could not find ZakKemble.avr-gcc in $packages_dir"
    exit 1
}
$avrdude_conf = Join-Path $avr_gcc_dir "avr-gcc-14.1.0-x64-windows\bin\avrdude.conf"
if (-not (Test-Path $avrdude_conf)) {
    Write-Error "avrdude.conf not found at $avrdude_conf"
    exit 1
}
Write-Host "[OK] Found avrdude.conf at: $avrdude_conf" -ForegroundColor Green

Write-Host "=== Step 6: Flashing the Board via avrdude ===" -ForegroundColor Yellow
avrdude -C $avrdude_conf -F -V -c arduino -p m328p -P $port -b 115200 -U flash:w:target/avr-none/release/uno-blinky.hex:i
Write-Host "[OK] Successfully built, tested, and deployed to Arduino Uno!" -ForegroundColor Green
