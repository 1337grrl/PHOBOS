// Disabling the standard library
#![no_std]
// Overwriting the usual entry point, since we can use neither Rust runtime nor C runtime Zero (crt0)
// No runtime also means no main function
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

extern crate rlibc;
mod vga_buffer;
use core::panic::PanicInfo;


// Panic handler function
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

// static HELLO: &[u8] = b"Well, hello there! This is Deenas Minimal Rust Kernel is speaking.";

// Disabling name mangling 
#[no_mangle]
// By extern "C" we specify that _start is used by the calling convention of C, 
//meaning it uses C's scheme for how subroutines receive parameters 
//from their caller and how they return a result
pub extern "C" fn _start() -> ! {
	use core::fmt::Write;
	
	println!("Well, hello there. There is a lot of magic happening here, and I'm really not sure that I understand a lot of it. Mostly the part about building macros seemed like high magic to me. But that's okay - this is me just beginning with all the OS stuff and with Rust and therefore I'm telling myself over and over again, that I do not have to understand all the magic right away. By the way: Telling me that is actually nearly as hard as trying to understand any magical details... Phew!");
	
	// panic!("HAAAAALPH!");
	
	#[cfg(test)]
	test_main();
	
	loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
	use x86_64::instructions::port::Port;
	
	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
	
	exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
	print!("trivial assertion...");
	assert_eq!(1, 1);
	println!("[ok]");
}