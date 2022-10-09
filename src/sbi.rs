use core::arch::asm;

#[derive(Debug)]
#[repr(usize)]
pub enum Error {
    Success,
    Failed,
    NotSupported,
    InvalidParam,
    Denied,
    InvalidAddress,
    AlreadyAvailable,
}

impl From<isize> for Error {
    fn from(val: isize) -> Self {
        match val {
            0 => Self::Success,
            -1 => Self::Failed,
            -2 => Self::NotSupported,
            -3 => Self::InvalidParam,
            -4 => Self::Denied,
            -5 => Self::InvalidAddress,
            -6 => Self::AlreadyAvailable,
            x => panic!("Unknown sbi error: {}", x),
        }
    }
}

#[derive(Debug)]
pub struct SbiRet {
    pub error: Error,
    pub value: usize,
}

#[inline(always)]
fn sbi_call(ext_id: usize, func_id: usize, arg0: usize, arg1: usize, arg2: usize) -> SbiRet {
    unsafe {
        let error: isize;
        let value;
        asm!(
            "ecall",
            lateout("a0") error,
            lateout("a1") value,
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a6") func_id,
            in("a7") ext_id,
            options(nostack)
        );
        let error = error.into();
        SbiRet { error, value }
    }
}


pub mod base {
    pub const EID: usize = 0x10;

    use super::*;

    pub fn sbi_get_spec_version() -> SbiRet {
        sbi_call(EID, 0, 0, 0, 0)
    }

    pub fn sbi_get_impl_id() -> SbiRet {
        sbi_call(EID, 1, 0, 0, 0)
    }
    
    pub fn sbi_get_impl_version() -> SbiRet {
        sbi_call(EID, 2, 0, 0, 0)
    }
    
    pub fn sbi_probe_extension(id: usize) -> SbiRet {
        sbi_call(EID, 3, id, 0, 0)
    }
    
    pub fn sbi_get_vendorid() -> SbiRet {
        sbi_call(EID, 4, 0, 0, 0)
    }
    
    pub fn sbi_get_marchid() -> SbiRet {
        sbi_call(EID, 5, 0, 0, 0)
    }
    
    pub fn sbi_get_mimpid() -> SbiRet {
        sbi_call(EID, 6, 0, 0, 0)
    }
}

pub mod legacy {
    use super::*;

    pub const CONSOLE_PUTC_EID: usize = 0x01;
    pub const CONSOLE_GETC_EID: usize = 0x02;
    pub const SHUTDOWN_EID: usize = 0x08;

    pub fn sbi_console_putchar(c: u8) -> SbiRet {
        sbi_call(CONSOLE_PUTC_EID, 0, c.into(), 0, 0)
    }

    pub fn sbi_console_getchar() -> SbiRet {
        sbi_call(CONSOLE_GETC_EID, 0, 0, 0, 0)
    }

    pub fn sbi_shutdown() -> SbiRet {
        sbi_call(SHUTDOWN_EID, 0, 0, 0, 0)
    }
}

pub mod hsm {
    //use super::*;
    pub const EID: usize = 0x48534D;
}

pub mod reset {
    use super::*;
    pub const EID: usize = 0x53525354;

    #[derive(Debug, Clone, Copy)]
    pub enum Type {
        Shutdown,
        ColdReboot,
        WarmReboot,
    }

    impl From<Type> for usize {
        fn from(x: Type) -> Self {
            match x {
                Type::Shutdown => 0x00000000,
                Type::ColdReboot => 0x00000001,
                Type::WarmReboot => 0x00000002,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Reason {
        NoReason,
        SystemFailure,
    }

    impl From<Reason> for usize {
        fn from(x: Reason) -> Self {
            match x {
                Reason::NoReason => 0x00000000,
                Reason::SystemFailure => 0x00000001,
            }
        }
    }

    pub fn sbi_system_reset(reset_type: Type, reset_reason: Reason) -> SbiRet {
        sbi_call(EID, 0, reset_type.into(), reset_reason.into(), 0)
    }
}

pub struct LegacyConsole;

impl LegacyConsole {
    pub fn supports_write(&self) -> bool {
        match base::sbi_probe_extension(legacy::CONSOLE_PUTC_EID) {
            SbiRet {error: Error::Success, value: res} if res != 0 => true,
            _ => false,
        }
    }

    pub fn supports_read(&self) -> bool {
        match base::sbi_probe_extension(legacy::CONSOLE_GETC_EID) {
            SbiRet {error: Error::Success, value: res} if res != 0 => true,
            _ => false,
        }
    }

    pub fn putc(&self, c: u8) {
        legacy::sbi_console_putchar(c);
    }

    pub fn getc(&self) -> Option<u8> {
        match legacy::sbi_console_getchar() {
            SbiRet {error: Error::Success, value: c} => Some(c as u8),
            _ => None,
        }
    }
}

impl core::fmt::Write for LegacyConsole {
    fn write_str(&mut self, out: &str) -> Result<(), core::fmt::Error> {
        for c in out.bytes() {
            self.putc(c);
        }
        Ok(())
    }
}


//#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::sbi::LegacyConsole, $($args)+);
			});
}

//#[macro_export]
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

pub(crate) use print;
pub(crate) use println;