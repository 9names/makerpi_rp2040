# Board Support for the [Cytron Maker Pi RP2040]

You should include this crate if you are writing code that you want to run on
a `Cytron Maker Pi RP2040`.

This crate includes the [rp2040-hal](https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal), but also configures each pin of the
RP2040 chip according to how it is connected up on the Maker Pi RP2040.

## Cytron Maker Pi RP2040

[Github page](https://github.com/CytronTechnologies/MAKER-PI-RP2040)


[Store](https://www.cytron.io/p-maker-pi-rp2040-simplifying-robotics-with-raspberry-pi-rp2040)

[Schematic](https://drive.google.com/file/d/1Zp8GYO8x7ThObB1G8RIZx2YdqrXtdUc0/view)

[rp2040-hal](https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal)

[Raspberry Silicon RP2040](https://www.raspberrypi.org/products/rp2040/)

## Using

To use this crate, your `Cargo.toml` file should contain:

```toml
cytron_maker_pi_rp2040 = "0.1.0"
```

In your program, you will need to call `cytron_maker_pi_rp2040::Pins::new` to create
a new `Pins` structure. This will set up all the GPIOs for any on-board
devices. See the [examples](./examples) folder for more details.

## Examples

### General Instructions

To compile an example, clone this repository and run:

```console
makepi_rp2040 $ cargo build --release --example <name>
```

You will get an ELF file called
`./target/thumbv6m-none-eabi/release/examples/<name>`, where the `target`
folder is located at the top of the _rp-hal_ repository checkout. Normally
you would also need to specify `--target=thumbv6m-none-eabi` but when
building examples from this git repository, that is set as the default.

If you want to convert the ELF file to a UF2 and automatically copy it to the
USB drive exported by the RP2040 bootloader, simply boot your board into
bootloader mode and run:

```console
rp-hal/boards/pico $ cargo run --release --example <name>
```

If you get an error about not being able to find `elf2uf2-rs`, try:

```console
$ cargo install elf2uf2-rs
```
then try repeating the `cargo run` command above.

### [cycle_leds](./examples/cycle_leds.rs)

Flashes a sequences across the Digital IO Status LEDs

### [pwm_buzzer](./examples/pwm_buzzer.rs)

Plays a sweeping frequency pitch through the on-board buzzer

### [stepper_motor](./examples/stepper_motor.rs)

Rotates a stepper motor through 360 degrees clockwise then anticlockwise.
Note that this requires a specific stepper motor from 
[Seeedstudio](https://www.seeedstudio.com/Small-Size-and-High-Torque-Stepper-Motor-24BYJ48-p-1922.html)

### [rgb_leds](./examples/rgb_leds.rs)

Cycle through colors on the pair of onboard RGB LEDs



`SPDX-License-Identifier: Apache-2.0 OR MIT`