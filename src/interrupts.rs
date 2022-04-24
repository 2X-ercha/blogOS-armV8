// GICD和GICC寄存器内存映射后的起始地址
const GICD_BASE: u64 = 0x08000000;
const GICC_BASE: u64 = 0x08010000;

// Distributor
const GICD_CTLR: *mut u32 = (GICD_BASE + 0x0) as *mut u32;
const GICD_ISENABLER: *mut u32 = (GICD_BASE + 0x0100) as *mut u32;
const GICD_ICPENDR: *mut u32 = (GICD_BASE + 0x0280) as *mut u32;
const GICD_IPRIORITYR: *mut u32 = (GICD_BASE + 0x0400) as *mut u32;
const GICD_ICFGR: *mut u32 = (GICD_BASE + 0x0c00) as *mut u32;

const GICD_CTLR_ENABLE: u32 = 1;  /* Enable GICD */
const GICD_CTLR_DISABLE: u32 = 0;     /* Disable GICD */
const GICD_ISENABLER_SIZE: u32 = 32;
const GICD_ICPENDR_SIZE: u32 = 32;
const GICD_IPRIORITY_SIZE: u32 = 4;
const GICD_IPRIORITY_BITS: u32 = 8;
const GICD_ICFGR_SIZE: u32 = 16;
const GICD_ICFGR_BITS: u32 = 2;


// CPU Interface
const GICC_CTLR: *mut u32 = (GICC_BASE + 0x0) as *mut u32;
const GICC_PMR: *mut u32 = (GICC_BASE + 0x0004) as *mut u32;
const GICC_BPR: *mut u32 = (GICC_BASE + 0x0008) as *mut u32;
const GICC_IAR: *mut u32 = (GICC_BASE + 0x0c) as *mut u32;
const GICC_EOIR: *mut u32 = (GICC_BASE + 0x10) as *mut u32;

const GICC_CTLR_ENABLE: u32 = 1;
const GICC_CTLR_DISABLE: u32 = 0;
// Priority Mask Register. interrupt priority filter, Higher priority corresponds to a lower Priority field value.
const GICC_PMR_PRIO_LOW: u32 = 0xff;
// The register defines the point at which the priority value fields split into two parts,
// the group priority field and the subpriority field. The group priority field is used to
// determine interrupt preemption. NO GROUP.
const GICC_BPR_NO_GROUP: u32 = 0x00;

// 电平触发
const ICFGR_LEVEL: u32 = 0;
// 时钟中断号30
const TIMER_IRQ: u32 = 30;
// 串口输入中断号
const UART0_IRQ: u32 = 33;

use core::ptr;

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
        asm!("mrs x1, CNTFRQ_EL0"); //读取系统频率
        asm!("msr CNTP_TVAL_EL0, x1");  //设置定时寄存器
        asm!("mov x0, 1");
        asm!("msr CNTP_CTL_EL0, x0"); //enable=1, imask=0, istatus= 0,
        asm!("msr daifclr, #2");
    }

    // 初始化UART0 中断
    // interrupts = <0x00 0x01 0x04>; SPI, 0x01, level
    set_config(UART0_IRQ, ICFGR_LEVEL); //电平触发
    set_priority(UART0_IRQ, 0); //优先级设定
    clear(UART0_IRQ); //清除中断请求
    enable(UART0_IRQ); //使能中断

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
}
*/

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

#[no_mangle]
unsafe extern "C" fn el1_irq(ctx: &mut ExceptionCtx) {
    // reads this register to obtain the interrupt ID of the signaled interrupt.
    // This read acts as an acknowledge for the interrupt.
    // 中断确认
    let value: u32 = ptr::read_volatile(GICC_IAR);
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
    }/*else if irq_num == UART0_IRQ {
        handle_uart0_rx_irq(ctx);
    }*/
    else{
        catch(ctx, EL1_IRQ);
    }
}

fn handle_timer_irq(_ctx: &mut ExceptionCtx){

    crate::print!(".");

    // 每2秒产生一次中断
    unsafe {
        asm!("mrs x1, CNTFRQ_EL0");
        asm!("add x1, x1, x1");
        asm!("msr CNTP_TVAL_EL0, x1");
    }

}