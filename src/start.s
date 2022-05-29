.globl _start
.extern LD_STACK_PTR

.section ".text.boot"
_start:
    ldr     x30, =LD_STACK_PTR
    mov   sp, x30

    // Initialize exceptions
    ldr     x0, =exception_vector_table
    msr     vbar_el1, x0
    isb

_setup_mmu:
    // Initialize translation table control registers
    ldr     x0, =TCR_EL1_VALUE
    msr     tcr_el1, x0
    ldr     x0, =MAIR_EL1_VALUE
    msr     mair_el1, x0

_setup_pagetable:
    // 因为采用的36位地址空间，所以是level1 page table
    // 注：这里内存上大下小
    ldr     x1, =LD_TTBR0_BASE
    msr     ttbr0_el1, x1           // 页表基地址TTBR0，指向内存下半
    ldr     x2, =LD_TTBR1_BASE
    msr     ttbr1_el1, x2           // 页表基地址TTBR1，指向内存上半

    /* 虚拟地址空间的下半部分直接映射 */
    // 第一项 虚拟地址0 - 1g（无内容）
    ldr     x5, =0x0                // add flags
    str     x5, [x1], #8

    // 第二项 虚拟地址1 - 2g（存放内核）
    ldr     x3, =0x40080000
    lsr     x4, x3, #30             // 内核启动地址 / 1G
    lsl     x5, x4, #30             // 标记第30位为1
    ldr     x6, =IDENTITY_MAP_ATTR
    orr     x5, x5, x6              // add flags
    str     x5, [x1], #8

    /* 虚拟地址空间的上半部分处理 */
    // entries of level1 page table
    // 第一项 虚拟地址空间上半部分的首个1g映射到物理地址空间的0~1G，根据virt的定义为flash和外设，参见virt.c
    ldr     x3, =0x0
    lsr     x4, x3, #30
    lsl     x5, x4, #30
    ldr     x6, =PERIPHERALS_ATTR
    orr     x5, x5, x6             // add flags
    str     x5, [x2], #8

    // 第二项，映射到内存（首先简单地实现块映射，没有问题了再进一步将其映射到页表）
    ldr     x3, =0x40010000
    lsr     x4, x3, #30
    lsl     x5, x4, #30
    ldr     x6, =KERNEL_ATTR
    orr     x5, x5, x6             // add flags
    str     x5, [x2], #8

/*
    // 第二项，映射到页表(映射到页表)
    ldr     x3, =LD_TTBR1_L2TBL
    ldr     x4, =0xFFFFF000
    and     x5, x3, x4             // NSTable=0 APTable=0 XNTable=0 PXNTable=0.
    orr     x5, x5, 0x3            // Valid page table entry
    str     x5, [x2], #8

    // entries of level2 page table，二级页表共16M，详见aarch64-qemu.ld文件
    ldr     x3, =LD_TTBR1_L2TBL
    mov     x4, #8                 // 8个二级页表项
    ldr     x5, =KERNEL_ATTR       // 内核属性，可读写，可执行
    ldr     x7, =0x1
    add     x5, x5, x7, lsl #30    // 物理地址在1G开始的位置
    ldr     x6, =0x00200000        // 每次增加2M

_build_2nd_pgtbl:
    str     x5, [x3], #8           // 填入内容到页表项
    add     x5, x5, x6             // 下一项的地址增加2M
    subs    x4, x4, #1             // 项数减少1
    bne     _build_2nd_pgtbl
*/

_enable_mmu:
    // Enable the MMU.
    mrs     x0, sctlr_el1
    orr     x0, x0, #0x1
    msr     sctlr_el1, x0
    dsb     sy                     // 检查前面内存操作是否执行完整
    isb

_start_main:
    bl      not_main

.equ PSCI_SYSTEM_OFF, 0x84000002
.globl system_off
system_off:
        ldr     x0, =PSCI_SYSTEM_OFF
        hvc     #0

.equ TCR_EL1_VALUE, 0x1B55C351C
/*
IPS   | b001    << 32 | 36bits address space - 64GB
TG1   | b10     << 30 | 4KB granule size for TTBR1_EL1
SH1   | b11     << 28 | 页表所在memory: Inner shareable
ORGN1 | b01     << 26 | 页表所在memory: Normal, Outer Wr.Back Rd.alloc Wr.alloc Cacheble
IRGN1 | b01     << 24 | 页表所在memory: Normal, Inner Wr.Back Rd.alloc Wr.alloc Cacheble
EPD   | b0      << 23 | Perform translation table walk using TTBR1_EL1
A1    | b1      << 22 | TTBR1_EL1.ASID defined the ASID
T1SZ  | b011100 << 16 | Memory region 2^(64-28) -> 0xffffffexxxxxxxxx
TG0   | b00     << 14 | 4KB granule size
SH0   | b11     << 12 | 页表所在memory: Inner Sharebale
ORGN0 | b01     << 10 | 页表所在memory: Normal, Outer Wr.Back Rd.alloc Wr.alloc Cacheble
IRGN0 | b01     << 8  | 页表所在memory: Normal, Inner Wr.Back Rd.alloc Wr.alloc Cacheble
EPD0  | b0      << 7  | Perform translation table walk using TTBR0_EL1
0     | b0      << 6  | Zero field (reserve)
T0SZ  | b011100 << 0  | Memory region 2^(64-28)
*/

.equ MAIR_EL1_VALUE, 0xFF440C0400
/*
                  INDX         MAIR
DEVICE_nGnRnE    b000(0)     b00000000
DEVICE_nGnRE     b001(1)     b00000100
DEVICE_GRE       b010(2)     b00001100
NORMAL_NC        b011(3)     b01000100
NORMAL           b100(4)     b11111111
*/

.equ PERIPHERALS_ATTR, 0x60000000000601
/*
UXN   | b1      << 54 | Unprivileged eXecute Never
PXN   | b1      << 53 | Privileged eXecute Never
AF    | b1      << 10 | Access Flag
SH    | b10     << 8  | Outer shareable
AP    | b01     << 6  | R/W, EL0 access denied
NS    | b0      << 5  | Security bit (EL3 and Secure EL1 only)
INDX  | b000    << 2  | Attribute index in MAIR_ELn，参见MAIR_EL1_VALUE
ENTRY | b01     << 0  | Block entry
*/

.equ KERNEL_ATTR, 0x40000000000711
/*
UXN   | b1      << 54 | Unprivileged eXecute Never
PXN   | b0      << 53 | Privileged eXecute Never
AF    | b1      << 10 | Access Flag
SH    | b11     << 8  | Inner shareable
AP    | b00     << 6  | R/W, EL0 access denied
NS    | b0      << 5  | Security bit (EL3 and Secure EL1 only)
INDX  | b100    << 2  | Attribute index in MAIR_ELn，参见MAIR_EL1_VALUE
ENTRY | b01     << 0  | Block entry
*/

.equ IDENTITY_MAP_ATTR, 0x40000000000711
/*
UXN   | b1      << 54 | Unprivileged eXecute Never
PXN   | b0      << 53 | Privileged eXecute Never
AF    | b1      << 10 | Access Flag
SH    | b11     << 8  | Inner shareable
AP    | b00     << 6  | R/W, EL0 access denied
NS    | b0      << 5  | Security bit (EL3 and Secure EL1 only)
INDX  | b100    << 2  | Attribute index in MAIR_ELn，参见MAIR_EL1_VALUE
ENTRY | b01     << 0  | Block entry
*/
