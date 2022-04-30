use core::ptr;
use core::arch::asm;
use core::arch::global_asm;

/*
    GIC 部分的设备树描述：

    * intc中的 reg 指明GICD寄存器映射到内存的位置为0x8000000，长度为0x10000， GICC寄存器映射到内存的位置为0x8010000，长度为0x10000

    * intc中的 #interrupt-cells 指明 interrupts 包括3个cells。文档指明：
        第一个cell为中断类型，0表示SPI，1表示PPI；
        第二个cell为中断号，SPI范围为[0-987]，PPI为[0-15]；
        第三个cell为flags，其中
            [3:0]位表示触发类型，4表示高电平触发，
            [15:8]为PPI的cpu中断掩码，每1位对应一个cpu，为1表示该中断会连接到对应的cpu。

    intc@8000000 {
            phandle = <0x8001>;
            reg = <0x00 0x8000000 0x00 0x10000 0x00 0x8010000 0x00 0x10000>;
            compatible = "arm,cortex-a15-gic";
            ranges;
            #size-cells = <0x02>;
            #address-cells = <0x02>;
            interrupt-controller;
            #interrupt-cells = <0x03>;

            v2m@8020000 {
                    phandle = <0x8002>;
                    reg = <0x00 0x8020000 0x00 0x1000>;
                    msi-controller;
                    compatible = "arm,gic-v2m-frame";
            };
    };
 */

// GICD和GICC寄存器内存映射后的起始地址
const GICD_BASE: u64 = 0x08000000;
const GICC_BASE: u64 = 0x08010000;

// Distributor
const GICD_CTLR: *mut u32 = (GICD_BASE + 0x0) as *mut u32;
const GICD_ISENABLER: *mut u32 = (GICD_BASE + 0x0100) as *mut u32;
// const GICD_ICENABLER: *mut u32 = (GICD_BASE + 0x0180) as *mut u32;
const GICD_ICPENDR: *mut u32 = (GICD_BASE + 0x0280) as *mut u32;
const GICD_IPRIORITYR: *mut u32 = (GICD_BASE + 0x0400) as *mut u32;
const GICD_ICFGR: *mut u32 = (GICD_BASE + 0x0c00) as *mut u32;

const GICD_CTLR_ENABLE: u32 = 1;  /* Enable GICD */
const GICD_CTLR_DISABLE: u32 = 0;     /* Disable GICD */
const GICD_ISENABLER_SIZE: u32 = 32;
// const GICD_ICENABLER_SIZE: u32 = 32;
const GICD_ICPENDR_SIZE: u32 = 32;
const GICD_IPRIORITY_SIZE: u32 = 4;
const GICD_IPRIORITY_BITS: u32 = 8;
const GICD_ICFGR_SIZE: u32 = 16;
const GICD_ICFGR_BITS: u32 = 2;


// CPU Interface
const GICC_CTLR: *mut u32 = (GICC_BASE + 0x0) as *mut u32;
const GICC_PMR: *mut u32 = (GICC_BASE + 0x0004) as *mut u32;
const GICC_BPR: *mut u32 = (GICC_BASE + 0x0008) as *mut u32;

const GICC_CTLR_ENABLE: u32 = 1;
const GICC_CTLR_DISABLE: u32 = 0;
// Priority Mask Register. interrupt priority filter, Higher priority corresponds to a lower Priority field value.
const GICC_PMR_PRIO_LOW: u32 = 0xff;
// The register defines the point at which the priority value fields split into two parts,
// the group priority field and the subpriority field. The group priority field is used to
// determine interrupt preemption. NO GROUP.
const GICC_BPR_NO_GROUP: u32 = 0x00;

const GICC_IAR: *mut u32 = (GICC_BASE + 0x0c) as *mut u32;
const GICC_EOIR: *mut u32 = (GICC_BASE + 0x10) as *mut u32;

