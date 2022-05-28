(还在乱写笔记)

LD_TTBR0_BASE    0x40080000   0x7375722f
LD_TTBR1_BASE    0x40081edd   0x0

STR R0，[R1]，＃8             ；将R0中的字数据写入以R1为地址的存储器中，并将新地址R1＋8写入R1。

[ARM Cortex-A Series Programmer's Guide for ARMv8-A](https://developer.arm.com/documentation/den0024/a/The-Memory-Management-Unit/Context-switching) 中提到：TTBR0指向整个虚拟空间下半部分通常用于应用程序的空间，TTBR1指向虚拟空间的上半部分通常用于内核的空间。其中TTBR0除了在EL1中存在外，也在EL2 and EL3中存在，但TTBR1只在EL1中存在。

`TTBR0_ELn`和`TTBR1_ELn`是页表基地址寄存器，地址转换的过程如下所示。

![](https://os2022exps-doc.readthedocs.io/zh_CN/latest/_images/v2p-translate.svg)

在只涉及一级查找的简单地址转换中。它假设我们使用的是具有`42位`虚拟地址的`64KB`粒度。MMU将虚拟地址转换如下：

* 如果`VA[63:42] == 1`，第一页表的基址则使用TTBR1。当`VA[63:42] == 0`时，第一页表的基址则使用TTBR0。

* 页表包含8192个64位页表条目，并使用`VA[41:29]`编制索引。MMU从表中读取相关的2级页面表条目。

MMU检查页表条目的有效性，以及是否允许请求的内存访问。假设它有效，则允许内存访问。

在上图中，页表条目指的是512MB页（它是块描述符）。

位`[47:29]`取自此页表条目，并形成物理地址的位`[47:29]`。

因为我们有一个 512MB 的页面，所以VA的位`[28:0]`被用来形成`PA[28:0]`。请参见颗粒大小对转换表的影响

最后返回完整的PA[47:0]，以及来自页面表条目的附加信息。

实际上，这样一个简单的转换过程严重限制了地址空间的划分。第一级表条目也可以指向第二级页表，而不是仅使用此第一级转换表。
