#![no_std]
#![no_main]

// #[cfg(feature = "defmt")]
// use defmt_rtt as _; // global logger

use cortex_m_rt::entry;

use rustBoot_hal::pico::rp2040::FlashWriterEraser;
use rustBoot_update::update::{update_flash::FlashUpdater, UpdateInterface};

#[entry]
fn main() -> ! {
    let updater = FlashUpdater::new(FlashWriterEraser::new());
    updater.rustboot_start()
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
