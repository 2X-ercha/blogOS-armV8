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

    print!("{}",banner);
    interrupts::init_gicv2();
    print!("interrupt init!\n");

    loop{
        unsafe {
            // 在ARM体系结构中，处理器内部有通用计时器，通用计时器包含一组比较器，用来与系统计数器进行比较，一旦通用计时器的值小于等于系统计数器时便会产生时钟中断。
            // 比较寄存器有64位，如果设置了之后，当系统计数器达到或超过了这个值之后（CVAL<系统计数器），就会触发定时器中断。
            // 定时寄存器有32位，如果设置了之后，会将比较寄存器设置成当前系统计数器加上设置的定时寄存器的值（CVAL=系统计数器+TVAL）
            /*
                对于系统计数器来说，可以通过读取控制寄存器CNTPCT_EL0来获得当前的系统计数值（无论处于哪个异常级别）
                CNTPCT_EL0- physical counter value register
                CNTP_CTL_EL0- physical counter control register
                CNTP_TVAL_EL0 and CNTP_CVAL_EL0- two threshold value registers, 定时寄存器（TVAL） and 比较寄存器（CVAL）
                CNTFRQ_EL0- counter frequency register

                MRS: 状态寄存器到通用寄存器的传送指令。
                MRS:({R0-R12}<-CPSR,SPSR)
                MSR: 通用寄存器到状态寄存器的传送指令。
                MSR:(CPSR,SPSR<-{R0-R12})
            */
            asm!("wfi"); //  to low-power(close clock)
        }
    }
}