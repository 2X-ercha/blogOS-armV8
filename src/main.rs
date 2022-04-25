// 不使用标准库
#![no_std]
// 不使用预定义入口点
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

mod panic;
mod uart_console;
mod interrupts;
mod pl061;

global_asm!(include_str!("start.s"));

#[no_mangle] // 不修改函数名
pub extern "C" fn not_main() {
    println!("[0] Hello from Rust!");
    interrupts::init_gicv2();
    unsafe {
        loop {
            asm!("wfi");
        }
    }
}

