use core::arch::asm;

#[lang = "eh_personality"] extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	// print!("Aborting: ");
	// if let Some(p) = info.location() {
	// 	println!(
	// 	         "line {}, file {}: {}",
	// 	         p.line(),
	// 	         p.file(),
	// 	         info.message().unwrap()
	// 	);
	// }
	// else {
	// 	println!("no information available.");
	// }
	abort();
}

#[cfg(test)]
#[no_mangle]
extern "C" fn abort() -> ! {
	// use crate::sbi;
	// sbi::reset::sbi_system_reset(sbi::reset::Type::Shutdown, sbi::reset::Reason::NoReason);
	// sbi::legacy::sbi_shutdown();
	// println!("Failed to shut down. Haning...");
	loop {
		unsafe {
			asm!("wfi");
		}
	}
}

#[cfg(not(test))]
#[no_mangle]
extern "C" fn abort() -> ! {
	loop {
		unsafe {
			asm!("wfi");
		}
	}
}