// 电平触发
const ICFGR_LEVEL: u32 = 0;
// 时钟中断号30
const TIMER_IRQ: u32 = 30;
// 设备中断号33
const UART0_IRQ: u32 = 33;
// GPIO中断号39
const GPIO_IRQ: u32 = 39;

use tock_registers::interfaces::{Readable, Writeable};

static mut RUN_TIME: u32 = 0;
static mut RUN_TIME_INFO_SWITCH: bool = true;

pub fn init_gicv2() {
    // 初始化Gicv2的distributor和cpu interface
    // 禁用distributor和cpu interface后进行相应配置
    unsafe {
        ptr::write_volatile(GICD_CTLR, GICD_CTLR_DISABLE);
        ptr::write_volatile(GICC_CTLR, GICC_CTLR_DISABLE);
        ptr::write_volatile(GICC_PMR, GICC_PMR_PRIO_LOW);
        ptr::write_volatile(GICC_BPR, GICC_BPR_NO_GROUP);
    }

    // 启用distributor和cpu interface
    unsafe {
        ptr::write_volatile(GICD_CTLR, GICD_CTLR_ENABLE);
        ptr::write_volatile(GICC_CTLR, GICC_CTLR_ENABLE);
    }

    set_config(TIMER_IRQ, ICFGR_LEVEL); //电平触发
    set_priority(TIMER_IRQ, 0); //优先级设定
    clear(TIMER_IRQ); //清除中断请求
    enable(TIMER_IRQ); //使能中断


    //配置timer
    unsafe {
        // 在ARM体系结构中，处理器内部有通用计时器，通用计时器包含一组比较器，用来与系统计数器进行比较，一旦通用计时器的值小于等于系统计数器时便会产生时钟中断。
        // 比较寄存器有64位，如果设置了之后，当系统计数器达到或超过了这个值之后（CVAL<系统计数器），就会触发定时器中断。
        // 定时寄存器有32位，如果设置了之后，会将比较寄存器设置成当前系统计数器加上设置的定时寄存器的值（CVAL=系统计数器+TVAL）
        /*
            timer 设备树描述：

            * timer设备其中包括4个中断。
                以第二个中断的参数 0x01 0x0e 0x104 为例，其指明该中断为PPI类型的中断，中断号14， 路由到第一个cpu，且高电平触发。
                但注意到PPI的起始中断号为16，所以实际上该中断在GICv2中的中断号应为16 + 14 = 30。

            timer {
                    interrupts = <0x01 0x0d 0x104 0x01 0x0e 0x104 0x01 0x0b 0x104 0x01 0x0a 0x104>;
                    always-on;
                    compatible = "arm,armv8-timer\0arm,armv7-timer";
            };
         */
        /*
            对于系统计数器来说，可以通过读取控制寄存器CNTPCT_EL0来获得当前的系统计数值（无论处于哪个异常级别）
            CNTPCT_EL0- physical counter value register
            CNTP_CTL_EL0- physical counter control register
            CNTP_TVAL_EL0 and CNTP_CVAL_EL0- two threshold value registers, 定时寄存器（TVAL） and 比较寄存器（CVAL）
            CNTFRQ_EL0- counter frequency register

            每组定时器都还有一个控制寄存器（CTL），其只有最低三位有意义，其它的60位全是保留的，设置成0.
            0:ENABLE：是否打开定时器，使其工作；
            1:IMASK：中断掩码，如果设置成1，则即使定时器是工作的，仍然不会发出中断；
            2:ISTATUS：如果定时器打开的话，且满足了触发条件，则将这一位设置成1。
         */
        asm!("mrs x1, CNTFRQ_EL0"); //读取系统频率
        asm!("msr CNTP_TVAL_EL0, x1");  //设置定时寄存器
        asm!("mov x0, 1");
        asm!("msr CNTP_CTL_EL0, x0"); //enable=1, imask=0, istatus= 0
        /*
            MRS: 状态寄存器到通用寄存器的传送指令。
            MRS:({R0-R12}<-CPSR,SPSR)
            MSR: 通用寄存器到状态寄存器的传送指令。
            MSR:(CPSR,SPSR<-{R0-R12})
         */
        // irq enable
        asm!("msr daifclr, #2");
    }

    unsafe {
        RUN_TIME = 0;
        RUN_TIME_INFO_SWITCH = true;
    }

    // 初始化UART0 中断
    // interrupts = <0x00 0x01 0x04>; SPI, 0x01, level
    set_config(UART0_IRQ, ICFGR_LEVEL); //电平触发
    set_priority(UART0_IRQ, 0); //优先级设定
    // set_core(TIMER_IRQ, 0x1); // 单核实现无需设置中断目标核
    clear(UART0_IRQ); //清除中断请求
    enable(UART0_IRQ); //使能中断

    // 初始化GPIO中断
    set_config(GPIO_IRQ, ICFGR_LEVEL); //电平触发
    set_priority(GPIO_IRQ, 0); //优先级设定
    clear(GPIO_IRQ); //清除中断请求
    enable(GPIO_IRQ); //使能中断

    // 使能GPIO的poweroff key中断
    use crate::pl061::*;
    unsafe{
        let pl061r: &PL061Regs = &*PL061REGS;

        // 启用pl061 gpio中的3号线中断
        // .write(): 写入一个或多个字段的值，将其他字段改写为零
        pl061r.ie.write(GPIOIE::IO3::Enabled);

    }
}

