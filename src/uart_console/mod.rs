use core::fmt;
use tock_registers::{interfaces::Writeable};

pub mod pl011;
use pl011::*;

//嵌入式系统使用串口，而不是vga，直接输出，没有颜色控制，不记录列号，也没有frame buffer，所以采用空结构
pub struct Writer;

//往串口寄存器写入字节和字符串进行输出
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        // const UART0: *mut u8 = 0x0900_0000 as *mut u8;
        unsafe {
            // pl011 device registers
            let pl011r: &PL011Regs = &*PL011REGS;
    
            // ptr::write_volatile(UART0, byte);
            pl011r.dr.write(UARTDR::DATA.val(byte as u32));
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.chars() {
            self.write_byte(byte as u8)
        }
    }

    pub fn new() -> Writer{

        unsafe {
            // pl011 device registers
            let pl011r: &PL011Regs = &*PL011REGS;

            // 禁用pl011
            pl011r.cr.write(UARTCR::TXE::Disabled + UARTCR::RXE::Disabled + UARTCR::UARTEN::Disabled);
            // 清空中断状态
            pl011r.icr.write(UARTICR::ALL::Clear);
            // 设定中断mask，需要使能的中断
            pl011r.imsc.write(UARTIMSC::RXIM::Enabled);
            // IBRD = UART_CLK / (16 * BAUD_RATE)
            // FBRD = ROUND((64 * MOD(UART_CLK,(16 * BAUD_RATE))) / (16 * BAUD_RATE))
            // UART_CLK = 24M
            // BAUD_RATE = 115200
            pl011r.ibrd.write(UARTIBRD::IBRD.val(13));
            pl011r.fbrd.write(UARTFBRD::FBRD.val(1));
            // 8N1 FIFO enable
            pl011r.lcr_h.write(UARTLCR_H::WLEN::EightBit + UARTLCR_H::PEN::Disabled + UARTLCR_H::STP2::Stop1
                + UARTLCR_H::FEN::Enabled);
            // enable pl011
            pl011r.cr.write(UARTCR::UARTEN::Enabled + UARTCR::RXE::Enabled + UARTCR::TXE::Enabled);
        }

        Writer
    }
}

//为Write结构实现core::fmt::Write trait，该trait会自动实现write_fmt方法，支持格式化。
impl core::fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
      self.write_string(s);

      Ok(())
  }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    /// A global `Writer` instance that can be used for printing to the VGA text buffer.
    ///
    /// Used by the `print!` and `println!` macros.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart_console::_print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    WRITER.lock().write_fmt(args).unwrap();
}