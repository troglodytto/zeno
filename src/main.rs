#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zeno::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use zeno::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    zeno::init();

    #[cfg(test)]
    test_main();

    zeno::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    zeno::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zeno::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
