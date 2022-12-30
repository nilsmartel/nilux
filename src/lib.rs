mod uart;

#![no_std]
#![feature(panic_info_message)]

use core::arch::asm;

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print {
    ($($args:tt)+) => {{}};
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

// ///////////////////////////////////
// / LANGUAGE STRUCTURES / FUNCTIONS
// ///////////////////////////////////
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting: ");
    let p = info.location().unwrap_or_else(|| {
        println!("no information available.");
        abort()
    });

    let _line = p.line();
    let _file = p.file();

    println!(
        "line {}, file {}: {}",
        _line,
        _file,
        info.message().unwrap()
    );

    abort();
}
#[no_mangle]
extern "C" fn abort() -> ! {
    loop {
        wait_for_interrupt();
    }
}

fn wait_for_interrupt() {
    unsafe {
        asm!("wfi");
    }
}

#[no_mangle]
extern "C" fn kmain() {
    // Initialize all subsystems and get ready
    // to start scheduling.
    // finally start timer

    abort();
}
