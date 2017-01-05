#![no_std]
#![feature(link_args)]

#[link_args = "-Wl,--gc-sections -Wl,--sort-section=alignment -mcpu=cortex-m0 -mthumb -T NRF51822.ld startup_NRF51822.S"]
extern { }

#[macro_use]
extern crate nrf51_lang;
extern crate nrf51_hardware;

use nrf51_hardware::{busy_loop, gpio, pins};

const PERIOD_MS: u32 = 1000;
const ON_MS: u32 = 50;

fn main() {
    let rows = [gpio::Pin::output(pins::ROW_1),
                gpio::Pin::output(pins::ROW_2),
                gpio::Pin::output(pins::ROW_3)];
    let cols = [gpio::Pin::output(pins::COL_1),
                gpio::Pin::output(pins::COL_2),
                gpio::Pin::output(pins::COL_3),
                gpio::Pin::output(pins::COL_4),
                gpio::Pin::output(pins::COL_5),
                gpio::Pin::output(pins::COL_6),
                gpio::Pin::output(pins::COL_7),
                gpio::Pin::output(pins::COL_8),
                gpio::Pin::output(pins::COL_9)];

    for row in &rows {
        row.set_low();
    }
    for col in &cols {
        col.set_high();
    }

    let pin_indicies = [(1, 1), (2, 4), (1, 2), (2, 5), (1, 3), (3, 4), (3, 5), (3, 6), (3, 7),
                        (3, 8), (2, 2), (1, 9), (2, 3), (3, 9), (2, 1), (1, 8), (1, 7), (1, 6),
                        (1, 5), (1, 4), (3, 3), (2, 7), (3, 1), (2, 6), (3, 2)];
    let pins = pin_indicies.iter().map(|&(row, col)| (&rows[row - 1], &cols[col - 1]));

    for (uptime, (ref row, ref col)) in pins.enumerate() {
        println!("Uptime: {}", uptime);
        row.set_high();
        col.set_low();
        busy_loop::wait_approx_ms(ON_MS);
        row.set_low();
        col.set_high();
        busy_loop::wait_approx_ms(PERIOD_MS - ON_MS);
    }
}
