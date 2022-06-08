# 实验构建及运行

> 你将在每个实验对应分支上都看到这句话，确保作者实验代码在被下载后，能在正确的环境中运行。

运行环境请参考: [lab1 环境搭建](https://github.com/2X-ercha/blogOS-armV8/tree/lab1/docs/environment)

```bash
cargo build
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/blogos_armv8 -semihosting
```

--------

# 实验八 内存管理

第一部分补充：自行实验部分：`identity mapping`偏移映射与页面共享（外设映射到`2-3g`部分）

我们保留上一个实验部分内核的直接映射到 1G - 2G 部分，尝试将外设映射到 2G - 3G（原本是 0 - 1G）

--------

## 实验目的

实验指导书这么写的，全面理解分页式内存管理的基本方法以及访问页表，完成地址转换等的方法。这部分我也不太能搞懂，所以还是按它说的来吧。

--------

## 目录

这部分的四节我都用一个单页写完，所以目录没啥用。。。

[使用`identity mapping`直接映射（下）](./docs/)
