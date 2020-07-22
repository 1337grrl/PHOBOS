// Disabling the standard library
#![no_std]
// Overwriting the usual entry point, since we can use neither Rust runtime nor C runtime Zero (crt0)
// No runtime also means no main function
#![no_main]

use core::panic::PanicInfo;

// Panic handler function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

// Disabling name mangling 
#[no_mangle]
// By extern "C" we specify that _start is used by the calling convention of C, 
//meaning it uses C's scheme for how subroutines receive parameters 
//from their caller and how they return a result
pub extern "C" fn _start() -> ! {
	loop {}
}