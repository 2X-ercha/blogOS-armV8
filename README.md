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
...

--------

## 实验目的

...

1. ...

--------

## 目录

...

* ...
