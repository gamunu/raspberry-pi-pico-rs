#![no_std]
#![no_main]

// For string formatting.
use core::fmt::Write;

// The macro for our start-up function
use rp_pico::entry;

// Time handling traits:
use fugit::{ExtU32, RateExtU32};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;
// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// CountDown timer for the counter on the display:
use embedded_hal::timer::CountDown;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

// The display driver:
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals,
/// gets a handle on the I2C peripheral,
/// initializes the SSD1306 driver, initializes the text builder
/// and then draws some text on the display.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut peripherals = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(peripherals.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(peripherals.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    // Configure two pins as being I²C, not GPIO
    let sda_pin = pins.gpio16.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio17.into_mode::<hal::gpio::FunctionI2C>();

    // Create the I²C driver, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = hal::I2C::i2c0(
        peripherals.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut peripherals.RESETS,
        &clocks.peripheral_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let timer = hal::Timer::new(peripherals.TIMER, &mut peripherals.RESETS);
    let mut delay = timer.count_down();

    let mut count = 0;

    let mut buf = FmtBuf::new();
    loop {
        buf.reset();
        // Format some text into a static buffer:
        write!(&mut buf, "counter: {}", count).unwrap();
        count += 1;

        // Empty the display:
        display.clear();

        // Draw 3 lines of text:
        Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline(buf.as_str(), Point::new(0, 32), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        // Wait a bit:
        delay.start(500.millis());
        let _ = nb::block!(delay.wait());
    }
}
/// This is a very simple buffer to pre format a short line of text
/// limited arbitrarily to 64 bytes.
struct FmtBuf {
    buf: [u8; 64],
    ptr: usize,
}

impl FmtBuf {
    fn new() -> Self {
        Self {
            buf: [0; 64],
            ptr: 0,
        }
    }

    fn reset(&mut self) {
        self.ptr = 0;
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[0..self.ptr]).unwrap()
    }
}

impl core::fmt::Write for FmtBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let rest_len = self.buf.len() - self.ptr;
        let len = if rest_len < s.len() {
            rest_len
        } else {
            s.len()
        };
        self.buf[self.ptr..(self.ptr + len)].copy_from_slice(&s.as_bytes()[0..len]);
        self.ptr += len;
        Ok(())
    }
}
