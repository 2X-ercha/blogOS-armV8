# 实验构建及运行

> 你将在每个实验对应分支上都看到这句话，确保作者实验代码在被下载后，能在正确的环境中运行。

运行环境请参考: [lab1 环境搭建](https://github.com/2X-ercha/blogOS-armV8/tree/lab1/docs/environment)

```bash
cargo build
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/blogos_armv8
```

--------

# 实验四 中断

中断、异常和陷阱指令是操作系统的基石，现代操作系统就是由中断驱动的。实验四的目的在于深刻理解中断的原理和机制，掌握CPU访问设备控制器的方法，掌握Arm体系结构的中断机制和规范，实现时钟中断服务和部分异常处理等。

中断是一种硬件机制。借助于中断，CPU可以不必再采用轮询这种低效的方式访问外部设备。将所有的外部设备与CPU直接相连是不现实的，外部设备的中断请求一般经由中断控制器，由中断控制器仲裁后再转发给CPU。Arm采用的中断控制器叫做`GIC`，即`general interrupt controller`。`gic`包括多个版本，如`GICv1`（已弃用），`GICv2`，`GICv3`，`GICv4`。简单起见，我们实验将选用`GICv2`版本。

为了配置好`gicv2`中断控制器，我们需要阅读其技术参考手册，以及上一个实验中讲到的设备树中关于`gic`的内存映射范围、中断基本说明，为`gic`编写内核驱动。

另外，为了检验我们中断的成功运行，我们在这节实验中也一并为**linux高精度计时器**`timer`编写应用。`timer`的精确计时依赖着系统的时钟中断，可以作为中断发生的检验方式。

--------

## 实验目的

实验指导书是这么写的：

> 本实验的目的在于深刻理解中断的原理和机制，掌握CPU访问设备控制器的方法，掌握Arm体系结构的中断机制和规范，实现时钟中断服务和部分异常处理等。

但实际上，异常处理是在实验五才进行处理的，这个实验指编写了异常发生时的回调函数规范。因此实验目的如下：

1. 理解中断原理和机制

2. 掌握CPU访问设备控制器（这里是`GIC`）的方法，即为设备编写驱动和初始化等基本方法

3. 掌握Arm体系结构的中断机制和规范，即定义异常向量表

4. 掌握异常回调函数的写法

5. 了解`timer`计时器原理，实现时间中断服务。

--------

## 目录

* [中断原理（目的1）](./docs/interrupts/)

* [GIC内核驱动编写及调用（目的2）](./docs/gic/)

* [ArmV8中断机制及异常回调（目的3、4）](./docs/exceptions/)

* [Timer计时器的原理和时钟中断服务实现](./docs/timer/)