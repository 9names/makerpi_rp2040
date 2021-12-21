//! # LED cycle Example
//!
//! Flashes a sequences across the Digital IO Status LEDs
//!
//!
//! See the `Cargo.toml` file for Copyright and licence details.

#![no_std]
#![no_main]

// The macro for our start-up function
use cortex_m_rt::entry;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;

// Time handling traits
use embedded_time::rate::*;

use hal::gpio::DynPin;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Make an alias for our board support package so copying examples to other boards is easier
use cytron_maker_pi_rp2040 as bsp;

// Pull in any important traits
use bsp::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use bsp::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use bsp::hal;

//// The linker will place this boot block at the start of our program image. We
//// need this to help the ROM bootloader get our code up and running.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // We gave the pins on this board nice names, but we're just using them for their LEDs
    // To put them in an array we have to convert them to DynPins. Let's do that now...
    let mut pinarray: [DynPin; 13] = [
        pins.grove_1_a.into_push_pull_output().into(),
        pins.grove_1_b.into_push_pull_output().into(),
        pins.grove_2_a.into_push_pull_output().into(),
        pins.grove_2_b.into_push_pull_output().into(),
        pins.grove_3_a.into_push_pull_output().into(),
        pins.grove_3_b.into_push_pull_output().into(),
        pins.grove_5_a.into_push_pull_output().into(),
        pins.grove_7_a.into_push_pull_output().into(),
        pins.grove_4_a.into_push_pull_output().into(),
        pins.grove_4_b.into_push_pull_output().into(),
        pins.grove_5_b.into_push_pull_output().into(),
        pins.grove_6_b.into_push_pull_output().into(),
        pins.grove_7_b.into_push_pull_output().into(),
    ];

    // Light one LED at a time. Start at GP0 and go through to GP28, then reverse.
    loop {
        for led in pinarray.iter_mut() {
            led.set_high().unwrap();
            delay.delay_ms(50);
            led.set_low().unwrap();
        }
        for led in pinarray.iter_mut().rev() {
            led.set_high().unwrap();
            delay.delay_ms(50);
            led.set_low().unwrap();
        }
    }
}
// End of file
