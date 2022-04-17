// 不使用标准库
#![no_std]
// 不使用预定义入口点
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

// use core::ptr;

mod panic;
mod uart_console;
mod interrupts;

global_asm!(include_str!("start.s"));

#[no_mangle] // 不修改函数名
pub extern "C" fn not_main() {
/*
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"AArch64 Bare Metal";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
*/
    println!("[0] Hello from Rust!");
    let banner = r#"
  ___  ____                     _    ____  __  __       ___      ____  _   _ _   _ _   _
 / _ \/ ___|    ___  _ __      / \  |  _ \|  \/  |_   _( _ )    / __ \| | | | \ | | | | |
| | | \___ \   / _ \| '_ \    / _ \ | |_) | |\/| \ \ / / _ \   / / _` | |_| |  \| | | | |
| |_| |___) | | (_) | | | |  / ___ \|  _ <| |  | |\ V / (_) | | | (_| |  _  | |\  | |_| |
 \___/|____/   \___/|_| |_| /_/   \_\_| \_\_|  |_| \_/ \___/   \ \__,_|_| |_|_| \_|\___/
                                                                \____/
   
                    /////////////////////////////////////////////
                    //                                         //
                    //                                         //
                    //    +   +                                //
                    //   +++ +++                               //
                    //  +++++++++                              //
                    //    +++++            /\                 ///
                    //     +++            /  \               / //
                    //      +            /    \             /  //
                    //                  /      \           /   //
                    //                 /        \_________/    //
                    //                /                        //
                    //               /       \         /       //
                    //              /       __\       /__      //
                    //             /                           //
                    //            /             ____           //
                    //           /              \  /           //
                    //          /     _~_        \/    _~_     //
                    //         /     /  /             |   |    //
                    //        /     /  /              |   |    //
                    //       /     /  /               |   |    //
                    /////////////////////////////////////////////
    "#;

    print!("{}\n\n",banner);
    interrupts::init_gicv2();
    print!("[1] gic_v2 init finished!\n\n");

    loop{
        unsafe {
            asm!("wfi"); //  to low-power
        }
    }
}