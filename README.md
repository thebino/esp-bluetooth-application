# Bare-metal ESP Bluetooth Application

[![CI](https://github.com/thebino/esp-bluetooth-application/actions/workflows/rust_ci.yml/badge.svg)](https://github.com/thebino/esp-bluetooth-application/actions/workflows/rust_ci.yml)

An embedded application written in 🦀Rust using `no_std` bare-metal.
The application is targeting an [ESP32S3](https://www.espressif.com/en/products/socs/esp32-s3) from [Espressif](https://www.espressif.com).


## Prerequisites

You'll need:
 - ESP32S3 development board
 - `espup` - a tool to install the ESP32 toolchain and other tools
 - `cargo-espflash` - a tool to flash the ESP32

```
cargo install espup cargo-espflash
espup install
```

## Quick start

Check the connected device
```shell
espflash board-info
❯ /dev/cu.usbmodem101 - USB JTAG_serial debug unit
Chip type:         esp32s3 (revision v0.1)
Crystal frequency: 40 MHz
Flash size:        8MB
Features:          WiFi, BLE
MAC address:       7c:df:a1:f5:62:8c
```

Delete previous installed applications from the esp32
```shell
espflash erase-flash
```

Prepare a custom partition table
```shell
espflash partition-table --to-binary --output partition-table.bin partition-table.csv
```

Build the project
```shell
cargo build
```

Flash the project by adding the custom partition table as binary
```shell
espflash flash --partition-table partition-table.bin --monitor target/xtensa-esp32s3-none-elf/debug/esp-bluetooth-application
```


## Initial setup
Erase flash to delete all previous data from the device

- connect device
- press and hold BOOT
- espflash erase-flash

	