// 使能中断号为interrupt的中断
pub fn enable(interrupt: u32) {
    unsafe {
        ptr::write_volatile(
            GICD_ISENABLER.add((interrupt / GICD_ISENABLER_SIZE) as usize),
            1 << (interrupt % GICD_ISENABLER_SIZE)
        );
    }
}

// 禁用中断号为interrupt的中断
/*
pub fn disable(interrupt: u32) {
    unsafe {
        ptr::write_volatile(
            GICD_ICENABLER.add((interrupt / GICD_ICENABLER_SIZE) as usize),
            1 << (interrupt % GICD_ICENABLER_SIZE)
        );
    }
}*/

// 清除中断号为interrupt的中断
pub fn clear(interrupt: u32) {
    unsafe {
        ptr::write_volatile(
            GICD_ICPENDR.add((interrupt / GICD_ICPENDR_SIZE) as usize),
            1 << (interrupt % GICD_ICPENDR_SIZE)
        );
    }
}

// 设置中断号为interrupt的中断的优先级为priority
pub fn set_priority(interrupt: u32, priority: u32) {
    let shift = (interrupt % GICD_IPRIORITY_SIZE) * GICD_IPRIORITY_BITS;
    unsafe {
        let addr: *mut u32 = GICD_IPRIORITYR.add((interrupt / GICD_IPRIORITY_SIZE) as usize);
        let mut value: u32 = ptr::read_volatile(addr);
        value &= !(0xff << shift);
        value |= priority << shift;
        ptr::write_volatile(addr, value);
    }
}

// 设置中断号为interrupt的中断的属性为config
pub fn set_config(interrupt: u32, config: u32) {
    let shift = (interrupt % GICD_ICFGR_SIZE) * GICD_ICFGR_BITS;
    unsafe {
        let addr: *mut u32 = GICD_ICFGR.add((interrupt / GICD_ICFGR_SIZE) as usize);
        let mut value: u32 = ptr::read_volatile(addr);
        value &= !(0x03 << shift);
        value |= config << shift;
        ptr::write_volatile(addr, value);
    }
}

global_asm!(include_str!("exceptions.s"));

#[repr(C)]
pub struct ExceptionCtx {
    regs: [u64; 30],
    elr_el1: u64,
    spsr_el1: u64,
    lr: u64,
}

