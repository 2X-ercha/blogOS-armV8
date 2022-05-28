ENTRY(_start)
SECTIONS
{
    . = 0x40080000;

    LD_TTBR0_BASE = .; /*页表*/
    . = . + 0x1000;

    LD_TTBR1_BASE = .;
    . = . + 0x1000;
    
    .text.boot : { *(.text.boot) }
    .text :
        {
        KEEP(*(.text.boot))
        *(.text.exceptions)
        . = ALIGN(4096); /* align for exceptions_vector_table*/
        *(.text.exceptions_vector_table)
        *(.text)
        }
    .data : { *(.data) }
    .rodata : { *(.rodata) }
    .bss : { *(.bss) }

    . = ALIGN(8);
    . = . + 0x4000;
    LD_STACK_PTR = .;

}
