# 实验构建及运行

> 你将在每个实验对应分支上都看到这句话，确保作者实验代码在被下载后，能在正确的环境中运行。

运行环境请参考: [lab1 环境搭建](https://github.com/2X-ercha/blogOS-armV8/tree/lab1/docs/environment)

```bash
cargo build
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/blogos_armv8 -semihosting
```

--------

# 实验六 GPIO关机

我们不能一直到qemu的暴力退出来关机（就是不用系统的关机而暴力断电）。所幸，virt机器为我们提供了GPIO来实现关机功能。这节我们将编写GPIO相关的驱动来实现关机功能。

--------

## 实验目的

实验指导书中这节也没有写实验目的。我大致把目的划分如下：

1. 编写pl061（GPIO）通用输入输出模块的驱动

2. 实现关机功能

--------

## 目录

* [pl061（GPIO）模块驱动编写（对应目的1）](./docs/GPIO/)

* [实现关机中断及其处理回调函数（对应目的2）](./docs/powerdown/)