// 异常处理函数
const EL1_SP0_SYNC: &'static str = "EL1_SP0_SYNC";
const EL1_SP0_IRQ: &'static str = "EL1_SP0_IRQ";
const EL1_SP0_FIQ: &'static str = "EL1_SP0_FIQ";
const EL1_SP0_ERROR: &'static str = "EL1_SP0_ERROR";
const EL1_SYNC: &'static str = "EL1_SYNC";
const EL1_IRQ: &'static str = "EL1_IRQ";
const EL1_FIQ: &'static str = "EL1_FIQ";
const EL1_ERROR: &'static str = "EL1_ERROR";
const EL0_SYNC: &'static str = "EL0_SYNC";
const EL0_IRQ: &'static str = "EL0_IRQ";
const EL0_FIQ: &'static str = "EL0_FIQ";
const EL0_ERROR: &'static str = "EL0_ERROR";
const EL0_32_SYNC: &'static str = "EL0_32_SYNC";
const EL0_32_IRQ: &'static str = "EL0_32_IRQ";
const EL0_32_FIQ: &'static str = "EL0_32_FIQ";
const EL0_32_ERROR: &'static str = "EL0_32_ERROR";

// 调用print!宏打印异常信息，你也可以选择打印异常发生时所有寄存器的信息
fn catch(ctx: &mut ExceptionCtx, name: &str) {
    crate::print!(
        "\n  \
        {} @ 0x{:016x}\n",
        name, ctx.elr_el1,
    );
}

#[no_mangle]
unsafe extern "C" fn el1_sp0_sync(ctx: &mut ExceptionCtx) {
    catch(ctx, EL1_SP0_SYNC);
}
#[no_mangle]
unsafe extern "C" fn el1_sp0_irq(ctx: &mut ExceptionCtx) {
    catch(ctx, EL1_SP0_IRQ);
}
#[no_mangle]
unsafe extern "C" fn el1_sp0_fiq(ctx: &mut ExceptionCtx) {
    catch(ctx, EL1_SP0_FIQ);
}
#[no_mangle]
unsafe extern "C" fn el1_sp0_error(ctx: &mut ExceptionCtx) {
    catch(ctx, EL1_SP0_ERROR);
}
#[no_mangle]
unsafe extern "C" fn el1_sync(ctx: &mut ExceptionCtx) {
    catch(ctx, EL1_SYNC);
}
#[no_mangle]
unsafe extern "C" fn el1_irq(ctx: &mut ExceptionCtx) {
    // reads this register to obtain the interrupt ID of the signaled interrupt.
    // This read acts as an acknowledge for the interrupt.
    // 中断确认
    let value: u32 = ptr::read_volatile(GICC_IAR);
    // only value == 30 (0x21) print ......
    let irq_num: u32 = value & 0x1ff;
    let core_num: u32 = value & 0xe00;

    // 实际处理中断
    handle_irq_lines(ctx, core_num, irq_num);
    // catch(ctx, EL1_IRQ);

    // A processor writes to this register to inform the CPU interface either:
    // • that it has completed the processing of the specified interrupt
    // • in a GICv2 implementation, when the appropriate GICC_CTLR.EOImode bit is set to 1, to indicate that the interface should perform priority drop for the specified interrupt.
    // 标记中断完成，清除相应中断位
    ptr::write_volatile(GICC_EOIR, core_num | irq_num);
    clear(irq_num);
}
#[no_mangle]
unsafe extern "C" fn el1_fiq(ctx: &mut ExceptionCtx) {
    catch(ctx, EL1_FIQ);
}
#[no_mangle]
unsafe extern "C" fn el1_error(ctx: &mut ExceptionCtx) {
    catch(ctx, EL1_ERROR);
}
#[no_mangle]
unsafe extern "C" fn el0_sync(ctx: &mut ExceptionCtx) {
    catch(ctx, EL0_SYNC);
}
#[no_mangle]
unsafe extern "C" fn el0_irq(ctx: &mut ExceptionCtx) {
    catch(ctx, EL0_IRQ);
}
#[no_mangle]
unsafe extern "C" fn el0_fiq(ctx: &mut ExceptionCtx) {
    catch(ctx, EL0_FIQ);
}
#[no_mangle]
unsafe extern "C" fn el0_error(ctx: &mut ExceptionCtx) {
    catch(ctx, EL0_ERROR);
}
#[no_mangle]
unsafe extern "C" fn el0_32_sync(ctx: &mut ExceptionCtx) {
    catch(ctx, EL0_32_SYNC);
}
#[no_mangle]
unsafe extern "C" fn el0_32_irq(ctx: &mut ExceptionCtx) {
    catch(ctx, EL0_32_IRQ);
}
#[no_mangle]
unsafe extern "C" fn el0_32_fiq(ctx: &mut ExceptionCtx) {
    catch(ctx, EL0_32_FIQ);
}
#[no_mangle]
unsafe extern "C" fn el0_32_error(ctx: &mut ExceptionCtx) {
    catch(ctx, EL0_32_ERROR);
}

