use tock_registers::{registers::{ReadWrite, WriteOnly}, register_bitfields, register_structs};

pub const PL061REGS: *mut PL061Regs = (0x8000_0000u32 + 0x0903_0000) as *mut PL061Regs;

// https://developer.arm.com/documentation/ddi0190/b/programmer-s-model/summary-of-primecell-gpio-registers

register_bitfields![
    u32,

    // PrimeCell GPIO interrupt mask
    pub GPIOIE [
        IO3 OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    // PrimeCell GPIO raw interrupt status, when system_powerdown, this reg: 0x00 -> 0x08
    // pub GPIORIS [],

    // PrimeCell GPIO interrupt clear
    // pub GPIOIC []
];

register_structs! {
    pub PL061Regs {
        (0x000 => __reserved_0),
        (0x410 => pub ie: ReadWrite<u32, GPIOIE::Register>),
        (0x414 => __reserved_1),
        (0x41c => pub ic: WriteOnly<u32>),
        (0x420 => @END),
    }
}
