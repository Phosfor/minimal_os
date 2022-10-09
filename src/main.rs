#![no_std]
#![no_main]
#![feature(lang_items)]

mod lang_items;
pub mod sbi;

use core::arch::global_asm;
use sbi::*;

global_asm!(include_str!("boot.asm"));


#[export_name = "kinit"]
pub extern "C" fn kinit() {
    println!("Hello world");
}
