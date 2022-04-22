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

_start_main:
    bl      not_main

.equ PSCI_SYSTEM_OFF, 0x84000002
.globl system_off
system_off:
        ldr     x0, =PSCI_SYSTEM_OFF
        hvc     #0