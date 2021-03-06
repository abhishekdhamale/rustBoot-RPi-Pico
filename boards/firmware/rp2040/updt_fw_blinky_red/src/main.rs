//! Blinks the LED on a Pico board
#![no_std]
#![no_main]

// #[cfg(feature = "defmt")]
// use defmt_rtt as _; // global logger
// use panic_probe as _; // global logger

// use defmt::*;
use embedded_hal::digital::v2::OutputPin;
// use embedded_time::fixed_point::FixedPoint;

use cortex_m_rt::entry;
use cortex_m::asm;

use rp_pico as bsp;

use bsp::hal::{
    // clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    // watchdog::Watchdog,
};

use rustBoot_hal::pico::rp2040::FlashWriterEraser;
use rustBoot_update::update::{update_flash::FlashUpdater, UpdateInterface};

#[entry]
fn main() -> ! {

    // defmt::println!("Starting Update Firmware...");

    let mut pac = pac::Peripherals::take().unwrap();
    // let core = pac::CorePeripherals::take().unwrap();
    // // let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // // External high-speed crystal on the pico board is 12Mhz
    // let external_xtal_freq_hz = 12_000_000u32;
    // let clocks = init_clocks_and_plls(
    //     external_xtal_freq_hz,
    //     pac.XOSC,
    //     pac.CLOCKS,
    //     pac.PLL_SYS,
    //     pac.PLL_USB,
    //     &mut pac.RESETS,
    //     //&mut watchdog,
    // )
    // .ok()
    // .unwrap();

    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output(); 

    let flash_writer = FlashWriterEraser {};
    let updater = FlashUpdater::new(flash_writer);

    match updater.update_success() {
        Ok(_v) => {}
        Err(e) => panic!("couldnt trigger update: {}", e),
    }

    loop {
        led_pin.set_high().unwrap();
        // delay.delay_ms(50);
        asm::delay(320000);
        led_pin.set_low().unwrap();
        // delay.delay_ms(50);
        asm::delay(320000);
    }
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
// End of file
