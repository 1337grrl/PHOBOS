// Disabling the standard library
#![no_std]
// Overwriting the usual entry point, since we can use neither Rust runtime nor C runtime Zero (crt0)
// No runtime also means no main function
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(phobos::test_runner)]

use phobos::println;
use core::panic::PanicInfo;

// Disabling name mangling 
#[no_mangle]
// By extern "C" we specify that _start is used by the calling convention of C, 
//meaning it uses C's scheme for how subroutines receive parameters 
//from their caller and how they return a result
pub extern "C" fn _start() -> ! {	
	println!("Well, hello there. There is a lot of magic happening here, and I'm really not sure that I understand a lot of it. Mostly the part about building macros seemed like high magic to me. But that's okay - this is me just beginning with all the OS stuff and with Rust and therefore I'm telling myself over and over again, that I do not have to understand all the magic right away. By the way: Telling me that is actually nearly as hard as trying to understand any magical details... Phew!");

    phobos::init();
	
	// Invoking breakpoint exception
	// x86_64::instructions::interrupts::int3();
	
	// Triggering a page fault
	/*unsafe {
		*(0xdeadbeef as *mut u64) = 42;
	};
	*/


	#[cfg(test)]
	test_main();
	
	println!("Did not crash...yet...");
	loop {
		/* Provokes a deadlock*/
		use phobos::print;
		for _ in 0..2147483647 {
			print!("-");
		}
		
	}
}

// Panic handler function
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	phobos::test_panic_handler(info)
}