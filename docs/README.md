### 使用identity mapping映射（外设映射偏移尝试）

自行实验部分：`identity mapping`偏移映射与页面共享（外设映射到`2-3g`部分）

我们保留上一个实验部分内核的直接映射到 1G - 2G 部分，尝试将外设映射到 2G - 3G（原本是 0 - 1G）

如果理解了上一节我们如何进行内存映射的，不难发现，虚拟地址的映射部分本质是汇编代码`str     x5, [x1], #8`。我们只需要令`x1`原本的第一项变成第三项即可。修改`src/start.s`中`_setup_pagetable`段如下：

```assembly
_setup_pagetable:
    // 因为采用的36位地址空间，所以是level1 page table
    ldr     x1, =LD_TTBR0_BASE
    msr     ttbr0_el1, x1 //页表基地址TTBR0
    ldr     x2, =LD_TTBR1_BASE
    msr     ttbr1_el1, x2 //页表基地址TTBR1

    // entries of level1 page table

    // 虚拟地址空间的下半部分做identity mapping
    // 第一项 虚拟地址0 - 1g (无内容)
    ldr     x5, =0x0               // add flags
    str     x5, [x1], #8

    // 第二项 虚拟地址1g - 2g（存放内核）
    ldr     x3, =_start
    lsr     x4, x3, #30             // 内核启动地址 / 1G
    lsl     x5, x4, #30             // 标记第30位为1
    ldr     x6, =IDENTITY_MAP_ATTR
    orr     x5, x5, x6              // add flags
    str     x5, [x1], #8

    // 第三项 虚拟地址2 - 3g（根据virt的定义为flash和外设，参见virt.c）
    ldr     x3, =0x0
    lsr     x4, x3, #30
    lsl     x5, x4, #30
    ldr     x6, =PERIPHERALS_ATTR
    orr     x5, x5, x6             // add flags
    str     x5, [x1], #8
  ```

  第一项我们令其为`0`（指向物理内存 0 - 1G，但不带任何权限属性），第二项不变，第三项即原第一项的属性内容。再次编译内核并运行

  ```bash
  cargo build
  qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/blogos_armv8 -semihosting
  ```

  屏幕上能够正常输出`[0] Hello from Rust!`并正常打点即说明成功实现了直接无偏移的映射。
