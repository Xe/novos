#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use novos::println;

entry_point!(kernel_main);

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    novos::hlt_loop();
}

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use novos::interrupts::PICS;
    use novos::memory;
    use novos::memory::{create_example_mapping, EmptyFrameAllocator};

    novos::gdt::init();
    novos::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let mut recursive_page_table = unsafe {
        memory::init(boot_info.p4_table_addr as usize)
    };

    let mut recursive_page_table = unsafe { memory::init(boot_info.p4_table_addr as usize) };
    let mut frame_allocator = memory::init_frame_allocator(&boot_info.memory_map);

    create_example_mapping(&mut recursive_page_table, &mut frame_allocator);
    unsafe { (0x1900 as *mut u64).write_volatile(0xf021f077f065f04e)}; 

    println!("It did not crash!");
    novos::hlt_loop();
}