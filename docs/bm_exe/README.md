## 构建最小化内核

### 能跑起来的裸机程序尝试

让我们试着创建一个`main`程序：

新建项目文档

```bash
cargo new blogos_armv8 --bin --edition 2021
cd blogos_armv8
```

然后新建`src/main.rs`

(这时候还别想着输出"hello world"，你的`print`还没实现呢)

```rust
#![no_std]

fn main() {}
```

然后`cargo build`进行构建，会产生两个报错：

```
error: language item required, but not found: `eh_personality`

error: `#[panic_handler]` function required, but not found
```

* `eh_personality`报错：

    eh_personality 语义项(language item)用于标记函数：该函数在 堆栈展开(stack unwinding) 时被调用。当程序发生 panic 时，rust 会调用 堆栈展开 析构堆栈中的所有生存变量，达到释放内存的目的。但是这是一个复杂的过程，而且依赖于一些其他的库文件。所以我们只是简单的将其禁用：

    编辑`Cargo.toml`，在后边加入如下代码:

    ```toml
    # dev时禁用panic时栈展开
    [profile.dev]
    panic = "abort"

    # release时禁用panic时栈展开
    [profile.release]
    panic = "abort"
    ```

* `panic`报错：

    当程序出现异常时（咱这个主函数还没有return)，程序将会进入`panic`，此时需要调用相应函数。标准库有对应函数，但是由于我们使用了 `no_std` 属性，所以接下来我们需要自己实现一个函数。新建`src/panic.rs`

    ```cargo
    use core::panic::PanicInfo;

    #[panic_handler]
    fn on_panic(_info: &PanicInfo) -> ! {
        loop {}
    }
    ```

    然后在`main.rs`中引入`panic`

    ```rust
    mod panic;
    ```

    由于程序 panic 后就应该结束，所以用 -> ! 表示该函数不会返回。由于目前的 OS 功能还很弱小，我们有希望系统保持开机状态，所以只能无限循环。

解决完如上几个报错后，再次`cargo build`，出现新的报错：

```
error: requires `start` lang_item
```

对于大多数语言，他们都使用了 运行时系统(runtime system) ，这导致 main 并不是他们执行的第一个函数。以 rust 语言为例：一个典型的 rust 程序会先链接标准库，然后运行 C runtime library 中的 crt0(C runtime zero) 设置 C 程序运行所需要的环境(比如：创建堆栈，设置寄存器参数等)。然后 C runtime 会调用 rust runtime 的 入口点(entry point) 。rust runtime 结束之后才会调用 main 。由于我们的程序无法访问 rust runtime 和 crt0 ，所以需要重写覆盖 crt0 入口点：

* `start`入口报错
    新建`src/start.s`，告诉函数我们程序的进入入口在哪：

    ```assembly
    .globl _start
    .extern LD_STACK_PTR
    .section ".text.boot"

    _start:
            ldr     x30, =LD_STACK_PTR
            mov     sp, x30
            bl      not_main

    .equ PSCI_SYSTEM_OFF, 0x84000002
    .globl system_off
    system_off:
            ldr     x0, =PSCI_SYSTEM_OFF
            hvc     #0
    ```

    可以看到我们想告诉程序：我们这玩意的入口是`not_main`，程序要从`not_main`函数开始。然后修改`main.rs`，将主函数删除，替换成：

    ```rust
    #![no_main]

    #[no_mangle] // 不修改函数名
    pub extern "C" fn not_main() {}
    ```

    这里 pub extern "C" fn not_main 就是我们需要的 `start` 。 #[no_mangle] 属性用于防止改名称被混淆。
    
    由于 `start` 只能由操作系统或引导加载程序直接调用，不会被其他函数调用，所以不能够返回。如果需要离开该函数，应该使用 `exit` 系统调用。
    
    由于 start 函数无法返回或退出，自然也就不会调用 main 。所以将 main 函数删除，并且增加属性标签 `#![no_main]` 。

再次编译，出现了我们本节的最后一个错误：

```
error: linking with `cc` failed: exit status: 1
```

在链接 C runtime 时，会需要一些 C 标准库(libc)的内容。由于 #![no_std] 禁用了标准库，所以我们需要禁用常规的 C 启动例程：

* `linker`报错

历经千辛万苦，我们终于成功构建了一个裸机程序！