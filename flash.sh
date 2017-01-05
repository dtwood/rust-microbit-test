#!/bin/sh

set -e
cd $(dirname $0)
xargo build --target=cortex-m0 --release
arm-none-eabi-objcopy -O ihex target/cortex-m0/release/microbit-demo target/cortex-m0/release/microbit-demo.hex
srec_cat \
    BLE_BOOTLOADER_RESERVED.hex -intel \
    s110_nrf51822_8.0.0_softdevice.hex -intel \
    target/cortex-m0/release/microbit-demo.hex -intel \
    -o target/cortex-m0/release/combined.hex -intel
echo Flashing…
cp target/cortex-m0/release/combined.hex /Volumes/MICROBIT/
