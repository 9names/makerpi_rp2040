//! # PWM Buzzer Example
//!
//! Cycles through frequencies on the onboard buzzer using the PWM peripheral.
//!
//! See the `Cargo.toml` file for Copyright and licence details.

#![no_std]
#![no_main]

// The macro for our start-up function
use cortex_m_rt::entry;

// GPIO traits
use embedded_hal::PwmPin;

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

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then fades the LED in an
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

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM3
    let pwm = &mut pwm_slices.pwm3;
    pwm.set_ph_correct();
    pwm.enable();

    // Output channel A on PWM3 to the buzzerLED pin
    let channel = &mut pwm.channel_a;
    channel.output_to(pins.buzzer);

    // This adjusts the volume of the buzzer.
    // It's quiet by default - make this larger and it will get quite loud
    channel.set_duty(100);

    const MIN_PERIOD: u16 = 10000;
    const MAX_PERIOD: u16 = u16::MAX;
    // Infinite loop, buzzer pitch moving down then up
    // set_top adjusts the period, so low = high frequency
    // lower top = higher frequency
    loop {
        // Ramp frequency down
        for i in MIN_PERIOD..=MAX_PERIOD {
            delay.delay_us(80);
            pwm.set_top(i);
        }

        delay.delay_ms(500);

        // Ramp frequency up
        for i in (MIN_PERIOD..=MAX_PERIOD).rev() {
            delay.delay_us(80);
            pwm.set_top(i);
        }
    }
}

// End of file
