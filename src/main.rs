#![no_std]
#![no_main]
#![feature(panic_info_message, int_roundings)]

use core::arch::asm;


pub mod assembly;
pub mod uart;

pub mod page;

use crate::uart::UART_BASE_ADDR;

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
			});
}
#[macro_export]
macro_rules! println
{
	() => ({
		   print!("\r\n")
		   });
	($fmt:expr) => ({
			print!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			print!(concat!($fmt, "\r\n"), $($args)+)
			});
}

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(p) = info.location() {
		println!(
					"line {}, file {}: {}",
					p.line(),
					p.file(),
					info.message().unwrap()
		);
	}
	else {
		println!("no information available.");
	}
	abort();
}

#[no_mangle]
extern "C"
fn abort() -> ! {
	loop {
		unsafe {
            // The asm! syntax has changed in Rust.
            // For the old, you can use llvm_asm!, but the
            // new syntax kicks ass--when we actually get to use it.
			asm!("wfi");
		}
	}
}

#[no_mangle]
extern "C"
fn kmain() {
	uart::Uart::new(UART_BASE_ADDR).init();
	print!("OS Starting... ");

	page::init();

	println!("[ Done ]")

	// Main should initialize all sub-systems and get
	// ready to start scheduling. The last thing this
	// should do is start the timer.
}