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
mod pl061;

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
    let banner = r#"
        ________  ___       ________  ________  ________  ________
       |\   __  \|\  \     |\   __  \|\   ____\|\   __  \|\   ____\
       \ \  \|\ /\ \  \    \ \  \|\  \ \  \___|\ \  \|\  \ \  \___|_
        \ \   __  \ \  \    \ \  \\\  \ \  \  __\ \  \\\  \ \_____  \
         \ \  \|\  \ \  \____\ \  \\\  \ \  \|\  \ \  \\\  \|____|\  \
          \ \_______\ \_______\ \_______\ \_______\ \_______\____\_\  \             __
           \|_______|\|_______|\|_______|\|_______|\|_______|\_________\          /'_ `\
                                                  __     _ __\|_________|  __  __/\ \L\ \
                                                /'__`\  /\`'__\/' __` __`\/\ \/\ \/_> _ <_
                                               /\ \L\.\_\ \ \/ /\ \/\ \/\ \ \ \_/ |/\ \L\ \
                                               \ \__/.\_\\ \_\ \ \_\ \_\ \_\ \___/ \ \____/
                                                \/__/\/_/ \/_/  \/_/\/_/\/_/\/__/   \/___/
    "#;

    let qaq = r#"

                        /////////////////////////////////////////////
                        //                                         //
                        //                                         //
                        //    +   +                  By noionion   //
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

    print!("\n\n{}\n", banner);
    print!("{}\n", qaq);
    println!("                      Hello from BlogOS_armV8! It's a os write by Rust. \n");
    interrupts::init_gicv2();
    print!("[0] gic_v2 init finished!\n");
    print!("[1] UART init finished!\n");
    print!("[2] GPIO init finished!\n");
    print!("\n[RUN TIME INFO] BlogOS for armV8 Timer Info has open. You can use `@` close/open it.\n");
    print!("----------------------------------------------------------------------------------------------\n");

    loop{
        unsafe {
            asm!("wfi"); //  to low-power
        }
    }
}
