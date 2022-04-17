/*
    QEMU的virt机器默认没有键盘作为输入设备，
    但当我们执行QEMU使用 -nographic 参数（disable graphical output and redirect serial I/Os to console）时
    QEMU会将串口重定向到控制台，因此我们可以使用UART作为输入设备。
    
    pl011@9000000 {
		clock-names = "uartclk\0apb_pclk";
		clocks = <0x8000 0x8000>;
		interrupts = <0x00 0x01 0x04>;
		reg = <0x00 0x9000000 0x00 0x1000>;
		compatible = "arm,pl011\0arm,primecell";
	};
 */

use tock_registers::{registers::{ReadOnly, ReadWrite, WriteOnly}, register_bitfields, register_structs};

pub const PL011REGS: *mut PL011Regs = (0x0900_0000) as *mut PL011Regs;

register_bitfields![
    u32,

    pub UARTDR [
        DATA OFFSET(0) NUMBITS(8) []
    ],
    /// Flag Register
    pub UARTFR [
        /// Transmit FIFO full. The meaning of this bit depends on the
        /// state of the FEN bit in the UARTLCR_ LCRH Register. If the
        /// FIFO is disabled, this bit is set when the transmit
        /// holding register is full. If the FIFO is enabled, the TXFF
        /// bit is set when the transmit FIFO is full.
        TXFF OFFSET(6) NUMBITS(1) [],

        /// Receive FIFO empty. The meaning of this bit depends on the
        /// state of the FEN bit in the UARTLCR_H Register. If the
        /// FIFO is disabled, this bit is set when the receive holding
        /// register is empty. If the FIFO is enabled, the RXFE bit is
        /// set when the receive FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) []
    ],

    /// Integer Baud rate divisor
    pub UARTIBRD [
        /// Integer Baud rate divisor
        IBRD OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud rate divisor
    pub UARTFBRD [
        /// Fractional Baud rate divisor
        FBRD OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control register
    pub UARTLCR_H [
        /// Parity enable. If this bit is set to 1, parity checking and generation
        /// is enabled, else parity is disabled and no parity bit added to the data frame.
        PEN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        /// Two stop bits select. If this bit is set to 1, two stop bits are transmitted
        /// at the end of the frame.
        STP2 OFFSET(3) NUMBITS(1) [
            Stop1 = 0,
            Stop2 = 1
        ],
        /// Enable FIFOs.
        FEN OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Word length. These bits indicate the number of data bits
        /// transmitted or received in a frame.
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ]
    ],

    /// Control Register
    pub UARTCR [
        /// Receive enable. If this bit is set to 1, the receive
        /// section of the UART is enabled. Data reception occurs for
        /// UART signals. When the UART is disabled in the middle of
        /// reception, it completes the current character before
        /// stopping.
        RXE    OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Transmit enable. If this bit is set to 1, the transmit
        /// section of the UART is enabled. Data transmission occurs
        /// for UART signals. When the UART is disabled in the middle
        /// of transmission, it completes the current character before
        /// stopping.
        TXE    OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// UART enable
        UARTEN OFFSET(0) NUMBITS(1) [
            /// If the UART is disabled in the middle of transmission
            /// or reception, it completes the current character
            /// before stopping.
            Disabled = 0,
            Enabled = 1
        ]
    ],

    pub UARTIMSC [
        RXIM OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],
    /// Interupt Clear Register
    pub UARTICR [
        /// Meta field for all pending interrupts
        ALL OFFSET(0) NUMBITS(11) [
            Clear = 0x7ff
        ]
    ]
];

register_structs! {
    pub PL011Regs {
        (0x00 => pub dr: ReadWrite<u32, UARTDR::Register>),                   // 0x00
        (0x04 => __reserved_0),               // 0x04
        (0x18 => pub fr: ReadOnly<u32, UARTFR::Register>),      // 0x18
        (0x1c => __reserved_1),               // 0x1c
        (0x24 => pub ibrd: WriteOnly<u32, UARTIBRD::Register>), // 0x24
        (0x28 => pub fbrd: WriteOnly<u32, UARTFBRD::Register>), // 0x28
        (0x2C => pub lcr_h: WriteOnly<u32, UARTLCR_H::Register>), // 0x2C
        (0x30 => pub cr: WriteOnly<u32, UARTCR::Register>),     // 0x30
        (0x34 => __reserved_2),               // 0x34
        (0x38 => pub imsc: ReadWrite<u32, UARTIMSC::Register>), // 0x38
        (0x44 => pub icr: WriteOnly<u32, UARTICR::Register>),   // 0x44
        (0x48 => @END),
    }
}