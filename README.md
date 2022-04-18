# 实验构建及运行

> 你将在每个实验对应分支上都看到这句话，确保作者实验代码在被下载后，能在正确的环境中运行。

运行环境请参考: [lab1 环境搭建](https://github.com/2X-ercha/blogOS-armV8/tree/lab1/docs/environment)

```bash
cargo build
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/blogos_armv8
```

--------

# 实验一 环境配置

这是实验的开始。由于我们的目标是编写一个操作系统，所以首先我们需要创建一个独立于操作系统的可执行程序，又称 **独立式可执行程序（freestanding executable）** 或 **裸机程序（bare-metal executable）** 。然后我们将此程序编译成为内核。

我们编写的独立程序得十分纯净，这意味着所有依赖于操作系统的库我们都不能使用。比如 std 中的大部分内容（io, thread, file system, etc.）都需要操作系统的支持，所以这部分内容我们不能使用。

但是，不依赖与操作系统的 rust 的语言特性 我们还是可以继续使用的，比如：迭代器、模式匹配、字符串格式化、所有权系统等。这使得 rust 依旧可以作为一个功能强大的高级语言，帮助我们编写操作系统。

最小化内核只干了两件事：能开机在屏幕上输出点东西，以及能保持运行。

--------

## 实验目的

因此实验一的代码也只让你干了这两件事。终上所述，实验一的目的在于：

1. 装好rust，装对版本

2. 装好`qemu`虚拟机来跑我们想运行的操作系统

3. 装好交叉编译用的调试工具`aarch64-none-elf-gdb`

4. 了解最小化内核（或者说裸机）是什么，它包含什么，并且我们要能在`qemu`里边跑的动它。

5. 学会用`gdb`远程调试这个内核，起码要会查看地址等等

知道了这个实验要干什么，我们可以一条一条开始学习了！

--------

## 目录

* [环境安装（对应目的1、2、3）](./docs/environment/)

* [构建最小化内核（对应目的4）](./docs/bm_exe/)

* [gdb调试（对应目的5）](./docs/bm_exe/)
