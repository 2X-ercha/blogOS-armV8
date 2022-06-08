# 实验构建及运行

> 你将在每个实验对应分支上都看到这句话，确保作者实验代码在被下载后，能在正确的环境中运行。

运行环境请参考: [lab1 环境搭建](https://github.com/2X-ercha/blogOS-armV8/tree/lab1/docs/environment)

```bash
cargo build
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/blogos_armv8 -semihosting
```

--------

# 实验八 内存管理

第二部分上：非identity mapping映射（内核置于下半部分-原始地址，外设置于虚拟页-0xffffffff00000000开始的二级页表处）

先尝试不用二级页表，用块映射实现

--------

## 实验目的

实验指导书这么写的，全面理解分页式内存管理的基本方法以及访问页表，完成地址转换等的方法。这部分我也不太能搞懂，所以还是按它说的来吧。

--------

## 目录

这部分的四节我都用一个单页写完，所以目录没啥用。。。

[非identity mapping映射：块映射](./docs/)
