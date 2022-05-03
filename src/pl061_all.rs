use tock_registers::{registers::{ReadWrite, WriteOnly, ReadOnly}, register_bitfields, register_structs};

pub const PL061REGS: *mut PL061Regs = (0x0903_0000) as *mut PL061Regs;

// https://developer.arm.com/documentation/ddi0190/b/programmer-s-model/summary-of-primecell-gpio-registers

register_bitfields![
    u32,

    // PrimeCell GPIO data direction register
    pub GPIODIR [
        /*
            GPIODATA寄存器是数据寄存器。
            在软件控制模式下，如果各个引脚已通过GPIODR寄存器配置为输出，则写入GPIODATA寄存器的值将传输到GPOUT引脚。

            为了写入GPIODATA，地址总线PADDR[9:2]产生的掩码中的相应位必须为高。否则，写入时位值保持不变。
            类似地，从该寄存器读取的值由从用于访问数据寄存器的地址PADDR[9:2]派生的掩码位为每个位确定。
            地址掩码中为1的位导致读取GPIODATA中的对应位，地址掩码中为0的位导致读取GPIODATA中的对应位，无论其值如何。

            如果各个管脚配置为输出，则从GPIODATA读取返回写入的最后一位值；如果这些管脚配置为输入，则从GPIODATA读取返回相应输入GPIN位上的值。
            所有位都通过重置清除。
        */
        Pin0 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin7 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ]
    ],

    // PrimeCell GPIO interrupt sense register
    pub GPIODIS [
        /*
            GPIOIS寄存器是中断检测寄存器。
            GPIOI中设置为高位的位配置相应的管脚以检测电平。
            清除一个位将配置引脚以检测边缘。所有位都通过重置清除。
        */
        Pin0 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin7 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ]
    ],

    // PrimeCell GPIO interrupt both edges register
    pub GPIOIBE [
        /*
            GPIOIBE寄存器是中断两边寄存器。
            当GPIOIS中的相应位设置为检测边缘时，GPIOIBE中设置为高的位将配置相应的引脚以检测上升和下降边缘，而不管GPIOIEV（中断事件寄存器）中的相应位如何。
            清除一个位将配置由GPIOIEV控制的引脚。所有位都通过重置清除。
        */
        Pin0 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin7 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ]
    ],

    // PrimeCell GPIO interrupt event register
    pub GPIOIEV [
        /*
            GPIOIEV寄存器是中断事件寄存器。
            GPIOIEV中设置为高的位根据GPIOIS中相应的位值配置相应的引脚以检测上升沿或高电平。
            根据GPIOIS中相应的位值，清除位可配置引脚以检测下降沿或低电平。
            所有位都通过重置清除。
         */
        Pin0 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin7 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ]
    ],

    // PrimeCell GPIO interrupt mask
    pub GPIOIE [
        /*
            GPIOIE寄存器是中断掩码寄存器。
            GPIOIE中设置为高位的位允许相应的管脚触发各自的中断和组合的GPIOINTR线。
            清除一个位将禁用该引脚上的中断触发。
            所有位都通过重置清除。
         */
        Pin0 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            clear = 0;
            set = 1;
        ],
        Pin7 OFFSET(0) NUMBITS(1) [
            clear = 0;
            set = 1;
        ]
    ],

    // PrimeCell GPIO raw interrupt status, when system_powerdown, this reg: 0x00 -> 0x08
    pub GPIORIS [
        /*
            GPIORIS寄存器是原始中断状态寄存器。
            GPIORI中读取的高位反映了检测到的中断触发条件的状态（原始，掩蔽之前），表明在GPIOIE最终允许触发之前，所有要求都已满足。
            读取为零的位表示相应的输入引脚未启动中断。
            该寄存器为只读，位通过复位清除。
         */
        Pin0 OFFSET(0) NUMBITS(1) [],
        Pin1 OFFSET(1) NUMBITS(1) [],
        Pin2 OFFSET(2) NUMBITS(1) [],
        Pin3 OFFSET(3) NUMBITS(1) [],
        Pin4 OFFSET(4) NUMBITS(1) [],
        Pin5 OFFSET(5) NUMBITS(1) [],
        Pin6 OFFSET(6) NUMBITS(1) [],
        Pin7 OFFSET(7) NUMBITS(1) []
    ],

    // PrimeCell GPIO masked interrupt status
    pub GPIOMIS [
        /*
            GPIOMIS寄存器是屏蔽中断状态寄存器。
            GPIOMIS中读取的高位反映了触发中断的输入线的状态。
            读取为低的位表示未生成中断，或中断被屏蔽。GPIOMIS是屏蔽后的中断状态。
            这个寄存器是只读的，所有的位都会被重置清除。
         */
        Pin0 OFFSET(0) NUMBITS(1) [],
        Pin1 OFFSET(1) NUMBITS(1) [],
        Pin2 OFFSET(2) NUMBITS(1) [],
        Pin3 OFFSET(3) NUMBITS(1) [],
        Pin4 OFFSET(4) NUMBITS(1) [],
        Pin5 OFFSET(5) NUMBITS(1) [],
        Pin6 OFFSET(6) NUMBITS(1) [],
        Pin7 OFFSET(7) NUMBITS(1) []
    ],

    // PrimeCell GPIO interrupt clear
    pub GPIOIC [
        /*
            GPIOIC寄存器是中断清除寄存器。
            在该寄存器中写入一个1到一个位将清除相应的中断边缘检测逻辑寄存器。写入0无效。
            该寄存器为只写寄存器，所有位均通过复位清除。
         */
        Pin0 OFFSET(0) NUMBITS(1) [
            set = 1;
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            set = 1;
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            set = 1;
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            set = 1;
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            set = 1;
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            set = 1;
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            set = 1;
        ],
        Pin7 OFFSET(0) NUMBITS(1) [
            set = 1;
        ]
    ],

    // PrimeCell GPIO mode control select
    pub GPIOAFSEL [
        /*
            GPIOAFSEL寄存器是模式控制选择寄存器。
            将1写入该寄存器中的任何位都会选择相应PrimeCell GPIO线的硬件控制。
            所有位都通过重置清除，因此默认情况下，没有PrimeCell GPIO线设置为硬件控制。
         */
        /*
            Bit written as 1, clears edge detection logic.
            Bit written as 0, has no effect.
         */
        Pin0 OFFSET(0) NUMBITS(1) [
            clear = 1;
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            clear = 1;
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            clear = 1;
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            clear = 1;
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            clear = 1;
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            clear = 1;
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            clear = 1;
        ],
        Pin7 OFFSET(0) NUMBITS(1) [
            clear = 1;
        ]
    ],
];

register_structs! {
    pub PL061Regs {
        (0x000 => pub data: ReadWrite<u32>),
        (0x004 => __reserved_0),
        (0x400 => pub dir: ReadWrite<u32, GPIODIR::Register>),
        (0x404 => pub is: ReadWrite<u32, GPIODIS::Register>),
        (0x408 => pub ibe: ReadWrite<u32, GPIOIBE:: Register>),
        (0x40c => pub iev: ReadWrite<u32, GPIOIEV::Register>),
        (0x410 => pub ie: ReadWrite<u32, GPIOIE::Register>),
        (0x414 => pub ris: ReadOnly<u32, GPIORIS::Register>),
        (0x418 => pub mis: ReadOnly<u32, GPIOMIS::Register>),
        (0x41c => pub ic: WriteOnly<u32, GPIOIC::Register>),
        (0x420 => pub afsel: ReadWrite<u32, GPIOAFSEL::Register>),
        (0x424 => @END),
    }
}
