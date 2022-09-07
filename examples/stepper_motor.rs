//! # Stepper motor example
//!
//! Rotate a stepper motor
//!
//! This will rotate a two-phase, 4 wire stepper motor
//! attached to Motor1 and Motor 2 screw terminals
//!
//! Note that if you halt the processor, you may leave a GPIO high and burn out your motor.
//! Hold the boot button and press reset if any light is still remaining to avoid this.
//! Treat this as a friendly reminder to use the watchdog for any unattended installation to avoid fires.
//!
//! See the `Cargo.toml` file for Copyright and licence details.

#![no_std]
#![no_main]

// The macro for our start-up function
use cortex_m_rt::entry;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;

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
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set the LED to be an output
    let mut motor1_a = pins.motor_1_a.into_push_pull_output();
    let mut motor1_b = pins.motor_1_b.into_push_pull_output();
    let mut motor2_a = pins.motor_2_a.into_push_pull_output();
    let mut motor2_b = pins.motor_2_b.into_push_pull_output();
    motor1_a.set_low().unwrap();
    motor1_b.set_low().unwrap();
    motor2_a.set_low().unwrap();
    motor2_b.set_low().unwrap();

    // How long between changing phases (do not go less than 4)
    const DELAY: u32 = 10;
    const STEPS_PER_CYCLE: usize = 256;

    for _ in 0..STEPS_PER_CYCLE {
        // step clockwise
        motor1_a.set_high().unwrap();
        delay.delay_ms(DELAY);
        motor1_a.set_low().unwrap();

        motor2_a.set_high().unwrap();
        delay.delay_ms(DELAY);
        motor2_a.set_low().unwrap();

        motor1_b.set_high().unwrap();
        delay.delay_ms(DELAY);
        motor1_b.set_low().unwrap();

        motor2_b.set_high().unwrap();
        delay.delay_ms(DELAY);
        motor2_b.set_low().unwrap();
    }

    for _ in 0..STEPS_PER_CYCLE {
        // step counter-clockwise
        motor2_a.set_high().unwrap();
        delay.delay_ms(DELAY);
        motor2_a.set_low().unwrap();

        motor1_a.set_high().unwrap();
        delay.delay_ms(DELAY);
        motor1_a.set_low().unwrap();

        motor2_b.set_high().unwrap();
        delay.delay_ms(DELAY);
        motor2_b.set_low().unwrap();

        motor1_b.set_high().unwrap();
        delay.delay_ms(DELAY);
        motor1_b.set_low().unwrap();
    }

    // Motor demo done, blink an LED now.
    // These little motors are quite strong, but are not designed to be
    // left running for long periods of time. A quick demo is best.
    let mut led = pins.grove_1_a.into_push_pull_output();
    loop {
        led.set_high().unwrap();
        delay.delay_ms(50);
        led.set_low().unwrap();
        delay.delay_ms(50);
    }
}

// End of file
