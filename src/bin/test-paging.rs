#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use novos::serial_println;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed\n{}", info);
    novos::hlt_loop();
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use novos::interrupts::PICS;

    novos::gdt::init();
    novos::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    use novos::memory::{self, translate_addr};
    use novos::memory::{create_example_mapping, EmptyFrameAllocator};

    const LEVEL_4_TABLE_ADDR: usize = 0o_177777_777_777_777_777_0000;
    let mut recursive_page_table = unsafe { memory::init(LEVEL_4_TABLE_ADDR) };

    create_example_mapping(&mut recursive_page_table, &mut EmptyFrameAllocator);
    unsafe { (0x1900 as *mut u64).write_volatile(0xf021f077f065f04e)}; 
    
    serial_println!("ok");

    unsafe {
        novos::exit_qemu();
    }

    novos::hlt_loop();
}
