## gdb调试

我们运行内核文件，是没办法像普通可执行文件那样，编译时增加`-g`指令然后`gdb`运行。因此我们要利用到`qemu`服务端功能，也就是开放端口让外部程序能够连接到`qemu`正在执行的程序中。

### qemu 启动参数

有人会好奇上一节`qemu`的启动为什么有这么长的启动参数。下面是`qemu`的启动参数表：

```
`-hda file'        `-hdb file' `-hdc file' `-hdd file'
    使用 file  作为硬盘0、1、2、3镜像。
`-fda file'  `-fdb file'
    使用 file  作为软盘镜像，可以使用 /dev/fd0 作为 file 来使用主机软盘。
`-cdrom file'
    使用 file  作为光盘镜像，可以使用 /dev/cdrom 作为 file 来使用主机 cd-rom。
`-boot [a|c|d]'
    从软盘(a)、光盘(c)、硬盘启动(d)，默认硬盘启动。
`-snapshot'
    写入临时文件而不写回磁盘镜像，可以使用 C-a s 来强制写回。
`-m megs'
    设置虚拟内存为 msg M字节，默认为 128M 字节。
`-smp n'
    设置为有 n 个 CPU 的 SMP 系统。以 PC 为目标机，最多支持 255 个 CPU。
`-nographic'
    禁止使用图形输出。
其他：
    可用的主机设备 dev 例如：
        vc
            虚拟终端。
        null
            空设备
        /dev/XXX
            使用主机的 tty。
        file: filename
            将输出写入到文件 filename 中。
        stdio
            标准输入/输出。
        pipe：pipename
            命令管道 pipename。
        等。
    使用 dev 设备的命令如：
        `-serial dev'
            重定向虚拟串口到主机设备 dev 中。
        `-parallel dev'
            重定向虚拟并口到主机设备 dev 中。
        `-monitor dev'
            重定向 monitor 到主机设备 dev 中。
    其他参数：
        `-s'
            等待 gdb 连接到端口 1234。
        `-p port'
            改变 gdb 连接端口到 port。
        `-S'
            在启动时不启动 CPU， 需要在 monitor 中输入 'c'，才能让qemu继续模拟工作。
        `-d'
            输出日志到 qemu.log 文件。
```

可以对照启动命令，来进行启动命令的解释，这里不做详解。

看到参数中`-S`和`-s`和`-p`，我们能知道如何启动`qemu`的服务端状态，开放相关的端口（默认`1234`来另`gdb`连接。

---------

### 启动调试

为了与`qemu`配合进行源代码级别的调试，需要先让`qemu`进入**等待gdb调试器的接入**并且**还不能让qemu中的CPU执行**，因此启动qemu的时候，我们需要使用参数`-S –s`这两个参数来做到这一点，这相当于在**本地的1234端口**开启远程调试功能。

在qemu内核启动命令后加上`-S -s`:

```bash
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/blogos_armv8 -S -s
```

内核不会马上运行，开始等待`gdb`的接管。由于我们是写给`arm`平台的操作系统，自然也需要`arm`平台的`gdb`调试工具。在项目根目录中，我们调用交叉编译工具链中的`aarch64-none-elf-gdb`工具来对程序进行调试。

保持`qemu`继续运行，新建一个终端后，在终端中输入：

（如果没有找到该命令，那么请返回第一节[环境安装](../environment/)交叉编译链安装处重新配置）

```bash
aarch64-none-elf-gdb target/aarch64-unknown-none-softfloat/debug/blogos_armv8
```

在`gdb`调试界面中输入：

```gdb
(gdb) target remote localhost:1234
```

连接到`qemu`中正在准备开始执行的内核后，可以像正常的`gdb`调试去调试我们的内核了。

--------

### 作者吐槽

不能不说，我看着实验指导书给好的现成的代码，不知道这些代码到底在干什么。我陷入了沉思，作为一个想学习嵌入式系统的学生而言，我似乎不能从这个实验中学到些什么。

然而这些知识，理应是一个想做嵌入式的人应该有的，但看着现成代码再看注解，大部分情况下还是一头雾水。老师说理解原理，但又理解不能，于是去翻阅资料。只有一步步实现，才能更好的知道我们为什么要这么做。

很多的代码细节，我也仍然没办法去一行行解释。面对想学的东西，更多的还是保持求知欲和不厌其烦。

路漫漫其修远兮，吾将上下而求索，说的莫若如是。