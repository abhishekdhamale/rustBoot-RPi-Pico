#![no_std]
#![feature(const_fn_fn_ptr_basics)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(asm)]
#![feature(asm_const)]
#![allow(warnings)]
#![feature(core_intrinsics)]
// #[cfg(feature = "nrf")]
// pub mod nrf;
// #[cfg(feature = "rpi")]
// pub mod rpi;
// #[cfg(feature = "stm")]
// pub mod stm;
#[cfg(feature = "pico")]
pub mod pico;

/// This is the trait that abstracts out the necessary hardware-specific flash operations
/// such as
///
/// - `writing to flash` - write an arbitrary blob of data to an arbitrary location in flash
/// - `erasing a flash page` - erase a page of flash, given the address (i.e. first word) of the page
/// to be erased and number of btyes to erase.
///
pub trait FlashInterface {
    fn hal_init();
    fn hal_flash_unlock(&self);
    fn hal_flash_lock(&self);
    fn hal_flash_write(&self, addr: usize, data: *const u8, len: usize);
    fn hal_flash_erase(&self, addr: usize, len: usize);
}

// Arch-specific code
pub fn preboot() {}
pub fn boot_from(fw_base_address: usize) -> ! {
    #[cfg(feature = "rp2040")]
    crate::pico::rp2040::boot_from(fw_base_address);

    panic!(": unrecognized board")
}
