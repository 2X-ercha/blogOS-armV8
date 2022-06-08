# 实验构建及运行

> 你将在每个实验对应分支上都看到这句话，确保作者实验代码在被下载后，能在正确的环境中运行。

运行环境请参考: [lab1 环境搭建](https://github.com/2X-ercha/blogOS-armV8/tree/lab1/docs/environment)

```bash
cargo build
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/blogos_armv8 -semihosting
```

--------

# 实验八 内存管理

第一部分：使用`identity mapping`直接映射

虚拟地址的转换很容易出错也很难调试，所以我们从最简单方式开始，即`identity mapping`，所谓`identity mapping`就是将虚拟地址映射到相同的物理地址。

比如内核部分位于物理内存 1G - 2G 的地方，外设位于物理内存 0 - 1G 的地方，我们先将其映射到虚拟内存中相同位置（也就是没有偏移的状态）

--------

## 实验目的

实验指导书这么写的，全面理解分页式内存管理的基本方法以及访问页表，完成地址转换等的方法。这部分我也不太能搞懂，所以还是按它说的来吧。

--------

## 目录

这部分的四节我都用一个单页写完，所以目录没啥用。。。

[使用`identity mapping`直接映射（上）](./docs/)
