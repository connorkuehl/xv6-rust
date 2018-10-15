#![no_std]

extern crate x86;

pub mod arch;
pub mod console;
pub mod file;
pub mod fs;
pub mod ioapic;
pub mod lapic;
pub mod string;
pub mod syscall;
pub mod sysproc;
pub mod param;
pub mod pipe;
pub mod proc;
pub mod trap;
pub mod traps;
pub mod types;
pub mod uart;
pub mod kbd;

use uart::uartputc;

use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let message = b"HELLO FROM RUST\n";

    for ch in message {
        uartputc(*ch as i32);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
