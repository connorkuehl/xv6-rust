#![no_std]

use core::panic::PanicInfo;

extern "C" {
    fn uartputc(c: i32);
}

#[no_mangle]
pub extern "C" fn kmain() {
    let message = b"HELLO FROM RUST\n";

    for ch in message {
        unsafe {
            uartputc(*ch as i32);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


