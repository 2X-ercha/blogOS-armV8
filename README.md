# 实验构建及运行

> 你将在每个实验对应分支上都看到这句话，确保作者实验代码在被下载后，能在正确的环境中运行。

运行环境请参考: [lab1 环境搭建](https://github.com/2X-ercha/blogOS-armV8/tree/lab1/docs/environment)

```bash
cargo build
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/blogos_armv8
```

--------

# 实验五 输入

QEMU的virt机器默认没有键盘作为输入设备，但当我们执行QEMU使用 -nographic 参数（disable graphical output and redirect serial I/Os to console）时QEMU会将串口重定向到控制台，因此我们可以使用UART作为输入设备。

同时这次实验也将顺便完成上一节没有完成的异常回调处理，我们将作对时钟中断和硬件中断的不同处理。

--------

## 实验目的

实验指导书中这节就没有写实验目的了。我大致把目的划分如下：

1. 完成实验四未完成的时钟中断处理回调

2. 编写pl011（UART）异步串行接口的驱动编写

3. 完成串口输入中断

--------

## 目录

* [时钟中断回调函数实现（目的1）](./docs/timer/)

* [pl011（UART）异步串行接口驱动编写（目的2）](./docs/pl011/)

* [串口输入中断处理回调（目的3）](./docs/uart_interrupt/)