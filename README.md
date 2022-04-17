# blogOS armV8

## 前言

这个仓库是用于记录操作系统课程的实验。这里不得不吐槽一下课程，把[blogos](https://github.com/phil-opp/blog_os)移植估计是湖大老师自己的操作。虽然说湖大开始用自己的东西是应该夸赞的，但是作为被试点的20级智能专业的学生是很不舒服的。起码需要的是个完备的实验指导书（我做这个实验的时候指导书还是缺了好多，以及细节部分的错误，还有版本不对应等等。

另外说的是这实验也几乎照抄某个版本的[leos](https://github.com/lowenware/leos-kernel)。许多代码看起来像是多个文件整合在了一起，，当然后面就有所不同了。

如果这个仓库帮助到了你，可以留下一个`star`。感谢支持！

## 实验指导书

https://os2022exps-doc.readthedocs.io/zh_CN/latest/index.html

## 分支说明

* master

    作者当前代码（这个分支更多的用于作者维护git以便于版本管理和试验）

* main（当前还没开始设置）

    等这个实验完全完成后，这个分支将被创建，记录该os的每个发布版本

* LAB-x （当前还没开始设置）

    这里会列出我做这个实验每个阶段的代码。比如根据实验指导书完成到实验5，那么对应的分支应该在LAB-5处，代码应该完成到的阶段则在该分支处展示

## 实验环境

rust版本及相关工具

```bash
cargo install cargo-binutils rustfilt
rustup install nightly-2021-11-20
rustup default nightly-2021-11-20
```

安装`arm v8`支持
```bash
rustup target add aarch64-unknown-none-softfloat
```

安装`qemu`模拟器

```bash
sudo apt-get install qemu qemu-system-aarch64
```

## 实验构建及运行

```bash
cargo build
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -kernel target/aarch64-unknown-none-softfloat/debug/rui_armv8_os
```

如果你要进行`gdb`调试，可以在运行指令最后加上`-S -s`，默认端口为`1234`

然后新建终端后启动调试客户端，远程连接`qemu`进行调试
```bash
aarch64-none-elf-gdb target/aarch64-unknown-none-softfloat/debug/rui_armv8_os
(gdb) target remote localhost:1234
```