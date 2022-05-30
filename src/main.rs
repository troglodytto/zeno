#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zeno::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use zeno::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    println,
};

entry_point!(main);

pub fn main(boot_info: &'static BootInfo) -> ! {
    println!("Hello, world!");
    zeno::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    #[cfg(test)]
    test_main();

    println!("Crash Prevented");
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
