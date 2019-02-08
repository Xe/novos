#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use novos::println;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    novos::hlt_loop();
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use novos::interrupts::PICS;
    println!("Hello World{}", "!");

    novos::gdt::init();
    novos::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    println!("goodbye");

    novos::hlt_loop();
}
