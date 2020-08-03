#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{ InterruptDescriptorTable, InterruptStackFrame };
use phobos::{ exit_qemu, QemuExitCode, serial_print, serial_println };

#[no_mangle]
pub extern "C" fn _start() -> ! {
	serial_print!("stack_overflow::stack_overflow...\t");
	
	phobos::gdt::init();
	init_test_idt();
	
	stack_overflow();
	
	panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
// Trigger a stack overflow
fn stack_overflow() {
	stack_overflow();
	volatile::Volatile::new(0).read(); // preventing tail recursion optimizations
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	phobos::test_panic_handler(info)
}

lazy_static! {
	static ref TEST_IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		unsafe {
			idt.double_fault
			.set_handler_fn(test_double_fault_handler)
			.set_stack_index(phobos::gdt::DOUBLE_FAULT_IST_INDEX);
		}
		
		idt
	};
}

pub fn init_test_idt() {
	TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}