fn handle_irq_lines(ctx: &mut ExceptionCtx, _core_num: u32, irq_num: u32) {
    if irq_num == TIMER_IRQ {
        handle_timer_irq(ctx);
    } else if irq_num == UART0_IRQ {
        handle_uart0_rx_irq(ctx);
    } else if irq_num == GPIO_IRQ {
        handle_gpio_irq(ctx);
    } else{
        catch(ctx, EL1_IRQ);
    }
}

fn handle_timer_irq(_ctx: &mut ExceptionCtx){

    unsafe {
        if RUN_TIME_INFO_SWITCH {
            crate::print!("\r[RUN TIME INFO] BlogOS for armV8 has run:\t {} h {:>02} m {:>02} s", RUN_TIME/3600, RUN_TIME%3600/60, RUN_TIME%60);
        }
    }

    // 每1秒产生一次中断
    unsafe {
        asm!("mrs x1, CNTFRQ_EL0");
        asm!("msr CNTP_TVAL_EL0, x1");
    }

    unsafe {
        RUN_TIME += 1;
    }
}

fn handle_uart0_rx_irq(_ctx: &mut ExceptionCtx){
    use crate::uart_console::pl011::*;

    // crate::print!("R");
    unsafe{
        // pl011 device registers
        let pl011r: &PL011Regs = &*PL011REGS;

        let mut flag = pl011r.fr.read(UARTFR::RXFE);
        while flag != 1 {
            let value = pl011r.dr.read(UARTDR::DATA);
            if value == 13 {
                // 回车
                crate::print!("\n");
            } else if value == 64 && RUN_TIME_INFO_SWITCH{
                RUN_TIME_INFO_SWITCH = false;
                crate::print!("\n[RUN TIME INFO] BlogOS for armV8 Timer Info has close.\n");
                crate::print!("--------------------------------------------------------------------------------------------\n");
            } else if value == 64 && !RUN_TIME_INFO_SWITCH{
                RUN_TIME_INFO_SWITCH = true;
                crate::print!("\n[RUN TIME INFO] BlogOS for armV8 Timer Info has open.\n");
                crate::print!("--------------------------------------------------------------------------------------------\n");
            } else {
                crate::print!("{}", value as u8 as char);
            }
            flag = pl011r.fr.read(UARTFR::RXFE);
        }
    }
}

fn handle_gpio_irq(_ctx: &mut ExceptionCtx){
    use crate::pl061::*;
    crate::println!("power off!\n");
    unsafe {
        let pl061r: &PL061Regs = &*PL061REGS;

        // 清除中断信号 此时get到的应该是0x8
        // .set(): 设置原始寄存器值; .get(): 获取原始寄存器值
        pl061r.ic.set(pl061r.ie.get());
        // 关机
        asm!("mov w0, #0x18");
        asm!("hlt #0xF000");
    }
}
