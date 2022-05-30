#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zeno::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use zeno::println;

entry_point!(main);

pub fn main(boot_info: &BootInfo) -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zeno::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
