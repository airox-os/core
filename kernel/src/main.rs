#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;
use common::vga_buffer::WRITER;

/// Entry point for the kernel. This symbol is looked up by the linker script.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Print a welcome message
    write!(WRITER.lock(), "Hello, Airox OS!\n").unwrap();
    loop {}
}

/// Panic handler: prints info and halts
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write!(WRITER.lock(), "Kernel panic: {}\n", info).unwrap();
    loop {}
}
