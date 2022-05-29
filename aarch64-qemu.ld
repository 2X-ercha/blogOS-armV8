__KERN_VMA_BASE = 0xfffffff000000000;
__PHY_DRAM_START_ADDR = 0x40000000;
__PHY_START_LOAD_ADDR = 0x40010000;

ENTRY(__PHY_START_LOAD_ADDR)
SECTIONS
{

    . = __KERN_VMA_BASE + __PHY_START_LOAD_ADDR;
    .text.boot : AT(__PHY_START_LOAD_ADDR) { KEEP(*(.text.boot)) }  /*ADDR(.text.boot) - __KERN_VMA_BASE*/

    .text : /*AT(ADDR(.text) - __KERN_VMA_BASE)*/
    {
        KEEP(*(.text.boot))
        *(.text.exceptions)
        . = ALIGN(4096); /* align for exceptions_vector_table*/
        *(.text.exceptions_vector_table)
        *(.text*)
    }
    . = ALIGN(0x1000);

    LD_RODATA_BASE = .;
    .rodata :  /*AT(ADDR(.rodata) - __KERN_VMA_BASE)*/ { *(.rodata*) }
    . = ALIGN(0x1000);

    LD_DATA_BASE = .;
    .data : /*AT(ADDR(.data) - __KERN_VMA_BASE)*/ { *(.data*) }
    . = ALIGN(0x1000);

    LD_BSS_BASE = .;
    .bss : /*AT(ADDR(.bss) - __KERN_VMA_BASE)*/  { *(.bss*)

    . = ALIGN(4096); /* align to page size */
    . += (4096 * 100); /* 栈的大小 */
    stack_top = .;
    LD_STACK_PTR = .;
    }
    . = ALIGN(0x1000);

    .pt : /*AT(ADDR(.pt) - __KERN_VMA_BASE)*/ /* 页表 */
    {
    . = ALIGN(4096); /* align to page size */
    LD_TTBR0_BASE = . - __KERN_VMA_BASE; /*页表*/
    . = . + 0x1000;

    LD_TTBR1_BASE = . - __KERN_VMA_BASE;
    . = . + 0x1000;

    LD_TTBR0_L2TBL = . - __KERN_VMA_BASE; /*二级页表*/
    . = . + 0x1000;

    LD_TTBR1_L2TBL = . - __KERN_VMA_BASE;
    . = . + 0x1000;
    }

    . = . + 0x1000;
    LD_KERNEL_END = . - __KERN_VMA_BASE;
}
